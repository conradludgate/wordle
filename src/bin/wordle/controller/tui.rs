use std::io::{self, Write};

use cl_wordle::{
    game::{Game, GameShare},
    state::GuessError,
    Match,
};
use crossterm::{
    cursor,
    event::{self, KeyCode, KeyModifiers},
    execute,
    terminal::{Clear, ClearType},
};
use eyre::Result;
use owo_colors::{
    colors::{Red, White},
    OwoColorize,
};

mod guess;
mod keyboard;
mod letters;
mod terminal;

use self::{guess::Guesses, keyboard::Keyboard, letters::WordMatch, terminal::Terminal};

pub struct Controller {
    game: Game,
    keyboard: Keyboard,
    stdout: Terminal,
    word: String,
}

impl Controller {
    pub fn new(game: Game) -> Result<Self> {
        Ok(Self {
            game,
            keyboard: Keyboard::default(),
            stdout: Terminal::new()?,
            word: String::with_capacity(5),
        })
    }

    pub fn run(mut self) -> Result<Option<GameShare>> {
        self.display_window()?;

        let game_over = loop {
            self.stdout.flush()?;
            if let event::Event::Key(key) = event::read()? {
                match (key.code, key.modifiers) {
                    (KeyCode::Char('w'), KeyModifiers::CONTROL) => {
                        self.word.clear();
                        self.display_window()?;
                    }
                    (KeyCode::Esc, _) | (KeyCode::Char('c'), KeyModifiers::CONTROL) => {
                        return Ok(None);
                    },
                    (KeyCode::Enter, _) if self.word.len() == 5 => match self.guess() {
                        Ok(()) => {
                            self.display_window()?;

                            if let Some(win) = self.game.game_over() {
                                break win;
                            }
                        }
                        Err(_) => {
                            self.display_window()?;
                            self.display_invalid()?;
                        }
                    },
                    (KeyCode::Char(','), _) => {
                        self.keyboard.shuffle();
                        self.display_window()?;
                    }
                    (KeyCode::Char(c), _) if c.is_ascii_alphabetic() && self.word.len() < 5 => {
                        let c = c.to_ascii_lowercase();
                        write!(self.stdout, "{}", c.to_ascii_uppercase())?;
                        self.word.push(c);
                    }
                    (KeyCode::Backspace, _) => {
                        self.word.pop();
                        if self.word.len() == 4 {
                            write!(
                                self.stdout,
                                "{bol}{word} {back}",
                                back = cursor::MoveLeft(1),
                                bol = cursor::MoveLeft(5),
                                word = self.word.to_ascii_uppercase()
                                )?;
                        } else {
                            write!(self.stdout, "{back} {back}", back = cursor::MoveLeft(1))?;
                        }
                    }
                    _ => {}
                }
            }
        };

        if game_over.is_lose() {
            self.write_final_solution()?;
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

        Ok(Some(self.game.share()))
    }

    fn guess(&mut self) -> Result<(), GuessError> {
        let matches = self.game.guess(&*self.word)?;
        self.keyboard.push(&*self.word, matches);
        self.word.clear();
        Ok(())
    }

    pub fn write_final_solution(&mut self) -> io::Result<()> {
        write!(self.stdout, "{}", cursor::MoveDown(1))?;
        write!(
            self.stdout,
            "{}",
            WordMatch(self.game.solution(), Match::Exact)
        )?;
        write!(self.stdout, "{}", cursor::MoveTo(0, 10))
    }

    fn display_invalid(&mut self) -> io::Result<()> {
        write!(
            self.stdout,
            "{back}{invalid}",
            back = cursor::MoveLeft(5),
            invalid = self.word.to_ascii_uppercase().fg::<White>().bg::<Red>()
        )
    }

    fn display_window(&mut self) -> io::Result<()> {
        let (_width, height) =
            crossterm::terminal::size().map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        write!(
            self.stdout,
            "{clear_all}{bottom_left}> Press ESC to exit. Press ',' to shuffle the keyboard.",
            clear_all = Clear(ClearType::All),
            bottom_left = cursor::MoveTo(0, height - 1),
        )?;

        write!(
            self.stdout,
            "{top_left}Wordle {game_type} {current_guess}/{total_guesses}{hard_mode}{down}{keyboard}{state}{word}",
            top_left = cursor::MoveTo(0, 0),
            game_type = self.game.game_type(),
            current_guess = self.game.current_guess(),
            total_guesses = self.game.max_guess(),
            hard_mode = self.game.hard_mode_indicator(),
            down = cursor::MoveTo(0, 2),
            keyboard = self.keyboard,
            state = Guesses::from(&*self.game),
            word = self.word.to_ascii_uppercase(),
        )?;
        if self.game.guess(&*self.word).is_err() {
            self.display_invalid()?;
        }

        Ok(())
    }
}
