use anyhow::Result;
use std::io::{Stdout, Write};
use termion::raw::RawTerminal;

use crate::app::Mode;

/// 1 based
pub struct Cursor {
    pub real_x: u16,
    pub real_y: u16,
    /// 0 is the first line of the file, etc
    pub file_y: usize,
}

impl Cursor {
    pub fn new(real_x: u16, real_y: u16) -> Self {
        Self {
            real_x,
            real_y,
            file_y: 0,
        }
    }

    pub fn move_by<'a>(
        &mut self,
        stdout: &mut RawTerminal<&'a Stdout>,
        x_off: i16,
        y_off: i16,
        file_data: &Vec<String>,
        mode: Mode,
    ) -> Result<()> {
        (self.real_x, self.real_y, self.file_y) =
            self.get_final_coords(x_off, y_off, file_data, mode)?;

        self.apply_coords(stdout)?;

        Ok(())
    }
    /// returns (x, y, file_y, mem_x)
    /// file_data is to determine length of each line
    /// Mode is to add extre length for insert
    pub fn get_final_coords(
        &self,
        x_off: i16,
        y_off: i16,
        file_data: &Vec<String>,
        mode: Mode,
    ) -> Result<(u16, u16, usize)> {
        let mut x = x_off + self.real_x as i16;
        let mut y = y_off + self.real_y as i16;
        let mut file_y = y_off as i32 + self.file_y as i32;

        if x < 1 {
            x = 1;
        }
        if y < 1 {
            y = 1;
        }

        if file_y < 1 {
            file_y = 1;
        }

        let mut x = x as u16;
        let mut y = y as u16;

        // reset x to be end of the line in the file
        let file_y = file_y as usize;
        if file_y < file_data.len() {
            if x as usize > file_data[file_y as usize].len() + mode.get_extension() {
                x = u16::max(file_data[file_y as usize].len() as u16, 1);
            }
        }

        let size = termion::terminal_size()?;

        if x > size.0 {
            x = size.0;
        }
        if y > size.1 {
            y = size.1;
        }

        Ok((x, y, file_y))
    }

    pub fn apply_coords<'a>(&mut self, stdout: &mut RawTerminal<&'a Stdout>) -> Result<()> {
        write!(
            stdout,
            "{}",
            termion::cursor::Goto(self.real_x, self.real_y)
        )?;
        stdout.flush()?;

        Ok(())
    }
}
