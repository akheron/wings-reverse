use crate::palette::Palette;
use image::RgbImage;
use nom::bytes::complete::take;
use nom::combinator::map;
use nom::number::complete::{le_u16, le_u32};
use nom::sequence::tuple;
use nom::IResult;

#[derive(Debug)]
pub struct Image {
    pub width: u16,
    pub height: u16,
    pub pixels: Vec<u8>,
}

impl Image {
    pub fn parse(start_input: &[u8]) -> IResult<&[u8], Image> {
        let (input, (width, height, data_len)) = tuple((le_u16, le_u16, le_u32))(start_input)?;
        let (input, pixels) = map(take(data_len), rle_decode)(input)?;
        if pixels.len() != (width as usize) * (height as usize) {
            Err(nom::Err::Error(nom::error::Error::new(
                start_input,
                nom::error::ErrorKind::Verify,
            )))
        } else {
            Ok((
                input,
                Image {
                    width,
                    height,
                    pixels,
                },
            ))
        }
    }

    pub fn to_rgb_image(&self, palette: &Palette) -> RgbImage {
        let width: u32 = self.width.into();
        let height: u32 = self.height.into();
        let mut image = RgbImage::new(width, height);

        for y in 0..height {
            for x in 0..width {
                let index: usize = (y * width + x).try_into().unwrap();
                let pixel = palette.get(self.pixels[index]);
                image.put_pixel(x, y, pixel);
            }
        }
        image
    }
}

fn rle_decode(data: &[u8]) -> Vec<u8> {
    let mut result = Vec::new();
    let mut index = 0;
    while index < data.len() {
        let byte1 = data[index];
        index += 1;

        if byte1 & 0b11000000 == 0b11000000 {
            let run_length = byte1 & 0b00111111;
            let byte2 = data[index];
            index += 1;

            for _ in 0..run_length {
                result.push(byte2);
            }
        } else {
            result.push(byte1);
        }
    }
    result
}
