use clap::Parser;
use std::path::PathBuf;

use crate::font::convert_font;
use crate::ship::convert_ship;

mod font;
mod ship;

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

    if let Ok(entries) = std::fs::read_dir(cli.wings_dir.join("SHIPS")) {
        let ships_output = cli.output_dir.join("ships");
        std::fs::create_dir_all(&ships_output).unwrap();

        for entry in entries {
            let entry = entry.unwrap();
            convert_ship(
                entry.path(),
                ships_output.join(entry.file_name()).with_extension("PNG"),
            );
        }
    }
}
