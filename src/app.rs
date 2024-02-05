use anyhow::{anyhow, Result};
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
        write!(self.stdout, "{}", termion::cursor::Goto(1, 1))?;
        self.stdout.flush()?;

        let stdin = stdin();

        for c in stdin.keys() {
            match InputHandler::handle_input(self, c?)? {
                ReturnCommand::Quit => break,
                ReturnCommand::AddChar(c) => {
                    self.insert_char(c)?;
                    self.show_screen()?;
                    self.move_cursor(1, 0)?;
                    self.show_cursor()?;
                }
                ReturnCommand::DelChar => {
                    self.del_char()?;
                    self.show_screen()?;
                    self.move_cursor(-1, 0)?;
                    self.show_cursor()?;
                }
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

    pub fn show_cursor(&mut self) -> Result<()> {
        self.cursor.apply_coords(&mut self.stdout)
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

    fn insert_char(&mut self, c: char) -> Result<()> {
        self.file_data
            .as_deref_mut()
            .ok_or(anyhow!("No file; can't insert character"))?[self.cursor.file_y]
            .insert(self.cursor.real_x as usize - 1, c);
        Ok(())
    }

    fn del_char(&mut self) -> Result<()> {
        if self.cursor.real_x == 1 {
            return Ok(());
        }
        self.file_data
            .as_deref_mut()
            .ok_or(anyhow!("No file; can't insert character"))?[self.cursor.file_y]
            .remove(self.cursor.real_x as usize - 2);
        Ok(())
    }
}
