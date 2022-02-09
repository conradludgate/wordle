use std::fmt;

use crate::{iter::StateIter, valid, Match, Matches};

pub struct State {
    solution: String,
    guesses: Vec<String>,
}

#[derive(Debug)]
pub enum GuessError {
    MissingExactValues(usize),
    NotInWordList,
}

impl State {
    pub fn new(solution: String) -> Self {
        Self {
            solution,
            guesses: Vec::with_capacity(6),
        }
    }

    pub fn solution(&self) -> &str {
        &*self.solution
    }

    pub fn guesses(&self) -> StateIter<'_> {
        StateIter {
            solution: &*self.solution(),
            guesses: self.guesses.iter(),
        }
    }

    pub fn guess(&mut self, word: &str, hard: bool) -> Result<Matches, GuessError> {
        if valid(word) {
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

    pub fn game_over(&self) -> Option<bool> {
        let last = &self.guesses[self.guesses.len() - 1];
        if last == &self.solution {
            Some(true)
        } else if self.guesses.len() >= 6 {
            Some(false)
        } else {
            None
        }
    }

    pub fn display_score_card(&self, mut w: impl fmt::Write) -> fmt::Result {
        let Self { solution, guesses } = self;
        let n = guesses.len();
        let score = if n < 6 || &guesses[5] == solution {
            std::char::from_digit(n as u32, 10).ok_or(fmt::Error)?
        } else {
            'X'
        };

        write!(w, "{score}/6",)?;
        for g in self.guesses() {
            write!(w, "\n{}", g.1)?;
        }
        Ok(())
    }
}
