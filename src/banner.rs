use std::io::Stdout;
use std::io::Write;

use anyhow::Result;
use termion::{raw::RawTerminal, style::Reset};

use crate::{app::Mode, file::File};

pub struct Banner {
    pub height: u16,
}

impl Banner {
    pub fn new(height: u16) -> Self {
        Self { height }
    }

    fn display_text(&self, x: usize, y: usize, file: &File, mode: &Mode) -> String {
        let normal_style = format!("{}", termion::color::Fg(termion::color::White),);

        let position = format!("{}:{}", y, x);
        let pos_text = format!("{}{}{}", normal_style, position, Reset);

        let file_name = file.get_path_name();
        let file_name_color = if file.path.is_none() {
            format!(
                "{}{}",
                termion::color::Bg(termion::color::Red),
                termion::color::Fg(termion::color::White),
            )
        } else {
            "".to_string()
        };
        let file_text = format!("{}{}{}", file_name_color, file_name, Reset);

        let modified_text = if file.modified {
            format!(
                "{}{} [+]",
                normal_style,
                termion::color::Fg(termion::color::Green)
            )
        } else {
            "".to_string()
        };

        let mode_text = match mode {
            Mode::Normal => {
                format!(
                    "{}{}{}",
                    termion::color::Fg(termion::color::LightBlue),
                    "Normal",
                    Reset
                )
            }
            Mode::Insert => {
                format!(
                    "{}{}{}",
                    termion::color::Fg(termion::color::Green),
                    "Insert",
                    Reset
                )
            }
        };

        format!(
            "{} {} - {}{}{}",
            pos_text, mode_text, file_text, modified_text, Reset
        )
    }

    pub fn write_banner<'a>(
        &mut self,
        stdout: &mut RawTerminal<&'a Stdout>,
        x: usize,
        y: usize,
        file: &File,
        mode: &Mode,
        bottom: u16,
    ) -> Result<()> {
        let text = self.display_text(x, y, file, mode);

        let y = bottom - self.height;

        write!(
            stdout,
            "{}{}{}",
            termion::cursor::Goto(1, y),
            termion::clear::CurrentLine,
            text
        )?;
        stdout.flush()?;
        Ok(())
    }
}
