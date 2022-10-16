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
        let Rgb(r, g, b) = self.0[color_index as usize];
        image::Rgb([r, g, b])
    }
}

pub fn parse_palette<'a>(to_8bit: bool) -> impl FnMut(&'a [u8]) -> IResult<&'a [u8], Palette> {
    return map(count(parse_rgb(to_8bit), 256), |rgbs| {
        Palette(rgbs.try_into().unwrap())
    });
}

fn parse_rgb<'a>(to_8bit: bool) -> impl FnMut(&'a [u8]) -> IResult<&'a [u8], Rgb> {
    map(tuple((u8, u8, u8)), move |(r, g, b)| {
        Rgb(
            if to_8bit { r * 4 } else { r },
            if to_8bit { g * 4 } else { g },
            if to_8bit { b * 4 } else { b },
        )
    })
}

pub fn load_pcx_palette<I: AsRef<OsStr>>(path: I) -> Palette {
    let mut file = File::open(path.as_ref()).unwrap();
    file.seek(std::io::SeekFrom::End(-768)).unwrap();

    let mut data = Vec::new();
    file.read_to_end(&mut data).unwrap();

    let (_, palette) = all_consuming(parse_palette(false))(&data).unwrap();
    palette
}
