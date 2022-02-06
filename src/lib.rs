use std::fmt::Display;

pub mod words;

pub fn get_solution(day: usize) -> &'static str {
    words::FINAL[day % words::FINAL.len()]
}

pub fn get_day(date: time::Date) -> usize {
    (date.to_julian_day() - words::FIRST_DAY.to_julian_day()) as usize
}

pub fn valid(word: &str) -> bool {
    words::ACCEPT.contains(&word) || words::FINAL.contains(&word)
}

#[derive(Clone, Copy, Debug, PartialEq)]
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

pub fn diff(input: &str, solution: &str) -> Matches {
    assert!(
        input.is_ascii(),
        "input guess should only be 5 ascii letters"
    );
    assert_eq!(input.len(), 5, "input guess should only be 5 ascii letters");
    debug_assert!(solution.is_ascii());
    debug_assert_eq!(solution.len(), 5);

    let input = input.as_bytes();
    let mut solution = solution.as_bytes().to_owned();

    let mut diff = [Match::Black; 5];

    // find exact matches first
    for (i, &b) in input.iter().enumerate() {
        if solution[i] == b {
            solution[i] = 0; // letters only match once
            diff[i] = Match::Green;
        }
    }

    // now, find amber matches
    for (i, &b) in input.iter().enumerate() {
        if diff[i] != Match::Black {
            continue;
        }
        if let Some(j) = solution.iter().position(|&x| x == b) {
            solution[j] = 0; // letters only match once
            diff[i] = Match::Amber;
        }
    }

    Matches(diff)
}
