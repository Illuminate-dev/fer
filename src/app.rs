use anyhow::Result;
use std::io::{stdin, Stdout, Write};
use termion::input::TermRead;
use termion::raw::RawTerminal;

use crate::args::Args;
use crate::banner::Banner;
use crate::cursor::Cursor;
use crate::file::File;
use crate::input::{InputHandler, ReturnCommand};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Mode {
    Normal,
    Insert,
}

impl Mode {
    /// # of spaces to allow cursor to go to after line
    pub fn get_extension(&self) -> usize {
        match self {
            Mode::Normal => 0,
            Mode::Insert => 1,
        }
    }
}

pub struct App<'a> {
    stdout: RawTerminal<&'a Stdout>,
    pub cursor: Cursor,
    pub file: File,
    pub banner: Banner,
    pub current_mode: Mode,
}

impl<'a> App<'a> {
    pub fn new(stdout: RawTerminal<&'a Stdout>, args: Args) -> Self {
        App {
            cursor: Cursor::new(1, 1),
            stdout,
            file: File::new(args.file),
            banner: Banner::new(1),
            current_mode: Mode::Normal,
        }
    }

    pub fn run(&mut self) -> Result<()> {
        self.file.load()?;
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
                ReturnCommand::SaveFile => {
                    self.file.save()?;
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
            &self.file.data,
            self.current_mode,
        )
    }

    pub fn show_cursor(&mut self) -> Result<()> {
        self.cursor.apply_coords(&mut self.stdout)
    }

    fn show_screen(&mut self) -> Result<()> {
        write!(
            self.stdout,
            "{}{}",
            termion::cursor::Goto(1, 1),
            termion::clear::All
        )?;

        let (width, end) = termion::terminal_size()?;

        let mut start = 0;

        // show file if provided
        for line in &self.file.data {
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

        for _row in start..end - self.banner.height {
            write!(self.stdout, "{}", "~")?;
            write!(self.stdout, "\n\r")?;
        }

        // print banner
        write!(
            self.stdout,
            "{}",
            self.banner.display(
                self.cursor.real_x as usize,
                self.cursor.real_y as usize,
                &self.file
            )
        )?;

        self.stdout.flush()?;
        Ok(())
    }

    fn insert_char(&mut self, c: char) -> Result<()> {
        // real_x is 1 indexed, change later
        self.file
            .insert_char(c, self.cursor.file_y, self.cursor.real_x as usize - 1)
    }

    fn del_char(&mut self) -> Result<()> {
        // don't delete at 0
        if self.cursor.real_x == 1 {
            return Ok(());
        }
        self.file
            .del_char(self.cursor.file_y, self.cursor.real_x as usize - 2)
    }
}
