use std::convert::TryInto;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{Read, Seek};

use nom::combinator::{all_consuming, map};
use nom::multi::count;
use nom::number::complete::u8;
use nom::sequence::tuple;
use nom::IResult;

#[derive(Debug)]
struct Rgb(u8, u8, u8);

#[derive(Debug)]
pub struct Palette([Rgb; 256]);

impl Palette {
    pub fn get(&self, color_index: u8) -> image::Rgb<u8> {
        let index: usize = color_index.into();
        let Rgb(r, g, b) = self.0[index];
        image::Rgb([r, g, b])
    }
}

fn parse_palette(input: &[u8]) -> IResult<&[u8], Palette> {
    map(count(parse_rgb, 256), |rgbs| {
        Palette(rgbs.try_into().unwrap())
    })(input)
}

fn parse_rgb(input: &[u8]) -> IResult<&[u8], Rgb> {
    map(tuple((u8, u8, u8)), |(r, g, b)| Rgb(r, g, b))(input)
}

pub fn load_palette<I: AsRef<OsStr>>(path: I) -> Palette {
    let mut file = File::open(path.as_ref()).unwrap();
    file.seek(std::io::SeekFrom::End(-768)).unwrap();

    let mut data = Vec::new();
    file.read_to_end(&mut data).unwrap();

    let (_, palette) = all_consuming(parse_palette)(&data).unwrap();
    palette
}
