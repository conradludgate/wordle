use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(name = "wordle", author, version)]
pub struct App {
    #[clap(subcommand)]
    pub game_mode: Option<GameMode>,
}

#[derive(Subcommand)]
pub enum GameMode {
    /// Play with a custom word
    Custom(Custom),
    /// Play the specified day
    Day(Day),
    /// Play a random day
    Random,
}

#[derive(Parser)]
pub struct Custom {
    pub word: String,
}

#[derive(Parser)]
pub struct Day {
    pub day: usize,
}
