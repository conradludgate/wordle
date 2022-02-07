use std::{fmt};

use cl_wordle::Match;
use owo_colors::{
    colors::{Black, Green, Yellow, xterm::Gray},
    OwoColorize,
};

pub struct LetterMatch(pub char, pub Match);
impl fmt::Display for LetterMatch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        render(self.1, self.0.to_ascii_uppercase(), f)
    }
}

pub struct WordMatch<'a>(pub &'a str, pub Match);
impl<'a> fmt::Display for WordMatch<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        render(self.1, self.0.to_ascii_uppercase(), f)
    }
}

fn render(m: Match, d: impl fmt::Display, mut f: impl fmt::Write) -> fmt::Result {
    match m {
        Match::Exact => write!(f, "{}", d.fg::<Black>().bg::<Green>()),
        Match::Close => write!(f, "{}", d.fg::<Black>().bg::<Yellow>()),
        Match::Wrong => write!(f, "{}", d.fg::<Gray>()),
    }
}
