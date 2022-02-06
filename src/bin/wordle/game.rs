use std::{
    fmt::Display,
    io::{stdin, stdout, Stdout, Write},
};

use color_eyre::{
    eyre::{ensure, Context},
    owo_colors::{
        colors::{xterm::Gray, Black, Green, Red, Yellow},
        OwoColorize,
    },
    Result,
};
use termion::{
    cursor,
    event::Key,
    input::{MouseTerminal, TermRead},
    raw::{IntoRawMode, RawTerminal},
    terminal_size,
};

use cl_wordle as wordle;
use wordle::Match;

#[derive(Clone, Copy, Debug)]
enum GameType {
    Daily(usize),
    Custom,
}

impl Display for GameType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GameType::Daily(day) => write!(f, "{}", day),
            GameType::Custom => write!(f, "custom"),
        }
    }
}

pub struct Game {
    base: GameBase,
    terminal: MouseTerminal<RawTerminal<Stdout>>,
}

pub struct GameBase {
    state: State,
    game_type: GameType,
    keyboard: Keyboard,
}

impl Display for GameBase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (_width, height) = terminal_size().map_err(|_| std::fmt::Error)?;

        write!(
            f,
            "{clear_all}{bottom_left}Press ESC to exit.{top_left}Wordle {game_type}{down}{keyboard}{state}",
            clear_all = termion::clear::All,
            bottom_left = cursor::Goto(1, height),
            top_left = cursor::Goto(1, 1),
            game_type = self.game_type,
            down = cursor::Goto(1, 3),
            keyboard = self.keyboard,
            state = self.state
        )?;

        Ok(())
    }
}

impl GameBase {
    pub fn push(&mut self, word: &str) {
        self.state.guesses.push(word.to_owned());
        let matches = wordle::diff(word, &self.state.solution);
        for (b, m) in word.bytes().zip(matches.0) {
            let b = (b - b'a') as usize;
            let m2 = &mut self.keyboard.0[b];
            *m2 = Some(match (m, *m2) {
                (_, Some(Match::Green)) | (Match::Green, _) => Match::Green,
                (_, Some(Match::Amber)) | (Match::Amber, _) => Match::Amber,
                (_, Some(Match::Black)) | (Match::Black, _) => Match::Black,
            });
        }
    }
}

pub struct State {
    solution: String,
    guesses: Vec<String>,
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for input in &self.guesses {
            let matches = wordle::diff(&*input, &*self.solution);
            for (m, c) in matches.0.into_iter().zip(input.chars()) {
                let c = c.to_ascii_uppercase();
                match m {
                    Match::Green => write!(f, "{}", c.fg::<Black>().bg::<Green>())?,
                    Match::Amber => write!(f, "{}", c.fg::<Black>().bg::<Yellow>())?,
                    Match::Black => write!(f, "{}", c)?,
                };
            }
            write!(f, "{}{}", cursor::Down(1), cursor::Left(5))?;
        }
        Ok(())
    }
}

pub struct Keyboard([Option<Match>; 26]);

impl Display for Keyboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{save}{start}",
            save = cursor::Save,
            start = cursor::Goto(15, 3)
        )?;

        const LAYOUT: &[&str] = &["ABCDEFG", "HIJKLMO", "OPQRSTU", " VWXYZ"];

        for (i, layout) in LAYOUT.iter().enumerate() {
            for (mut j, b) in layout.bytes().enumerate() {
                if i == 3 && j > 0 {
                    j -= 1;
                }
                let c = b as char;
                match self.0[i * 7 + j] {
                    Some(Match::Green) => write!(f, "{}", c.fg::<Black>().bg::<Green>())?,
                    Some(Match::Amber) => write!(f, "{}", c.fg::<Black>().bg::<Yellow>())?,
                    Some(Match::Black) => write!(f, "{}", c.fg::<Gray>())?,
                    None => write!(f, "{}", c)?,
                }
            }
            write!(
                f,
                "{down}{left}",
                down = cursor::Down(1),
                left = cursor::Left(7)
            )?;
        }

        write!(f, "{restore}", restore = cursor::Restore,)?;

        Ok(())
    }
}

impl Game {
    pub fn new() -> Result<Self> {
        let now =
            time::OffsetDateTime::now_local().wrap_err("could not determine local timezone")?;
        Self::from_date(now.date())
    }

    pub fn custom(solution: String) -> Result<Self> {
        Self::new_raw(solution, GameType::Custom)
    }

    pub fn from_date(date: time::Date) -> Result<Self> {
        let day = wordle::get_day(date);
        Self::from_day(day)
    }

    pub fn from_day(day: usize) -> Result<Self> {
        let solution = wordle::get_solution(day).to_owned();
        Self::new_raw(solution, GameType::Daily(day))
    }

    fn new_raw(solution: String, game_type: GameType) -> Result<Self> {
        ensure!(
            wordle::words::FINAL.contains(&&*solution),
            "{} is not a valid solution",
            solution
        );

        Ok(Self {
            base: GameBase {
                state: State {
                    solution,
                    guesses: Vec::with_capacity(6),
                },
                game_type,
                keyboard: Keyboard([None; 26]),
            },
            terminal: MouseTerminal::from(
                stdout()
                    .into_raw_mode()
                    .wrap_err("could not get handle on a TTY")?,
            ),
        })
    }

    pub fn start(mut self) -> Result<Option<GameShare>> {
        self.draw_window()?;
        self.terminal.flush()?;

        let mut word = String::with_capacity(5);

        let stdin = stdin();

        for c in stdin.keys() {
            let evt = c?;
            match evt {
                Key::Esc => return Ok(None),
                Key::Char(c) if c.is_ascii_alphabetic() && word.len() < 5 => {
                    let c = c.to_ascii_lowercase();
                    write!(self.terminal, "{}", c.to_ascii_uppercase())?;
                    word.push(c);
                }
                Key::Char('\n') if word.len() == 5 => {
                    if wordle::valid(&word) {
                        self.base.push(&word);
                        self.draw_window()?;

                        if word == self.base.state.solution {
                            return Ok(Some(self.share()?));
                        } else if self.base.state.guesses.len() >= 6 {
                            self.draw_final_solution()?;
                            return Ok(Some(self.share()?));
                        }

                        word.clear();
                    } else {
                        self.draw_invalid(&word)?;
                    }
                }
                Key::Backspace => {
                    word.pop();
                    write!(self.terminal, "{back} {back}", back = cursor::Left(1))?;
                }
                _ => {}
            }
            self.terminal.flush()?;
        }

        Ok(None)
    }

    fn share(mut self) -> Result<GameShare> {
        write!(self.terminal, "{}", cursor::Goto(1, 12))?;
        Ok(GameShare(self.base))
    }

    fn draw_invalid(&mut self, invalid: &str) -> Result<()> {
        self.draw_window()?;
        write!(
            self.terminal,
            "{}",
            invalid.to_ascii_uppercase().bg::<Red>()
        )?;
        Ok(())
    }

    fn draw_final_solution(&mut self) -> Result<()> {
        write!(self.terminal, "{}", cursor::Down(1))?;

        write!(
            self.terminal,
            "{}",
            self.base
                .state
                .solution
                .to_ascii_uppercase()
                .fg::<Black>()
                .bg::<Green>()
        )?;

        write!(self.terminal, "{}", cursor::Goto(1, 11))?;
        Ok(())
    }

    fn draw_window(&mut self) -> Result<()> {
        write!(self.terminal, "{}", self.base)?;
        Ok(())
    }
}

pub struct GameShare(GameBase);

impl Display for GameShare {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let GameBase {
            keyboard: _,
            state,
            game_type,
        } = &self.0;
        let n = state.guesses.len();
        let score = if n < 6 || state.guesses[5] == state.solution {
            std::char::from_digit(n as u32, 10).ok_or(std::fmt::Error)?
        } else {
            'X'
        };

        write!(f, "Wordle {game_type} {score}/6",)?;
        for input in &state.guesses {
            write!(f, "\n{}", wordle::diff(&*input, &*state.solution))?;
        }
        Ok(())
    }
}
