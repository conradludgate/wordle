use std::collections::HashMap;

use crate::{game::Game, iter::Guess};

pub struct Cheat {
    answers: Vec<&'static str>,
}

impl Cheat {
    pub fn new() -> Self {
        Self { answers: crate::words::FINAL.to_owned() }
    }

    pub fn guess(&mut self, game: &mut Game) -> Guess<'static> {
        let next_move = self.next_move();
        let matches = game.guess(next_move).expect("cheat made an invalid guess");

        let mut i = 0;
        while i < self.answers.len() {
            if crate::diff(next_move, self.answers[i]) == matches {
                i += 1;
            } else {
                self.answers.swap_remove(i);
            }
        }

        (next_move, matches)
    }

    fn next_move(&self) -> &'static str {
        if self.answers.len() == crate::words::FINAL.len() {
            return "soare";
        }

        let n = self.answers.len() as f64;
        let mut available_moves = crate::words::ACCEPT.to_vec();
        available_moves.extend_from_slice(&*self.answers);
        let mut matches = HashMap::new();

        available_moves.sort_by_cached_key(|&can_move| {
            for answer in &self.answers {
                *matches.entry(crate::diff(can_move, answer)).or_insert(0.0) += 1.0;
            }

            let entropy = matches.drain().map(|(_, v)| v / n).map(|v| v * v.log2()).sum::<f64>();
            (entropy * -1000.0) as i32
        });

        available_moves.last().unwrap()

        // for can_move in available_moves {
        // }

        // replace with entropy calc
        // self.answers.first().unwrap()
    }
}

// def entropy_of_distributions(distributions, atol=1e-12):
//     axis = len(distributions.shape) - 1
//     return entropy(distributions, base=2, axis=axis)


// def get_entropies(allowed_words, possible_words, weights):
//     if weights.sum() == 0:
//         return np.zeros(len(allowed_words))
//     distributions = get_pattern_distributions(allowed_words, possible_words, weights)
//     return entropy_of_distributions(distributions)

impl Default for Cheat {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use indicatif::ProgressIterator;

    use crate::game::Game;

    use super::Cheat;

    #[test]
    fn play_all() {
        let n = crate::words::FINAL.len();
        let mut scores = Vec::with_capacity(n);
        for day in (0..n).progress() {
            scores.push(play(day).unwrap());
        }
        dbg!(scores.into_iter().sum::<usize>());
    }

    fn play(day: usize) -> Result<usize, usize> {
        let mut cheat = Cheat::new();
        let mut game = Game::from_day(day);
        for i in 0..6 {
            if cheat.guess(&mut game).1.win() {
                return Ok(i)
            }
        }
        Err(day)
    }
}
