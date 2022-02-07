use std::fmt;

use crate::{Matches, valid};

pub struct State {
    solution: String,
    guesses: Vec<String>,
}

pub struct Guess<'a>(pub &'a str, pub Matches);

pub struct StateIter<'a> {
    solution: &'a str,
    guesses: std::slice::Iter<'a, String>,
}

impl<'a> Iterator for StateIter<'a> {
    type Item = Guess<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.guesses
            .next()
            .map(|input| Guess(&*input, crate::diff(&*input, self.solution)))
    }
}

pub struct GuessError;

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

    pub fn guess(&mut self, word: &str) -> Result<Matches, GuessError> {
        if valid(word) {
            self.guesses.push(word.to_owned());
            Ok(crate::diff(word, &*self.solution))
        } else {
            Err(GuessError)
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
        for Guess(_, m) in self.guesses() {
            write!(w, "\n{}", m)?;
        }
        Ok(())
    }
}
