use std::{fmt::Display, ops::Deref};

pub mod words;
pub mod state;
pub mod game;
pub mod iter;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
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
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Matches(pub [Match; 5]);

impl Matches {
    pub fn win(self) -> bool {
        self.0 == [Match::Exact; 5]
    }
}

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

impl Deref for Matches {
    type Target = [Match; 5];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn diff(input: &str, solution: &str) -> Matches {
    debug_assert!(
        input.is_ascii(),
        "input guess should only be 5 ascii letters"
    );
    debug_assert_eq!(input.len(), 5, "input guess should only be 5 ascii letters");
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
