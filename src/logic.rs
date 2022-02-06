use std::fmt::Display;

use color_eyre::{eyre::ensure, Result};

use crate::words;

pub fn get_solution(day: usize) -> &'static str {
    words::FINAL[day % words::FINAL.len()]
}

pub fn get_day(date: time::Date) -> usize {
    (date.to_julian_day() - words::FIRST_DAY.to_julian_day()) as usize
}

#[derive(Clone, Copy, Debug)]
pub enum Match {
    Green,
    Amber,
    Black,
}

pub struct Matches(pub [Match; 5]);

impl Display for Match {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Match::Green => write!(f, "🟩"),
            Match::Amber => write!(f, "🟨"),
            Match::Black => write!(f, "⬛"),
        }
    }
}

impl Display for Matches {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for m in self.0 {
            write!(f, "{}", m)?;
        }
        write!(f, "")
    }
}

pub fn diff(input: &str, solution: &str) -> Result<Matches> {
    ensure!(
        input.is_ascii(),
        "input guess should only be 5 ascii letters"
    );
    ensure!(
        input.len() == 5,
        "input guess should only be 5 ascii letters"
    );

    debug_assert!(solution.is_ascii());
    debug_assert_eq!(solution.len(), 5);

    let input = input.as_bytes();
    let solution = solution.as_bytes();

    let mut diff = [Match::Black; 5];

    for (i, &b) in input.iter().enumerate() {
        if solution[i] == b {
            diff[i] = Match::Green;
        } else if solution.contains(&b) {
            diff[i] = Match::Amber;
        }
    }

    Ok(Matches(diff))
}
