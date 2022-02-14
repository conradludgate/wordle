use crate::Matches;

pub type Guess<'a> = (&'a str, Matches);

/// Iterator over guesses in a game state
pub struct StateIter<'a> {
    pub(crate) solution: &'a str,
    pub(crate) guesses: std::slice::Iter<'a, String>,
}

impl<'a> Iterator for StateIter<'a> {
    type Item = Guess<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.guesses
            .next()
            .map(|input| (input.as_str(), crate::diff(&*input, self.solution)))
    }
}

impl<'a> DoubleEndedIterator for StateIter<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.guesses
            .next_back()
            .map(|input| (input.as_str(), crate::diff(&*input, self.solution)))
    }
}
