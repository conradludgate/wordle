use std::{
    io::{self, stdout, Stdout},
    ops::{Deref, DerefMut},
};

use crossterm::{cursor, execute, style, terminal};

pub struct Terminal(Stdout);
impl Deref for Terminal {
    type Target = Stdout;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Terminal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Terminal {
    pub fn new() -> io::Result<Self> {
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
