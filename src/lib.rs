use anyhow::Result;
use app::App;
use std::io::{stdout, Write};
use termion::raw::IntoRawMode;

mod app;

pub fn run() -> Result<()> {
    let mut stdout = stdout().into_raw_mode()?;

    write!(
        stdout,
        "{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1)
    )
    .unwrap();

    stdout.flush().unwrap();
    let mut app = App::new(stdout);
    app.run()
}
