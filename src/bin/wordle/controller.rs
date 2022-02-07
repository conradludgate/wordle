use std::{
    fmt::Display,
    io,
    io::{stdout, Stdout, Write},
};

use crossterm::{
    cursor,
    event::{self, KeyCode},
    execute, style, terminal,
};
use eyre::Result;
use owo_colors::{colors::Red, OwoColorize};

use cl_wordle as wordle;

use crate::game::Game;

pub struct GameController {
    game: Game,
    stdout: Terminal,
}

struct Terminal(Stdout);

impl Write for Terminal {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.0.flush()
    }
}

impl Terminal {
    fn new() -> io::Result<Self> {
        let mut stdout = stdout();
        execute!(stdout, terminal::EnterAlternateScreen)?;
        terminal::enable_raw_mode()?;
        Ok(Self(stdout))
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        execute!(
            self.0,
            style::ResetColor,
            cursor::Show,
            terminal::LeaveAlternateScreen
        )
        .unwrap();
        terminal::disable_raw_mode().unwrap();
    }
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
                    KeyCode::Enter if word.len() == 5 => {
                        if wordle::valid(&word) {
                            self.game.push(&word);
                            self.display_window()?;

                            if let Some(win) = self.game.finish() {
                                break win;
                            }

                            word.clear();
                        } else {
                            self.display_invalid(&word)?;
                        }
                    }
                    KeyCode::Backspace => {
                        word.pop();
                        write!(self.stdout, "{back} {back}", back = cursor::MoveLeft(1))?;
                    }
                    _ => {}
                }
            }
        };

        if !win {
            self.game.write_final_solution(&mut self.stdout)?;
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
