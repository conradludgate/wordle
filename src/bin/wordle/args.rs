use clap::{Parser, Subcommand};
use time::macros::format_description;

#[derive(Parser)]
#[clap(name = "wordle", author, version)]
pub struct App {
    #[clap(long)]
    #[cfg(feature = "tui")]
    /// Disables the TUI
    ///
    /// This disables the TUI and any enriched experience
    pub no_tui: bool,

    #[clap(long)]
    /// Enables hard mode
    ///
    /// Hard mode adds the restriction that you must only make
    /// guesses including the all exact matches you have found
    pub hard: bool,

    #[clap(long)]
    /// Enables Original mode
    ///
    /// The NYTimes bought and are now hosting their own
    /// version of wordle. It's mostly the same
    /// but features a ever so slightly adapted word list.
    /// This disables using the new list and instead uses the original
    pub original: bool,

    #[clap(subcommand)]
    pub game_mode: Option<GameMode>,
}

#[derive(Subcommand)]
pub enum GameMode {
    /// Play with a custom word
    Custom(Custom),
    /// Play the specified day's wordle
    Day(Day),
    /// Play the specified date's wordle
    Date(Date),
    #[cfg(feature = "rand")]
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

#[derive(Parser)]
pub struct Date {
    #[clap(parse(try_from_str = parse_date))]
    /// year-month-day
    pub date: time::Date,
}

fn parse_date(input: &str) -> Result<time::Date, time::error::Parse> {
    let description = format_description!("[year]-[month padding:none]-[day padding:none]");
    time::Date::parse(input, description)
}
