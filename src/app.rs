use anyhow::Result;
use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

/// 1 based
struct Cursor {
    x: u16,
    y: u16,
}

impl Cursor {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
}

pub struct App {
    cursor: Cursor,
}

impl App {
    pub fn new() -> Self {
        App {
            cursor: Cursor::new(1, 1),
        }
    }

    pub fn run(&mut self) -> Result<()> {
        self.show_screen()?;

        let stdin = stdin();
        let mut stdout = stdout();

        for c in stdin.keys() {
            match c? {
                Key::Ctrl('q') => break,
                c => {
                    write!(stdout, "{:?}\r\n", c)?;
                }
            }
            stdout.flush()?;
        }
        Ok(())
    }

    pub fn show_screen(&mut self) -> Result<()> {
        Ok(())
    }
}
