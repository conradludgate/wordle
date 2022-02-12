use std::fmt;

use crate::{
    state::{GuessError, State},
    Matches,
};
use eyre::{ensure, Result};

pub struct Game {
    state: State,
    hard_mode: bool,
    game_type: GameType,
}

impl Game {
    #[cfg(feature = "time")]
    pub fn new() -> Result<Self> {
        use eyre::WrapErr;
        let now =
            time::OffsetDateTime::now_local().wrap_err("could not determine local timezone")?;
        Ok(Self::from_date(now.date()))
    }

    pub fn custom(solution: String) -> Result<Self> {
        ensure!(
            crate::words::FINAL.contains(&&*solution),
            "{} is not a valid solution",
            solution
        );
        Ok(Self::new_raw(solution, GameType::Custom))
    }

    #[cfg(feature = "time")]
    pub fn from_date(date: time::Date) -> Self {
        let day = crate::get_day(date);
        Self::from_day(day)
    }

    pub fn from_day(day: usize) -> Self {
        let solution = crate::get_solution(day).to_owned();
        Self::new_raw(solution, GameType::Daily(day))
    }

    fn new_raw(solution: String, game_type: GameType) -> Self {
        Self {
            state: State::new(solution),
            hard_mode: false,
            game_type,
        }
    }

    pub fn hard_mode(&mut self) {
        self.hard_mode = true;
    }

    pub fn game_type(&self) -> GameType {
        self.game_type
    }

    pub fn state(&self) -> &State {
        &self.state
    }

    pub fn guess(&mut self, word: &str) -> Result<Matches, GuessError> {
        self.state.guess(word, self.hard_mode)
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
        write!(f, "Wordle {game_type} ", game_type = self.0.game_type())?;
        self.0.state().display_score_card(f, self.0.hard_mode)?;
        Ok(())
    }
}
