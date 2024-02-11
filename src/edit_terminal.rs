use std::io::Stdout;
use std::io::Write;

use anyhow::Result;
use termion::raw::RawTerminal;

pub struct Terminal {
    pub visible: bool,
    pub height: u16,
}

impl Terminal {
    pub fn new(height: u16) -> Self {
        Self {
            visible: false,
            height,
        }
    }

    pub fn write_term<'a>(&self, stdout: &mut RawTerminal<&'a Stdout>, bottom: u16) -> Result<()> {
        let top = bottom - self.height;
        for line in top..bottom {
            write!(
                stdout,
                "{}{}{}",
                termion::cursor::Goto(1, line),
                termion::clear::CurrentLine,
                ":"
            )?;
        }

        Ok(())
    }

    pub fn get_real_height(&self) -> u16 {
        if self.visible {
            self.height
        } else {
            0
        }
    }

    pub fn toggle(&mut self) {
        self.visible = !self.visible;
    }
}
