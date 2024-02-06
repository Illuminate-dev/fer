use termion::style::Reset;

use crate::file::File;

pub struct Banner {
    pub height: u16,
}

impl Banner {
    pub fn new(height: u16) -> Self {
        Self { height }
    }

    pub fn display(&self, x: usize, y: usize, file: &File) -> String {
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

        format!("{} - {}{}{}", pos_text, file_text, modified_text, Reset)
    }
}
