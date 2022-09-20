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

    // Copy to output and change file extension to .PCX
    let pcx_files = &[
        "WINGS2.PIC",
        "WINGS.PIC",
        "W_PICT2.PIC",
        "W_PICT.PIC",
        "W_WEAP.PIC",
    ];
    for file in pcx_files {
        std::fs::copy(
            cli.wings_dir.join(file),
            cli.output_dir.join(file).with_extension("PCX"),
        )
        .unwrap();
    }
}
