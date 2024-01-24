use anyhow::Result;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader, Stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::RawTerminal;

use crate::args::Args;

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
    args: Args,
}

impl<'a> App<'a> {
    pub fn new(stdout: RawTerminal<&'a Stdout>, args: Args) -> Self {
        App {
            cursor: Cursor::new(1, 1),
            stdout,
            args,
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
        write!(self.stdout, "{}", termion::cursor::Goto(1, 1))?;

        let (width, end) = termion::terminal_size()?;

        let mut start = 0;

        // show file if provided
        if let Some(path) = &self.args.file {
            let f = File::open(path)?;
            let reader = BufReader::new(f);
            for line in reader.lines() {
                let line = line?;
                let length = usize::min(width as usize, line.len());
                let line = &line[..length];
                write!(self.stdout, "{}", line)?;
                // write!(self.stdout, "{}", "test")?;
                if start == end - 1 {
                    break;
                }
                write!(self.stdout, "\n\r")?;
                start += 1;
            }
        }

        for row in start..end {
            write!(self.stdout, "{}", "~")?;
            if row != end - 1 {
                write!(self.stdout, "\n\r")?;
            }
        }
        write!(self.stdout, "{}", termion::cursor::Goto(1, 1))?;
        self.stdout.flush()?;
        Ok(())
    }

    fn move_cursor(&mut self, x_off: i16, y_off: i16) -> Result<()> {
        self.cursor.move_by(&mut self.stdout, x_off, y_off)
    }
}
