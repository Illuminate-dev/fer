use anyhow::Result;
use std::io::{stdin, Stdout, Write};
use termion::input::TermRead;
use termion::raw::RawTerminal;

use crate::args::Args;
use crate::banner::Banner;
use crate::cursor::Cursor;
use crate::edit_terminal::Terminal;
use crate::file::File;
use crate::input::{InputHandler, ReturnCommand};
use crate::output::OutputHandler;

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

    /// returns string containing the code to change cursor to the relevant mode
    pub fn get_cursor_set_string(&self) -> String {
        match self {
            Mode::Normal => termion::cursor::SteadyBlock.to_string(),
            Mode::Insert => termion::cursor::SteadyBar.to_string(),
        }
    }
}

pub enum Focus {
    File,
    Term,
}

pub struct App<'a> {
    pub output: OutputHandler<'a>,
    pub cursor: Cursor,
    pub file: File,
    pub banner: Banner,
    pub current_mode: Mode,
    pub term: Terminal,
    pub focused: Focus,
}

impl<'a> App<'a> {
    pub fn new(stdout: RawTerminal<&'a Stdout>, args: Args) -> Self {
        App {
            cursor: Cursor::new(0, 0),
            output: OutputHandler::new(stdout),
            file: File::new(args.file),
            banner: Banner::new(1),
            current_mode: Mode::Normal,
            term: Terminal::new(1),
            focused: Focus::File,
        }
    }

    pub fn run(&mut self) -> Result<()> {
        self.file.load()?;
        self.show_screen()?;
        write!(self.output, "{}", termion::cursor::Goto(1, 1))?;
        self.output.flush()?;

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
                ReturnCommand::Backspace => {
                    self.backspace()?;
                    self.show_screen()?;
                    self.move_cursor(-1, 0)?;
                    self.show_cursor()?;
                }
                ReturnCommand::SaveFile => {
                    self.file.save()?;
                }
                ReturnCommand::ToggleTerm => {
                    self.term.toggle();
                    self.focused = match self.focused {
                        Focus::File => Focus::Term,
                        Focus::Term => Focus::File,
                    };
                }
                ReturnCommand::None => {}
            }
            self.show_screen()?;
            self.show_cursor()?;

            self.output.flush()?;
        }
        Ok(())
    }

    // is this in the right place?
    pub fn move_cursor(&mut self, x_off: isize, y_off: isize) -> Result<()> {
        self.cursor
            .move_and_set_coords(x_off, y_off, &self.file.data, self.current_mode)?;

        // TODO: scrolling/page positioning

        self.show_cursor()
    }

    pub fn show_cursor(&mut self) -> Result<()> {
        let coords = self
            .output
            .temp_clamp_coords(self.cursor.file_x + 1, self.cursor.file_y + 1);

        write!(self.output, "{}", termion::cursor::Goto(coords.0, coords.1))?;
        self.output.flush()?;
        Ok(())
    }

    fn show_screen(&mut self) -> Result<()> {
        write!(
            self.output,
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
            write!(self.output, "{}", line)?;
            // write!(self.stdout, "{}", "test")?;
            if start == end - 1 {
                break;
            }
            write!(self.output, "\n\r")?;
            start += 1;
        }

        if self.file.data.len() == 0 {
            start += 1;
            write!(self.output, "\n\r")?;
        }

        let bottom_offset = self.banner.height + self.term.get_real_height();

        for _row in start..end - bottom_offset {
            write!(self.output, "{}", "~")?;
            write!(self.output, "\n\r")?;
        }

        // write term
        if self.term.visible {
            self.write_term(end - self.banner.height)?;
        }

        // write banner
        self.write_banner(end)?;

        self.output.flush()?;
        Ok(())
    }

    fn insert_char(&mut self, c: char) -> Result<()> {
        // real_x is 1 indexed, change later
        self.file
            .insert_char(c, self.cursor.file_y, self.cursor.file_x)
    }

    fn backspace(&mut self) -> Result<()> {
        // don't delete at 0 on backspace
        if self.cursor.file_x == 0 {
            return Ok(());
        }

        self.file
            .del_char(self.cursor.file_y, self.cursor.file_x - 1)
    }

    pub fn update_mode(&mut self, mode: Mode) -> Result<()> {
        self.current_mode = mode;
        write!(self.output, "{}", self.current_mode.get_cursor_set_string())?;
        self.output.flush()?;
        Ok(())
    }

    pub fn write_banner(&mut self, bottom: u16) -> Result<()> {
        self.banner.write_banner(
            &mut self.output,
            self.cursor.file_x,
            self.cursor.file_y,
            &self.file,
            &self.current_mode,
            bottom,
        )?;
        // reset cursor position back to stored value
        self.show_cursor()
    }

    pub fn write_term(&mut self, bottom: u16) -> Result<()> {
        self.term.write_term(&mut self.output, bottom)
    }
}
