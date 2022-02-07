use cl_wordle::game::Game;
use clap::Parser;

mod args;
mod controller;

use args::{App, GameMode};
use controller::cli;

#[cfg(feature = "rand")]
use rand::Rng;

fn main() -> eyre::Result<()> {
    let app = App::parse();
    let game = match app.game_mode {
        None => Game::new()?,
        Some(GameMode::Custom(custom)) => Game::custom(custom.word)?,
        Some(GameMode::Day(day)) => Game::from_day(day.day)?,
        #[cfg(feature = "rand")]
        Some(GameMode::Random) => Game::from_day(rand::thread_rng().gen())?,
        Some(GameMode::Date(date)) => Game::from_date(date.date)?,
    };

    #[cfg(feature = "tui")]
    let output = if app.no_tui {
        Some(cli::Controller::new(game).run()?)
    } else {
        controller::tui::Controller::new(game)?.run()?
    };
    #[cfg(not(feature = "tui"))]
    let output = Some(cli::Controller::new(game).run()?);

    if let Some(share) = output {
        println!("{}", share);
    }

    Ok(())
}
