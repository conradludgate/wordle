mod nytimes;
mod original;

#[derive(Clone, Copy)]
pub struct WordSet<'a> {
    #[cfg(feature = "time")]
    pub date_offset: time::Date,
    pub solutions: &'a [&'a str],
    pub acceptable: &'a [&'a str],
}

impl Default for WordSet<'static> {
    fn default() -> Self {
        NYTIMES
    }
}

/// Set of the original words
pub const ORIGINAL: WordSet<'static> = WordSet {
    #[cfg(feature = "time")]
    date_offset: time::macros::date!(2021 - 06 - 19),
    solutions: original::FINAL,
    acceptable: original::ACCEPT,
};

pub const NYTIMES: WordSet<'static> = WordSet {
    #[cfg(feature = "time")]
    date_offset: time::macros::date!(2021 - 06 - 19),
    solutions: nytimes::FINAL,
    acceptable: nytimes::ACCEPT,
};

impl<'a> WordSet<'a> {
    /// Gets the solution word for the given day
    pub fn get_solution(self, day: usize) -> &'a str {
        self.solutions[day % self.solutions.len()]
    }

    /// Gets the current day number from the given date
    #[cfg(feature = "time")]
    pub fn get_day(self, date: time::Date) -> usize {
        (date.to_julian_day() - self.date_offset.to_julian_day()) as usize
    }

    /// Determines if the given word is valid, according to the default word lists
    pub fn valid(self, word: &str) -> bool {
        self.solutions.contains(&word) || self.acceptable.contains(&word)
    }
}
