use std::{
    fmt::Display,
    io,
    io::{stdin, stdout, Stdout, Write},
};

use eyre::{Context, Result};
use owo_colors::{colors::Red, OwoColorize};
use termion::{
    cursor,
    event::Key,
    input::{MouseTerminal, TermRead},
    raw::{IntoRawMode, RawTerminal},
};

use cl_wordle as wordle;

use crate::game::Game;

pub struct GameController {
    game: Game,
    terminal: MouseTerminal<RawTerminal<Stdout>>,
}

impl GameController {
    pub fn new(game: Game) -> Result<Self> {
        Ok(Self {
            game,
            terminal: MouseTerminal::from(
                stdout()
                    .into_raw_mode()
                    .wrap_err("could not get handle on a TTY")?,
            ),
        })
    }

    pub fn run(mut self) -> Result<Option<GameShare>> {
        self.display_window()?;
        self.terminal.flush()?;

        let mut word = String::with_capacity(5);

        for c in stdin().keys() {
            let evt = c?;
            match evt {
                Key::Esc => return Ok(None),
                Key::Char(c) if c.is_ascii_alphabetic() && word.len() < 5 => {
                    let c = c.to_ascii_lowercase();
                    write!(self.terminal, "{}", c.to_ascii_uppercase())?;
                    word.push(c);
                }
                Key::Char('\n') if word.len() == 5 => {
                    if wordle::valid(&word) {
                        self.game.push(&word);
                        self.display_window()?;

                        match self.game.finish() {
                            Some(true) => {
                                return Ok(Some(self.share()?));
                            }
                            Some(false) => {
                                self.game.write_final_solution(&mut self.terminal)?;
                                return Ok(Some(self.share()?));
                            }
                            None => {}
                        }

                        word.clear();
                    } else {
                        self.display_invalid(&word)?;
                    }
                }
                Key::Backspace => {
                    word.pop();
                    write!(self.terminal, "{back} {back}", back = cursor::Left(1))?;
                }
                _ => {}
            }
            self.terminal.flush()?;
        }

        Ok(None)
    }

    fn share(mut self) -> Result<GameShare> {
        write!(self.terminal, "{}", cursor::Goto(1, 12))?;
        Ok(GameShare(self.game))
    }

    fn display_invalid(&mut self, invalid: &str) -> io::Result<()> {
        self.display_window()?;
        write!(
            self.terminal,
            "{}",
            invalid.to_ascii_uppercase().bg::<Red>()
        )
    }

    fn display_window(&mut self) -> io::Result<()> {
        write!(self.terminal, "{}", self.game)
    }
}

pub struct GameShare(Game);

impl Display for GameShare {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.display_share_card(f)
    }
}
