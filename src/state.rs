use std::fmt;

use crate::{iter::StateIter, words::WordSet, Match, Matches};

pub struct State {
    word_set: WordSet<'static>,
    solution: String,
    guesses: Vec<String>,
}

#[derive(Debug)]
pub enum GuessError {
    MissingExactValues(usize),
    NotInWordList,
}

pub enum GameOver {
    Win,
    Lose,
}

impl GameOver {
    pub fn is_win(&self) -> bool {
        match self {
            GameOver::Win => true,
            GameOver::Lose => false,
        }
    }
    pub fn is_lose(&self) -> bool {
        match self {
            GameOver::Win => false,
            GameOver::Lose => true,
        }
    }
}

impl State {
    /// Create a new game state from the solution
    pub fn new(solution: String, word_set: WordSet<'static>) -> Self {
        Self {
            word_set,
            solution,
            guesses: Vec::with_capacity(6),
        }
    }

    /// Reveal the solution for the current game state
    pub fn solution(&self) -> &str {
        &*self.solution
    }

    /// Get the number of maximum possible guesses
    pub fn max_guesses(&self) -> usize {
        self.guesses.capacity()
    }

    /// Returns an iterator over the previous guesses
    pub fn guesses(&self) -> StateIter<'_> {
        StateIter {
            solution: &*self.solution(),
            guesses: self.guesses.iter(),
        }
    }

    /// Make a guess.
    ///
    /// # Errors
    /// If the guess is an invalid word, or if it doesn't match the
    /// requirements of hard mode, this function will return an error
    pub fn guess(&mut self, word: &str, hard: bool) -> Result<Matches, GuessError> {
        if self.word_set.valid(word) {
            if hard {
                if let Some((last_word, matches)) = self.guesses().last() {
                    for i in 0..5 {
                        if matches[i] == Match::Exact
                            && last_word.as_bytes()[i] != word.as_bytes()[i]
                        {
                            return Err(GuessError::MissingExactValues(i));
                        }
                    }
                }
            }

            self.guesses.push(word.to_owned());
            Ok(crate::diff(word, &*self.solution))
        } else {
            Err(GuessError::NotInWordList)
        }
    }

    /// Determine if the game is over.
    pub fn game_over(&self) -> Option<GameOver> {
        let last = &self.guesses[self.guesses.len() - 1];
        if last == &self.solution {
            Some(GameOver::Win)
        } else if self.guesses.len() >= 6 {
            Some(GameOver::Lose)
        } else {
            None
        }
    }

    /// Displays the score card for this game state to the given [`fmt::Write`].
    pub fn display_score_card(&self, mut w: impl fmt::Write, hard: bool) -> fmt::Result {
        let Self {
            solution, guesses, ..
        } = self;
        let n = guesses.len();
        let score = if n < 6 || &guesses[5] == solution {
            std::char::from_digit(n as u32, 10).ok_or(fmt::Error)?
        } else {
            'X'
        };

        let hard_mode = if hard { "*" } else { "" };

        write!(w, "{score}/6{hard_mode}",)?;
        for g in self.guesses() {
            write!(w, "\n{}", g.1)?;
        }
        Ok(())
    }
}
