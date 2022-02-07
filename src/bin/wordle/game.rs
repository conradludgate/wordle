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
use wordle::{Match, Matches};

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

pub struct LetterMatch(char, Match);
impl Display for LetterMatch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = self.0.to_ascii_uppercase();
        match self.1 {
            Match::Exact => write!(f, "{}", c.fg::<Black>().bg::<Green>()),
            Match::Close => write!(f, "{}", c.fg::<Black>().bg::<Yellow>()),
            Match::Wrong => write!(f, "{}", c.fg::<Gray>()),
        }
    }
}

pub struct GameController {
    base: Game,
    terminal: MouseTerminal<RawTerminal<Stdout>>,
}

pub struct Game {
    state: State,
    game_type: GameType,
    keyboard: Keyboard,
}

impl Display for Game {
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

impl Game {
    pub fn push(&mut self, word: &str) {
        self.state.guesses.push(word.to_owned());
        let matches = wordle::diff(word, &self.state.solution);
        self.keyboard.push(word, matches);
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
                write!(f, "{}", LetterMatch(c, m))?;
            }
            write!(f, "{}{}", cursor::Down(1), cursor::Left(5))?;
        }
        Ok(())
    }
}

pub struct Keyboard {
    arangement: String,
    letters: [Option<Match>; 26],
}

impl Keyboard {
    pub fn push(&mut self, word: &str, matches: Matches) {
        for (b, m) in word.bytes().zip(matches.0) {
            let b = (b - b'a') as usize;
            let m2 = &mut self.letters[b];
            *m2 = Some(match (m, *m2) {
                (_, Some(Match::Exact)) | (Match::Exact, _) => Match::Exact,
                (_, Some(Match::Close)) | (Match::Close, _) => Match::Close,
                (_, Some(Match::Wrong)) | (Match::Wrong, _) => Match::Wrong,
            });
        }
    }
}

impl Default for Keyboard {
    fn default() -> Self {
        Self {
            arangement: ('A'..='Z').collect(),
            letters: [None; 26],
        }
    }
}

impl Display for Keyboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{save}{start}",
            save = cursor::Save,
            start = cursor::Goto(15, 3)
        )?;

        for (i, b) in self.arangement.bytes().enumerate() {
            if i == 7 || i == 14 || i == 21 {
                write!(f, "{}{}", cursor::Down(1), cursor::Left(7))?;
            }
            if i == 21 {
                write!(f, " ")?;
            }
            match self.letters[i] {
                Some(m) => write!(f, "{}", LetterMatch(b as char, m))?,
                None => write!(f, "{}", b as char)?,
            }
        }

        write!(f, "{restore}", restore = cursor::Restore)?;

        Ok(())
    }
}

impl GameController {
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
            base: Game {
                state: State {
                    solution,
                    guesses: Vec::with_capacity(6),
                },
                game_type,
                keyboard: Keyboard::default(),
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

pub struct GameShare(Game);

impl Display for GameShare {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Game {
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
