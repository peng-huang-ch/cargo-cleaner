use std::path::PathBuf;

use clap::Parser;
use eyre::Ok;

use crate::walk_dirs::walk_dirs;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The directory to execute the command
    #[arg(long, default_value = ".")]
    path: String,

    /// The depth to descend into the directory
    #[arg(short, long, default_value_t = 1)]
    depth: u32,
}

pub fn run() -> eyre::Result<()> {
    let cli = Cli::parse();
    let dir = PathBuf::from(cli.path)
        .canonicalize()
        .expect("Failed to resolve path");
    let max_depth = cli.depth;
    walk_dirs(&dir, 1, max_depth)?;
    Ok(())
}
