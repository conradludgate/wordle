use std::fmt::Display;

pub mod words;
pub mod state;

/// Gets the solution word for the given day
pub fn get_solution(day: usize) -> &'static str {
    words::FINAL[day % words::FINAL.len()]
}

/// Gets the current day number from the given date
#[cfg(feature = "time")]
pub fn get_day(date: time::Date) -> usize {
    (date.to_julian_day() - words::FIRST_DAY.to_julian_day()) as usize
}

/// Determines if the given word is valid, according to the default word lists
pub fn valid(word: &str) -> bool {
    words::ACCEPT.contains(&word) || words::FINAL.contains(&word)
}

#[derive(Clone, Copy, Debug, PartialEq)]
/// Represents a match for a given letter against the solution
pub enum Match {
    /// Letter is in the correct position
    Exact,
    /// Letter is in the solution but not in the correct positon
    Close,
    /// Letter is not in the solution
    Wrong,
}

/// Represents the outcome for a single guess
pub struct Matches(pub [Match; 5]);

impl Display for Match {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Match::Exact => write!(f, "🟩"),
            Match::Close => write!(f, "🟨"),
            Match::Wrong => write!(f, "⬛"),
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

    let mut diff = [Match::Wrong; 5];

    // find exact matches first
    for (i, &b) in input.iter().enumerate() {
        if solution[i] == b {
            solution[i] = 0; // letters only match once
            diff[i] = Match::Exact;
        }
    }

    // now, find amber matches
    for (i, &b) in input.iter().enumerate() {
        if diff[i] != Match::Wrong {
            continue;
        }
        if let Some(j) = solution.iter().position(|&x| x == b) {
            solution[j] = 0; // letters only match once
            diff[i] = Match::Close;
        }
    }

    Matches(diff)
}

#[cfg(test)]
mod tests {
    use super::{
        diff,
        Match::{self, *},
    };
    use test_case::test_case;

    #[test_case("class", "crest", [Exact, Wrong, Wrong, Exact, Wrong]; "double letter, one exact, one wrong")]
    #[test_case("stars", "crest", [Close, Close, Wrong, Close, Wrong]; "double letter, one close, one wrong")]
    #[test_case("kills", "skill", [Close, Close, Close, Exact, Close]; "double letter, one exact, one close")]
    fn test_diff(input: &str, solution: &str, matches: [Match; 5]) {
        assert_eq!(diff(input, solution).0, matches);
    }
}
