use std::path::PathBuf;

use clap;

use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version , about, long_about = None)]
pub struct Args {
    /// path for the icon
    #[arg(short, long)]
    pub icon: Option<PathBuf>,
}
