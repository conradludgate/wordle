use std::fmt;

use crate::state::State;
use eyre::{ensure, Result};

pub struct Game {
    state: State,
    game_type: GameType,
}

impl Game {
    #[cfg(feature = "time")]
    pub fn new() -> Result<Self> {
        use eyre::WrapErr;
        let now =
            time::OffsetDateTime::now_local().wrap_err("could not determine local timezone")?;
        Self::from_date(now.date())
    }

    pub fn custom(solution: String) -> Result<Self> {
        Self::new_raw(solution, GameType::Custom)
    }

    #[cfg(feature = "time")]
    pub fn from_date(date: time::Date) -> Result<Self> {
        let day = crate::get_day(date);
        Self::from_day(day)
    }

    pub fn from_day(day: usize) -> Result<Self> {
        let solution = crate::get_solution(day).to_owned();
        Self::new_raw(solution, GameType::Daily(day))
    }

    fn new_raw(solution: String, game_type: GameType) -> Result<Self> {
        ensure!(
            crate::words::FINAL.contains(&&*solution),
            "{} is not a valid solution",
            solution
        );
        Ok(Self {
            state: State::new(solution),
            game_type,
        })
    }

    pub fn game_type(&self) -> GameType {
        self.game_type
    }

    pub fn state(&self) -> &State {
        &self.state
    }

    pub fn state_mut(&mut self) -> &mut State {
        &mut self.state
    }

    pub fn share(self) -> GameShare {
        GameShare(self)
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

pub struct GameShare(Game);

impl fmt::Display for GameShare {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Termo #{game_type} ", game_type = self.0.game_type())?;
        self.0.state().display_score_card(f)?;
        Ok(())
    }
}
