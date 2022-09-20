use image::{Rgb, RgbImage};
use nom::bytes::complete::take;
use nom::combinator::all_consuming;
use nom::multi::count;
use nom::number::complete::le_u16;
use nom::IResult;
use std::ffi::OsStr;
use std::fs::File;
use std::io::Read;

#[derive(Debug)]
pub struct Font {
    pub glyph_width: u32,
    pub glyph_height: u32,
    glyphs: [Glyph; 256],
}

impl Font {
    pub fn num_glyphs(&self) -> usize {
        256
    }

    pub fn glyph_pixel(&self, glyph: usize, x: u32, y: u32) -> Option<u8> {
        if glyph < self.num_glyphs() && x < self.glyph_width && y < self.glyph_height {
            let pixels: &[u8] = &self.glyphs[glyph].pixels;
            let pixel_index: usize = (x * self.glyph_width + y).try_into().unwrap();
            Some(pixels[pixel_index])
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct Glyph {
    pub width: u16,
    pub height: u16,
    pub pixels: Vec<u8>,
}

fn parse_font(start_input: &[u8]) -> IResult<&[u8], Font> {
    let (input, glyphs) = all_consuming(count(parse_glyph, 256))(start_input)?;

    let glyph_width = glyphs[0].width;
    let glyph_height = glyphs[0].height;
    for glyph in glyphs.iter() {
        if glyph.width != glyph_width || glyph.height != glyph_height {
            return Err(nom::Err::Error(nom::error::Error::new(
                start_input,
                nom::error::ErrorKind::Verify,
            )));
        };
    }

    Ok((
        input,
        Font {
            glyph_width: glyph_width.into(),
            glyph_height: glyph_height.into(),
            glyphs: glyphs.try_into().unwrap(),
        },
    ))
}

fn parse_glyph(input: &[u8]) -> IResult<&[u8], Glyph> {
    let (input, width) = le_u16(input)?;
    let (input, height) = le_u16(input)?;
    let (input, pixels) = take(width * height)(input)?;
    Ok((
        input,
        Glyph {
            width,
            height,
            pixels: pixels.into(),
        },
    ))
}

fn load_font<T: Read>(mut input: T) -> Option<Font> {
    let mut data = Vec::new();
    input.read_to_end(&mut data).ok()?;
    let (_, font) = parse_font(&data).ok()?;
    Some(font)
}

fn generate_image(font: &Font) -> RgbImage {
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

pub fn convert_font<I: AsRef<OsStr>, O: AsRef<OsStr>>(input_path: I, output_path: O) {
    let mut file = File::open(input_path.as_ref()).unwrap();
    let font = load_font(&mut file).unwrap();
    let image = generate_image(&font);
    image.save(output_path.as_ref()).unwrap();
}
