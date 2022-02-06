use std::{
    fmt::Display,
    io::{stdin, stdout, Stdout, Write},
};

use color_eyre::{
    eyre::{ensure, Context},
    owo_colors::{
        colors::{css::LightGreen, Black, Green, Red, Yellow},
        OwoColorize,
    },
    Result,
};
use termion::{
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
    solution: String,
    guesses: Vec<String>,
    game_type: GameType,
    terminal: MouseTerminal<RawTerminal<Stdout>>,
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
            solution,
            guesses: Vec::with_capacity(6),
            game_type,
            terminal: MouseTerminal::from(
                stdout()
                    .into_raw_mode()
                    .wrap_err("could not get handle on a TTY")?,
            ),
        })
    }

    pub fn start(mut self) -> Result<Option<GameShare>> {
        self.draw_window()?;

        let mut word = String::new();

        let stdin = stdin();

        for c in stdin.keys() {
            let evt = c?;
            match evt {
                Key::Esc => return Ok(None),
                Key::Char(c) if c.is_ascii() && word.len() < 5 => {
                    let c = c.to_ascii_lowercase();
                    write!(self.terminal, "{}", c.to_ascii_uppercase())?;
                    word.push(c);
                }
                Key::Char('\n') if word.len() == 5 => {
                    if wordle::valid(&word) {
                        self.guesses.push(word.clone());
                        self.draw_valid()?;

                        if word == self.solution {
                            let score =
                                std::char::from_digit(self.guesses.len() as u32, 10).unwrap();
                            return Ok(Some(self.share(score)?));
                        } else if self.guesses.len() >= 6 {
                            self.draw_final_solution()?;

                            return Ok(Some(self.share('X')?));
                        }

                        word.clear();
                    } else {
                        self.draw_invalid(&word)?;
                    }
                }
                Key::Backspace => {
                    word.pop();
                    write!(
                        self.terminal,
                        "{back} {back}",
                        back = termion::cursor::Left(1)
                    )?;
                }
                _ => {}
            }
            self.terminal.flush().unwrap();
        }

        Ok(None)
    }

    fn share(mut self, score: char) -> Result<GameShare> {
        write!(self.terminal, "{}", termion::cursor::Down(1))?;

        Ok(GameShare {
            game_type: self.game_type,
            matches: self
                .guesses
                .into_iter()
                .map(|input| wordle::diff(&*input, &*self.solution))
                .collect(),
            score,
        })
    }

    fn draw_invalid(&mut self, invalid: &str) -> Result<()> {
        self.draw_valid()?;
        write!(
            self.terminal,
            "{}",
            invalid.to_ascii_uppercase().bg::<Red>()
        )?;
        Ok(())
    }

    fn draw_valid(&mut self) -> Result<()> {
        self.draw_window()?;
        for i in 0..self.guesses.len() {
            self.draw_guess(i)?;
        }
        Ok(())
    }

    fn draw_final_solution(&mut self) -> Result<()> {
        write!(self.terminal, "{}", termion::cursor::Down(1))?;

        write!(
            self.terminal,
            "{}",
            self.solution
                .to_ascii_uppercase()
                .fg::<Black>()
                .bg::<LightGreen>()
        )?;

        write!(self.terminal, "{}", termion::cursor::Goto(1, 11))?;
        Ok(())
    }

    fn draw_guess(&mut self, i: usize) -> Result<()> {
        let input = &*self.guesses[i];
        let matches = wordle::diff(input, &*self.solution);
        for (m, c) in matches.0.into_iter().zip(input.chars()) {
            let c = c.to_ascii_uppercase();
            match m {
                Match::Green => write!(self.terminal, "{}", c.fg::<Black>().bg::<Green>())?,
                Match::Amber => write!(self.terminal, "{}", c.fg::<Black>().bg::<Yellow>())?,
                Match::Black => write!(self.terminal, "{}", c)?,
            };
        }
        write!(self.terminal, "{}", termion::cursor::Goto(1, 4 + i as u16))?;
        Ok(())
    }

    fn draw_window(&mut self) -> Result<()> {
        let (_width, height) = terminal_size()?;

        write!(
            self.terminal,
            "{clear_all}{bottom_left}Press ESC to exit.",
            clear_all = termion::clear::All,
            bottom_left = termion::cursor::Goto(1, height),
        )?;
        write!(
            self.terminal,
            "{top_left}Wordle {game_type}{down}",
            top_left = termion::cursor::Goto(1, 1),
            game_type = self.game_type,
            down = termion::cursor::Goto(1, 3),
        )?;
        self.terminal.flush()?;

        Ok(())
    }
}

pub struct GameShare {
    game_type: GameType,
    matches: Vec<wordle::Matches>,
    score: char,
}

impl Display for GameShare {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Wordle {game_type} {score}/6",
            game_type = self.game_type,
            score = self.score
        )?;
        for m in &self.matches {
            write!(f, "\n{m}")?;
        }
        Ok(())
    }
}
