use std::io::Write;

use anyhow::Result;

use crate::output::OutputHandler;

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

    pub fn write_term<'a>(&self, output: &mut OutputHandler<'a>, bottom: u16) -> Result<()> {
        let top = bottom - self.height;
        for line in top..bottom {
            write!(
                output,
                "{}{}",
                termion::cursor::Goto(1, line),
                termion::clear::CurrentLine,
            )?;
            write!(
                output,
                "{}{}{}{}",
                termion::color::Bg(termion::color::Rgb(60, 54, 66)),
                ":",
                " ".repeat(output.size.0 as usize - 1),
                termion::style::Reset,
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
