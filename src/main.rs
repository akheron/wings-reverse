use clap::Parser;
use std::path::PathBuf;

use crate::font::convert_font;

mod font;

#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
    /// Where to read Wings data files from
    wings_dir: PathBuf,

    /// Where to write output files
    output_dir: PathBuf,
}

fn main() {
    let cli = Cli::parse();

    std::fs::create_dir_all(&cli.output_dir).unwrap();
    convert_font(
        cli.wings_dir.join("VGAFONT1.PIC"),
        cli.output_dir.join("VGAFONT1.PNG"),
    );
}
