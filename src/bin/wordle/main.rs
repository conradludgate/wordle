use clap::Parser;

mod args;
mod game;

use args::{App, GameMode};
use game::Game;
use rand::Rng;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let app = App::parse();
    let game = match app.game_mode {
        None => Game::new()?,
        Some(GameMode::Custom(custom)) => Game::custom(custom.word)?,
        Some(GameMode::Day(day)) => Game::from_day(day.day)?,
        Some(GameMode::Random) => Game::from_day(rand::thread_rng().gen())?,
    };

    if let Some(share) = game.start()? {
        println!("{}", share);
    }

    Ok(())
}
