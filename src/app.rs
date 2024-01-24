use anyhow::Result;
use std::io::{stdin, Stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::RawTerminal;

/// 1 based
struct Cursor {
    x: u16,
    y: u16,
}

impl Cursor {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }

    fn move_by<'a>(
        &mut self,
        stdout: &mut RawTerminal<&'a Stdout>,
        x_off: i16,
        y_off: i16,
    ) -> Result<()> {
        let mut x = x_off + self.x as i16;
        let mut y = y_off + self.y as i16;

        if x < 1 {
            x = 1;
        }
        if y < 1 {
            y = 1;
        }

        let mut x = x as u16;
        let mut y = y as u16;

        let size = termion::terminal_size()?;

        if x > size.0 {
            x = size.0;
        }
        if y > size.1 {
            y = size.1;
        }

        self.x = x;
        self.y = y;

        write!(stdout, "{}", termion::cursor::Goto(self.x, self.y))?;
        stdout.flush()?;
        Ok(())
    }
}

pub struct App<'a> {
    cursor: Cursor,
    stdout: RawTerminal<&'a Stdout>,
}

impl<'a> App<'a> {
    pub fn new(stdout: RawTerminal<&'a Stdout>) -> Self {
        App {
            cursor: Cursor::new(1, 1),
            stdout,
        }
    }

    pub fn run(&mut self) -> Result<()> {
        self.show_screen()?;

        let stdin = stdin();

        for c in stdin.keys() {
            match c? {
                Key::Ctrl('q') => break,
                Key::Char('j') => self.move_cursor(0, 1)?,
                Key::Char('k') => self.move_cursor(0, -1)?,
                Key::Char('h') => self.move_cursor(-1, 0)?,
                Key::Char('l') => self.move_cursor(1, 0)?,
                _ => {}
            }
            self.stdout.flush()?;
        }
        Ok(())
    }

    fn show_screen(&mut self) -> Result<()> {
        for _row in 0..termion::terminal_size()?.1 {
            write!(self.stdout, "{}\r\n", "~")?;
        }
        write!(self.stdout, "{}", termion::cursor::Goto(1, 1))?;
        self.stdout.flush()?;
        Ok(())
    }

    fn move_cursor(&mut self, x_off: i16, y_off: i16) -> Result<()> {
        self.cursor.move_by(&mut self.stdout, x_off, y_off)
    }
}
