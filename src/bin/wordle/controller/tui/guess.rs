use std::fmt;

use cl_wordle::{state::State, Matches};
use crossterm::cursor;

use super::letters::LetterMatch;

pub struct Guesses<'a>(&'a State);

impl<'a> From<&'a State> for Guesses<'a> {
    fn from(state: &'a State) -> Self {
        Guesses(state)
    }
}

impl<'a> fmt::Display for Guesses<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (w, Matches(m)) in self.0.guesses() {
            for (b, m) in w.bytes().zip(m) {
                write!(f, "{}", LetterMatch(b as char, m))?;
            }
            write!(f, "{}{}", cursor::MoveDown(1), cursor::MoveLeft(5))?;
        }
        Ok(())
    }
}
