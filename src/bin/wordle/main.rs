use clap::Parser;

mod args;
mod game;

use args::{App, GameMode};
use game::GameController;
use rand::Rng;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let app = App::parse();
    let game = match app.game_mode {
        None => GameController::new()?,
        Some(GameMode::Custom(custom)) => GameController::custom(custom.word)?,
        Some(GameMode::Day(day)) => GameController::from_day(day.day)?,
        Some(GameMode::Random) => GameController::from_day(rand::thread_rng().gen())?,
    };

    if let Some(share) = game.start()? {
        println!("{}", share);
    }

    Ok(())
}
