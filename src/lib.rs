use anyhow::Result;
use app::App;
use clap::Parser;
use std::io::{stdout, Write};
use std::ops::Deref;
use termion::raw::IntoRawMode;
use termion::screen::IntoAlternateScreen;

mod app;
mod args;
mod banner;
mod cursor;
mod file;
mod input;

pub fn run() -> Result<()> {
    let args = args::Args::parse();

    let stdout = stdout().into_alternate_screen()?;
    let mut stdout = stdout.deref().into_raw_mode()?;
    write!(
        stdout,
        "{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1)
    )
    .unwrap();

    stdout.flush()?;
    let mut app = App::new(stdout, args);
    app.run()
}
