use crate::file::File;

pub struct Banner {
    pub height: u16,
}

impl Banner {
    pub fn new(height: u16) -> Self {
        Self { height }
    }

    pub fn display(&self, x: usize, y: usize, file: &File) -> String {
        format!(
            "{}{}{}:{} - {}{}{}",
            termion::color::Bg(termion::color::White),
            termion::color::Fg(termion::color::Black),
            x,
            y,
            if file.path.is_none() {
                format!(
                    "{}{}",
                    termion::color::Bg(termion::color::Red),
                    termion::color::Fg(termion::color::White),
                )
            } else {
                "".to_string()
            },
            file.get_path_name(),
            termion::style::Reset,
        )
    }
}
