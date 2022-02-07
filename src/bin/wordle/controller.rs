use std::{
    fmt::Display,
    io::{self, Write},
};

use crossterm::{
    cursor,
    event::{self, KeyCode},
    execute,
};
use eyre::Result;
use owo_colors::{colors::Red, OwoColorize};

use crate::{game::Game, terminal::Terminal};

pub struct GameController {
    game: Game,
    stdout: Terminal,
}

impl GameController {
    pub fn new(game: Game) -> Result<Self> {
        Ok(Self {
            game,
            stdout: Terminal::new()?,
        })
    }

    pub fn run(mut self) -> Result<Option<GameShare>> {
        self.display_window()?;

        let mut word = String::with_capacity(5);

        let win = loop {
            self.stdout.flush()?;
            if let event::Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Esc => return Ok(None),
                    KeyCode::Char(c) if c.is_ascii_alphabetic() && word.len() < 5 => {
                        let c = c.to_ascii_lowercase();
                        write!(self.stdout, "{}", c.to_ascii_uppercase())?;
                        word.push(c);
                    }
                    KeyCode::Enter if word.len() == 5 => match self.game.guess(&*word) {
                        Ok(()) => {
                            self.display_window()?;

                            if let Some(win) = self.game.over() {
                                break win;
                            }

                            word.clear();
                        }
                        Err(_) => self.display_invalid(&word)?,
                    },
                    KeyCode::Backspace => {
                        word.pop();
                        write!(self.stdout, "{back} {back}", back = cursor::MoveLeft(1))?;
                    }
                    _ => {}
                }
            }
        };

        if !win {
            self.game.write_final_solution(&mut *self.stdout)?;
        }

        execute!(self.stdout, cursor::Hide)?;

        loop {
            self.stdout.flush()?;
            if let event::Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Enter | KeyCode::Esc => break,
                    _ => {}
                }
            }
        }

        let share = self.share()?;
        Ok(Some(share))
    }

    fn share(self) -> Result<GameShare> {
        Ok(GameShare(self.game))
    }

    fn display_invalid(&mut self, invalid: &str) -> io::Result<()> {
        self.display_window()?;
        write!(self.stdout, "{}", invalid.to_ascii_uppercase().bg::<Red>())
    }

    fn display_window(&mut self) -> io::Result<()> {
        write!(self.stdout, "{}", self.game)
    }
}

pub struct GameShare(Game);

impl Display for GameShare {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.display_share_card(f)
    }
}
