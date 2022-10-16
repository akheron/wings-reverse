use clap::Parser;
use palette::load_pcx_palette;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

use crate::font::convert_font;
use crate::level::{convert_level, Asset};
use crate::ship::convert_ship;

mod font;
mod image;
mod level;
mod palette;
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

    fs::create_dir_all(&cli.output_dir).unwrap();
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
        fs::copy(
            cli.wings_dir.join(file),
            cli.output_dir.join(file).with_extension("PCX"),
        )
        .unwrap();
    }

    let palette = load_pcx_palette(cli.wings_dir.join("COLORS.PCX"));

    if let Ok(entries) = fs::read_dir(cli.wings_dir.join("SHIPS")) {
        let ships_output = cli.output_dir.join("ships");
        fs::create_dir_all(&ships_output).unwrap();

        for entry in entries {
            let entry = entry.unwrap();
            convert_ship(
                entry.path(),
                ships_output.join(entry.file_name()).with_extension("PNG"),
                &palette,
            );
        }
    }

    if let Ok(entries) = fs::read_dir(cli.wings_dir.join("LEV")) {
        let lev_output = cli.output_dir.join("lev");
        fs::create_dir_all(&lev_output).unwrap();

        for entry in entries {
            let entry = entry.unwrap();
            let file_name = entry.file_name();
            let stem = Path::new::<OsStr>(file_name.as_ref()).file_stem().unwrap();
            let output = lev_output.join(stem);
            fs::create_dir_all(&output).unwrap();

            convert_level(entry.path(), |asset| match asset {
                Asset::Level => output.join("LEVEL.PNG"),
                Asset::Parallax => output.join("PARALLAX.PNG"),
                Asset::Info => output.join("INFO.TXT"),
            });
        }
    }
}
