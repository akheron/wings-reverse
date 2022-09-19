use clap::Parser;
use image::{Rgb, RgbImage};

use crate::font::{load_font, Font};
use std::fs::File;
use std::path::PathBuf;

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

    let mut file = File::open(cli.wings_dir.join("VGAFONT1.PIC").as_os_str()).unwrap();
    let font = load_font(&mut file).unwrap();
    let image = generate_font_image(&font);

    std::fs::create_dir_all(&cli.output_dir).unwrap();
    image
        .save(cli.output_dir.join("VGAFONT1.PNG").as_os_str())
        .unwrap();
}

fn generate_font_image(font: &Font) -> RgbImage {
    assert_eq!(
        font.num_glyphs(),
        16 * 16,
        "assumed 16 * 16 = 256 font glyphs"
    );

    let glyph_width = font.glyph_width;
    let glyph_height = font.glyph_height;
    let mut image = RgbImage::new(glyph_width * 16, glyph_height * 16);

    // 16 * 16 = 256
    let mut glyph_index: usize = 0;
    for grid_x in 0..16u32 {
        for grid_y in 0..16u32 {
            let glyph_origo_y = grid_y * glyph_height;
            let glyph_origo_x = grid_x * glyph_width;
            for x in 0..glyph_width {
                for y in 0..glyph_height {
                    let image_x = glyph_origo_x + x;
                    let image_y = glyph_origo_y + y;
                    let pixel: u8 = font.glyph_pixel(glyph_index, x, y).unwrap();
                    image.put_pixel(image_y, image_x, Rgb([pixel, pixel, pixel]));
                }
            }
            glyph_index += 1;
        }
    }

    image
}
