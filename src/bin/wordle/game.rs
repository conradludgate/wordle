use std::{fmt, fmt::Display};

use crossterm::{
    cursor,
    terminal::{self, Clear, ClearType},
};
use eyre::{ensure, Context, Result};

use super::{keyboard::Keyboard, state::State};

pub struct Game {
    state: State,
    game_type: GameType,
    keyboard: Keyboard,
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (_width, height) = terminal::size().map_err(|_| std::fmt::Error)?;

        write!(
            f,
            "{clear_all}{bottom_left}Press ESC to exit.{top_left}Wordle {game_type}{down}{keyboard}{state}",
            clear_all = Clear(ClearType::All),
            bottom_left = cursor::MoveTo(0, height-1),
            top_left = cursor::MoveTo(0, 0),
            game_type = self.game_type,
            down = cursor::MoveTo(0, 2),
            keyboard = self.keyboard,
            state = self.state
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

    pub fn push(&mut self, word: &str) {
        let matches = self.state.push(word);
        self.keyboard.push(word, matches);
    }

    pub fn finish(&self) -> Option<bool> {
        self.state.finish()
    }

    pub fn display_final_solution(&self) {
        self.state.display_final_solution()
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

impl Display for GameType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GameType::Daily(day) => write!(f, "{}", day),
            GameType::Custom => write!(f, "custom"),
        }
    }
}
