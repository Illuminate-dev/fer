use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// The file to open. If none is provided, then create a new empty file
    pub file: Option<PathBuf>,
}
