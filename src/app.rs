use anyhow::Result;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader, Stdout, Write};
use termion::input::TermRead;
use termion::raw::RawTerminal;

use crate::args::Args;
use crate::cursor::Cursor;
use crate::input::{InputHandler, ReturnCommand};

pub enum Mode {
    Normal,
    Insert,
}

pub struct App<'a> {
    pub cursor: Cursor,
    stdout: RawTerminal<&'a Stdout>,
    args: Args,
    file_data: Option<Vec<String>>,
    pub current_mode: Mode,
}

impl<'a> App<'a> {
    pub fn new(stdout: RawTerminal<&'a Stdout>, args: Args) -> Self {
        App {
            cursor: Cursor::new(1, 1),
            stdout,
            args,
            file_data: None,
            current_mode: Mode::Normal,
        }
    }

    pub fn run(&mut self) -> Result<()> {
        self.load_file()?;
        self.show_screen()?;

        let stdin = stdin();

        for c in stdin.keys() {
            match InputHandler::handle_input(self, c?)? {
                ReturnCommand::Quit => break,
                ReturnCommand::None => {}
            }
            self.stdout.flush()?;
        }
        Ok(())
    }

    // is this in the right place?
    pub fn move_cursor(&mut self, x_off: i16, y_off: i16) -> Result<()> {
        self.cursor.move_by(
            &mut self.stdout,
            x_off,
            y_off,
            self.file_data.as_ref().unwrap_or(&vec![]),
        )
    }

    fn show_screen(&mut self) -> Result<()> {
        write!(self.stdout, "{}", termion::cursor::Goto(1, 1))?;

        let (width, end) = termion::terminal_size()?;

        let mut start = 0;

        // show file if provided
        for line in self.file_data.as_ref().unwrap_or(&vec![]) {
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

    fn load_file(&mut self) -> Result<()> {
        let mut v = Vec::new();
        if let Some(path) = &self.args.file {
            let f = File::open(path)?;
            let reader = BufReader::new(f);
            let mut lines = reader.lines();
            while let Some(line) = lines.next() {
                v.push(line?);
            }
        }
        self.file_data = Some(v);
        Ok(())
    }
}
