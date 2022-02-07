use std::io::stdin;

use eyre::Result;

use cl_wordle::game::{Game, GameShare};

pub struct Controller {
    game: Game,
}

impl Controller {
    pub fn new(game: Game) -> Self {
        Self { game }
    }

    pub fn run(mut self) -> Result<GameShare> {
        let stdin = stdin();
        let mut line = String::with_capacity(6);

        let win = loop {
            line.clear();
            stdin.read_line(&mut line)?;

            match self.game.state_mut().guess(line.trim_end()) {
                Err(_) => println!("INVALID"),
                Ok(m) => {
                    println!("{}", m);
                    if let Some(win) = self.game.state().game_over() {
                        break win;
                    }
                }
            }
        };

        if !win {
            println!(
                "GAME OVER - '{}'",
                self.game.state().solution().to_ascii_uppercase()
            );
        }

        Ok(self.game.share())
    }
}
