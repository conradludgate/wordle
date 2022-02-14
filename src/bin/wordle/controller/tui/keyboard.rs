use std::fmt::Display;

use cl_wordle::{Match, Matches};
use crossterm::cursor;
use rand::prelude::SliceRandom;

use super::letters::LetterMatch;

pub struct Keyboard {
    arangement: Vec<u8>,
    letters: [Option<Match>; 26],
}

impl Keyboard {
    pub fn push(&mut self, word: &str, matches: Matches) {
        for (b, m) in word.bytes().zip(matches.0) {
            let b = (b - b'a') as usize;
            let m2 = &mut self.letters[b];
            *m2 = Some(match (m, *m2) {
                (_, Some(Match::Exact)) | (Match::Exact, _) => Match::Exact,
                (_, Some(Match::Close)) | (Match::Close, _) => Match::Close,
                (_, Some(Match::Wrong)) | (Match::Wrong, _) => Match::Wrong,
            });
        }
        self.sort();
    }

    /// sorts letters that are `Some(Wrong)` to the end of the list
    fn sort(&mut self) {
        self.arangement.sort_by_key(|&b| self.letters[b as usize]);
    }

    pub fn shuffle(&mut self) {
        // finds the first wrong position, otherwise just returns the length (end)
        let i = self
            .arangement
            .iter()
            .position(|&b| self.letters[b as usize] == Some(Match::Wrong))
            .unwrap_or(self.arangement.len());
        // shuffles the current valid range of letters
        self.arangement[..i].shuffle(&mut rand::thread_rng())
    }
}

impl Default for Keyboard {
    fn default() -> Self {
        Self {
            arangement: (0..26).collect(),
            letters: [None; 26],
        }
    }
}

impl Display for Keyboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{save}{start}",
            save = cursor::SavePosition,
            start = cursor::MoveTo(14, 2)
        )?;

        for (i, &b) in self.arangement.iter().enumerate() {
            if i == 7 || i == 14 || i == 21 {
                write!(f, "{}{}", cursor::MoveDown(1), cursor::MoveLeft(7))?;
            }
            if i == 21 {
                write!(f, " ")?;
            }
            let c = (b + b'A') as char;
            match self.letters[b as usize] {
                Some(m) => write!(f, "{}", LetterMatch(c, m))?,
                None => write!(f, "{}", c)?,
            }
        }

        write!(f, "{restore}", restore = cursor::RestorePosition)?;

        Ok(())
    }
}
