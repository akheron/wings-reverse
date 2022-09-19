use nom::bytes::complete::take;
use nom::combinator::all_consuming;
use nom::multi::count;
use nom::number::complete::le_u16;
use nom::IResult;
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

pub fn load_font<T: Read>(mut input: T) -> Option<Font> {
    let mut data = Vec::new();
    input.read_to_end(&mut data).ok()?;
    let (_, font) = parse_font(&data).ok()?;
    Some(font)
}
