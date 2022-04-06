use std::{fmt, ops::Deref};

use crate::{
    state::{GuessError, State},
    Matches, words::WordSet,
};
use eyre::{ensure, Result};

/// A Wrapper over [`State`] that manages
/// creation and playing of games
pub struct Game {
    state: State,
    hard_mode: bool,
    game_type: GameType,
}

impl Deref for Game {
    type Target = State;

    fn deref(&self) -> &Self::Target {
        &self.state
    }
}

impl Game {
    /// Create a new game based on the current date (official style)
    #[cfg(feature = "time")]
    pub fn new(word_set: WordSet<'static>) -> Result<Self> {
        use eyre::WrapErr;
        let now =
            time::OffsetDateTime::now_local().wrap_err("could not determine local timezone")?;
        Ok(Self::from_date(now.date(), word_set))
    }

    /// Create a new game based on the given word
    pub fn custom(solution: String, word_set: WordSet<'static>) -> Result<Self> {
        ensure!(
            word_set.solutions.contains(&&*solution),
            "{} is not a valid solution",
            solution
        );
        Ok(Self::new_raw(solution, GameType::Custom, word_set))
    }

    /// Create a new game based on the given date
    #[cfg(feature = "time")]
    pub fn from_date(date: time::Date, word_set: WordSet<'static>) -> Self {
        let day = word_set.get_day(date);
        Self::from_day(day, word_set)
    }

    /// Create a new game based on the given day number
    pub fn from_day(day: usize, word_set: WordSet<'static>) -> Self {
        let solution = word_set.get_solution(day).to_owned();
        Self::new_raw(solution, GameType::Daily(day), word_set)
    }

    fn new_raw(solution: String, game_type: GameType, word_set: WordSet<'static>) -> Self {
        Self {
            state: State::new(solution, word_set),
            hard_mode: false,
            game_type,
        }
    }

    /// Sets the play style of this game to 'hard mode'.
    /// This means that any exact matches found must be
    /// re-used in later guesses
    pub fn hard_mode(&mut self) {
        self.hard_mode = true;
    }

    /// Get the number of maximum possible guesses
    pub fn max_guess(&self) -> usize {
        self.state.max_guesses()
    }

    /// Get the current attempt for this game
    pub fn current_guess(&self) -> usize {
        self.max_guesses().min(self.state.guesses().count() + 1)
    }

    /// Indicate whether hard mode is active or not
    pub fn hard_mode_indicator(&self) -> &str {
        if self.hard_mode {
            return "*"
        } else {
            return ""
        }
    }

    /// Get the [`GameType`] for this game
    pub fn game_type(&self) -> GameType {
        self.game_type
    }

    /// Make a guess.
    ///
    /// # Errors
    /// If the guess is an invalid word, or if it doesn't match the
    /// requirements of hard mode, this function will return an error
    pub fn guess(&mut self, word: &str) -> Result<Matches, GuessError> {
        self.state.guess(word, self.hard_mode)
    }

    /// Display the share card for this game
    ///
    /// ```
    /// use cl_wordle::game::Game;
    /// let mut game = Game::from_day(0, cl_wordle::words::ORIGINAL);
    /// game.guess("crane").unwrap();
    /// game.guess("carts").unwrap();
    /// game.guess("chair").unwrap();
    /// game.guess("cigar").unwrap();
    ///
    /// let share = game.share();
    /// let score_card = format!("{}", share);
    /// assert_eq!(score_card, r"Wordle 0 4/6
    /// 🟩🟨🟨⬛⬛
    /// 🟩🟨🟨⬛⬛
    /// 🟩⬛🟨🟨🟩
    /// 🟩🟩🟩🟩🟩");
    /// ```
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

/// Display the share card for this game
///
/// ```
/// use cl_wordle::game::Game;
/// let mut game = Game::from_day(0, cl_wordle::words::ORIGINAL);
/// game.guess("crane").unwrap();
/// game.guess("carts").unwrap();
/// game.guess("chair").unwrap();
/// game.guess("cigar").unwrap();
///
/// let share = game.share();
/// let score_card = format!("{}", share);
/// assert_eq!(score_card, r"Wordle 0 4/6
/// 🟩🟨🟨⬛⬛
/// 🟩🟨🟨⬛⬛
/// 🟩⬛🟨🟨🟩
/// 🟩🟩🟩🟩🟩");
/// ```
pub struct GameShare(Game);

impl fmt::Display for GameShare {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Wordle {game_type} ", game_type = self.0.game_type())?;
        self.0.display_score_card(f, self.0.hard_mode)?;
        Ok(())
    }
}
