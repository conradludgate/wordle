use std::{fmt, io};

use cl_wordle::{state::{State, GuessError}, Match};
use crossterm::{
    cursor,
    terminal::{self, Clear, ClearType},
};
use eyre::{ensure, Context, Result};

use crate::{guess::Guesses, letters::WordMatch};

use super::keyboard::Keyboard;

pub struct Game {
    state: State,
    game_type: GameType,
    keyboard: Keyboard,
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (_width, height) = terminal::size().map_err(|_| fmt::Error)?;

        write!(
            f,
            "{clear_all}{bottom_left}Press ESC to exit.{top_left}Wordle {game_type}{down}{keyboard}{state}",
            clear_all = Clear(ClearType::All),
            bottom_left = cursor::MoveTo(0, height-1),
            top_left = cursor::MoveTo(0, 0),
            game_type = self.game_type,
            down = cursor::MoveTo(0, 2),
            keyboard = self.keyboard,
            state = Guesses::from(&self.state),
        )?;

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
        let day = cl_wordle::get_day(date);
        Self::from_day(day)
    }

    pub fn from_day(day: usize) -> Result<Self> {
        let solution = cl_wordle::get_solution(day).to_owned();
        Self::new_raw(solution, GameType::Daily(day))
    }

    fn new_raw(solution: String, game_type: GameType) -> Result<Self> {
        ensure!(
            cl_wordle::words::FINAL.contains(&&*solution),
            "{} is not a valid solution",
            solution
        );
        Ok(Self {
            state: State::new(solution),
            game_type,
            keyboard: Keyboard::default(),
        })
    }

    pub fn guess(&mut self, word: &str) -> Result<(), GuessError> {
        let matches = self.state.guess(word)?;
        self.keyboard.push(word, matches);
        Ok(())
    }

    pub fn over(&self) -> Option<bool> {
        self.state.game_over()
    }

    pub fn write_final_solution(&self, mut w: impl io::Write) -> io::Result<()> {
        write!(w, "{}", cursor::MoveDown(1))?;
        write!(w, "{}", WordMatch(self.state.solution(), Match::Exact))?;
        write!(w, "{}", cursor::MoveTo(0, 10))
    }

    pub fn display_share_card(&self, mut f: impl fmt::Write) -> fmt::Result {
        let Self {
            keyboard: _,
            state,
            game_type,
        } = &self;

        write!(f, "Wordle {game_type} ")?;
        state.display_score_card(f)?;
        Ok(())
    }
}

#[derive(Clone, Copy, Debug)]
pub enum GameType {
    Daily(usize),
    Custom,
}

impl fmt::Display for GameType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GameType::Daily(day) => write!(f, "{}", day),
            GameType::Custom => write!(f, "custom"),
        }
    }
}
