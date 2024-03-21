use std::io::{Stdout, Write};
use termion::raw::RawTerminal;

pub struct OutputHandler<'a> {
    stdout: RawTerminal<&'a Stdout>,
    pub size: (u16, u16),
}

impl<'a> OutputHandler<'a> {
    pub fn new(stdout: RawTerminal<&'a Stdout>) -> Self {
        OutputHandler {
            stdout,
            size: termion::terminal_size().unwrap_or((0, 0)),
        }
    }

    pub fn temp_clamp_coords(&self, x: usize, y: usize) -> (u16, u16) {
        (
            x.clamp(1, self.size.0 as usize) as u16,
            y.clamp(1, self.size.1 as usize) as u16,
        )
    }
}

impl Write for OutputHandler<'_> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.stdout.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.stdout.flush()
    }
}
