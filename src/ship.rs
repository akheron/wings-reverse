use image::RgbImage;
use nom::bytes::complete::take;
use nom::combinator::{all_consuming, map};
use nom::multi::count;
use nom::number::complete::{le_u16, le_u32};
use nom::sequence::tuple;
use nom::IResult;
use std::ffi::OsStr;
use std::fs::File;
use std::io::Read;

use crate::palette::Palette;

#[derive(Debug)]
struct Ship {
    name: String,
    properties: [u32; 7],
    images: [Image; 72],
}

#[derive(Debug)]
struct Image {
    width: u16,
    height: u16,
    pixels: Vec<u8>,
}

fn parse_ship(start_input: &[u8]) -> IResult<&[u8], Ship> {
    let (input, magic) = take(4u32)(start_input)?;
    if magic != b"WSHP" {
        return Err(nom::Err::Error(nom::error::Error::new(
            start_input,
            nom::error::ErrorKind::Verify,
        )));
    }

    let (input, _) = take(4u32)(input)?;
    let (input, name_len) = le_u32(input)?;
    let (input, name_bytes) = take(name_len)(input)?;
    let name = String::from_utf8_lossy(name_bytes);

    let (input, properties) = count(le_u32, 7)(input)?;

    let (input, images) = all_consuming(count(parse_and_decode_image, 72))(input)?;

    // Make sure all images have the same dimensions
    let width = images[0].width;
    let height = images[0].height;
    for image in &images {
        if image.width != width || image.height != height {
            return Err(nom::Err::Error(nom::error::Error::new(
                start_input,
                nom::error::ErrorKind::Verify,
            )));
        }
    }

    Ok((
        input,
        Ship {
            name: name.to_string(),
            properties: properties.try_into().unwrap(),
            images: images.try_into().unwrap(),
        },
    ))
}

fn parse_and_decode_image(start_input: &[u8]) -> IResult<&[u8], Image> {
    let (input, (width, height, data_len)) = tuple((le_u16, le_u16, le_u32))(start_input)?;
    let (input, pixels) = map(take(data_len), rle_decode)(input)?;
    if pixels.len() != (width * height).into() {
        return Err(nom::Err::Error(nom::error::Error::new(
            start_input,
            nom::error::ErrorKind::Verify,
        )));
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

fn load_ship<T: Read>(mut input: T) -> Option<Ship> {
    let mut data = Vec::new();
    input.read_to_end(&mut data).ok()?;
    let (_, ship) = parse_ship(&data).ok()?;
    Some(ship)
}

fn generate_image(ship: &Ship, palette: &Palette) -> RgbImage {
    let width: u32 = ship.images[0].width.into();
    let height: u32 = ship.images[0].height.into();
    let mut image = RgbImage::new(width * 72, height);

    let mut x_offset = 0;
    for im in &ship.images {
        for y in 0..height {
            for x in 0..width {
                let index: usize = (y * width + x).try_into().unwrap();
                let pixel = palette.get(im.pixels[index]);
                image.put_pixel(x + x_offset, y, pixel);
            }
        }
        x_offset += width;
    }

    image
}

pub fn convert_ship<I: AsRef<OsStr>, O: AsRef<OsStr>>(
    input_path: I,
    output_path: O,
    palette: &Palette,
) {
    let mut file = File::open(input_path.as_ref()).unwrap();
    let ship = load_ship(&mut file).unwrap();
    let image = generate_image(&ship, palette);
    image.save(output_path.as_ref()).unwrap();
}
