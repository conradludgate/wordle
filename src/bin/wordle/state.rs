use std::{fmt, fmt::Display, io};

use cl_wordle::{Match, Matches};
use termion::cursor;

use crate::letters::{LetterMatch, WordMatch};

pub struct State {
    solution: String,
    guesses: Vec<String>,
}

impl State {
    pub fn new(solution: String) -> Self {
        Self {
            solution,
            guesses: Vec::with_capacity(6),
        }
    }

    pub fn push(&mut self, word: &str) -> Matches {
        self.guesses.push(word.to_owned());
        cl_wordle::diff(word, &*self.solution)
    }

    pub fn finish(&self) -> Option<bool> {
        let last = &self.guesses[self.guesses.len() - 1];
        if last == &self.solution {
            Some(true)
        } else if self.guesses.len() >= 6 {
            Some(false)
        } else {
            None
        }
    }

    pub fn write_final_solution(&self, mut w: impl io::Write) -> io::Result<()> {
        write!(w, "{}", cursor::Down(1))?;
        write!(w, "{}", WordMatch(&self.solution, Match::Exact))?;
        write!(w, "{}", cursor::Goto(1, 11))
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
        for input in guesses {
            write!(w, "\n{}", cl_wordle::diff(&*input, &*solution))?;
        }
        Ok(())
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for input in &self.guesses {
            let matches = cl_wordle::diff(&*input, &*self.solution);
            for (m, c) in matches.0.into_iter().zip(input.chars()) {
                write!(f, "{}", LetterMatch(c, m))?;
            }
            write!(f, "{}{}", cursor::Down(1), cursor::Left(5))?;
        }
        Ok(())
    }
}
