use std::fmt::Display;

use cl_wordle::{Match, Matches};
use crossterm::cursor;

use crate::letters::LetterMatch;

pub struct Keyboard {
    arangement: String,
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
    }
}

impl Default for Keyboard {
    fn default() -> Self {
        Self {
            arangement: ('A'..='Z').collect(),
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

        for (i, b) in self.arangement.bytes().enumerate() {
            if i == 7 || i == 14 || i == 21 {
                write!(f, "{}{}", cursor::MoveDown(1), cursor::MoveLeft(7))?;
            }
            if i == 21 {
                write!(f, " ")?;
            }
            match self.letters[i] {
                Some(m) => write!(f, "{}", LetterMatch(b as char, m))?,
                None => write!(f, "{}", b as char)?,
            }
        }

        write!(f, "{restore}", restore = cursor::RestorePosition)?;

        Ok(())
    }
}
