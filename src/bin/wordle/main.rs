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

    let word_set = if app.original {
        cl_wordle::words::ORIGINAL
    } else {
        cl_wordle::words::NYTIMES
    };

    let mut game = match app.game_mode {
        None => Game::new(word_set)?,
        Some(GameMode::Custom(custom)) => Game::custom(custom.word, word_set)?,
        Some(GameMode::Day(day)) => Game::from_day(day.day, word_set),
        #[cfg(feature = "rand")]
        Some(GameMode::Random) => Game::from_day(rand::thread_rng().gen(), word_set),
        Some(GameMode::Date(date)) => Game::from_date(date.date, word_set),
    };

    if app.hard {
        game.hard_mode();
    }

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
