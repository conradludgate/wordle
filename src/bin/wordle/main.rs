use clap::Parser;

mod args;
mod controller;
mod game;
mod guess;
mod keyboard;
mod letters;
mod terminal;

use args::{App, GameMode};
use controller::GameController;
use game::Game;
use rand::Rng;

fn main() -> eyre::Result<()> {
    let app = App::parse();
    let game = match app.game_mode {
        None => Game::new()?,
        Some(GameMode::Custom(custom)) => Game::custom(custom.word)?,
        Some(GameMode::Day(day)) => Game::from_day(day.day)?,
        Some(GameMode::Random) => Game::from_day(rand::thread_rng().gen())?,
        Some(GameMode::Date(date)) => Game::from_date(date.date)?,
    };

    let controller = GameController::new(game)?;
    if let Some(share) = controller.run()? {
        println!("{}", share);
    }

    Ok(())
}
