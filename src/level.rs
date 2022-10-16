use crate::image::Image;
use crate::palette::{parse_palette, Palette};
use nom::number::complete::{le_u16, u8};
use nom::sequence::tuple;
use nom::IResult;
use std::ffi::OsStr;
use std::fs;
use std::fs::File;
use std::io::Read;

pub struct Level {
    pub palette: Palette,
    pub level: Image,
    pub parallax: Option<Image>,
    pub show_stars: bool,
    pub rain_probability: u32,
    pub snow_probability: u32,
    pub bombing_probability: u32,
    pub num_civilians: u32,
    pub armed_civilians_percentage: u32,
}

fn parse_level(input: &[u8]) -> IResult<&[u8], Level> {
    // The palette has been scaled down to 6 bits per color
    let (input, palette) = parse_palette(true)(input)?;

    let (input, level) = Image::parse(input)?;

    let (input, is_parallax) = u8(input)?;
    let (input, parallax) = if is_parallax == 1 {
        let (input, parallax) = Image::parse(input)?;
        (input, Some(parallax))
    } else {
        (input, None)
    };

    let (input, show_stars) = u8(input)?;
    let (input, use_defaults) = le_u16(input)?;
    let (
        input,
        (
            rain_probability,
            snow_probability,
            bombing_probability,
            num_civilians,
            armed_civilians_percentage,
        ),
    ) = if use_defaults == 2 {
        // Do not use defaults
        tuple((le_u16, le_u16, le_u16, le_u16, le_u16))(input)?
    } else {
        (input, (0, 0, 4, 50, 100))
    };

    Ok((
        input,
        Level {
            palette,
            level,
            parallax,
            show_stars: show_stars == 1,
            rain_probability: rain_probability as u32,
            snow_probability: snow_probability as u32,
            bombing_probability: bombing_probability as u32,
            num_civilians: num_civilians as u32,
            armed_civilians_percentage: armed_civilians_percentage as u32,
        },
    ))
}

fn load_level<T: Read>(mut input: T) -> Option<Level> {
    let mut data = Vec::new();
    input.read_to_end(&mut data).ok()?;
    let (_, level) = parse_level(&data).ok()?;
    Some(level)
}

pub enum Asset {
    Level,
    Parallax,
    Info,
}

pub fn convert_level<I: AsRef<OsStr>, O: AsRef<OsStr>, F: Fn(Asset) -> O>(
    input_path: I,
    output_path: F,
) {
    let mut file = File::open(input_path.as_ref()).unwrap();
    let level = load_level(&mut file).unwrap();
    let level_img_path = output_path(Asset::Level);
    level
        .level
        .to_rgb_image(&level.palette)
        .save(level_img_path.as_ref())
        .unwrap();
    if let Some(parallax) = level.parallax {
        let parallax_img_path = output_path(Asset::Parallax);
        parallax
            .to_rgb_image(&level.palette)
            .save(parallax_img_path.as_ref())
            .unwrap();
    }

    let info_path = output_path(Asset::Info);

    // write textual info about level to file named by info_path
    fs::write(
        info_path.as_ref(),
        format!(
            "\
show_stars: {}
rain_probability: {}
snow_probability: {}
bombing_probability: {}
num_civilians: {}
armed_civilians_percentage: {}
",
            level.show_stars,
            level.rain_probability,
            level.snow_probability,
            level.bombing_probability,
            level.num_civilians,
            level.armed_civilians_percentage
        ),
    )
    .unwrap();
}
