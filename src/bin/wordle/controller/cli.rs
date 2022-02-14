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

        let game_over = loop {
            line.clear();
            stdin.read_line(&mut line)?;

            match self.game.guess(line.trim_end()) {
                Err(_) => println!("INVALID"),
                Ok(m) => {
                    println!("{}", m);
                    if let Some(win) = self.game.game_over() {
                        break win;
                    }
                }
            }
        };

        if game_over.is_lose() {
            println!(
                "GAME OVER - '{}'",
                self.game.solution().to_ascii_uppercase()
            );
        }

        Ok(self.game.share())
    }
}
