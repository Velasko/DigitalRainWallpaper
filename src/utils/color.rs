use regex::Regex;
use std::str::FromStr;

extern crate strum;
extern crate strum_macros;

use strum_macros::EnumString;

#[derive(Debug, Clone, Copy)]
pub struct Color {
    values: [u8; 4],
}

#[derive(Debug, PartialEq, EnumString)]
#[strum(serialize_all = "kebab-case")]
pub enum ColorName {
    Black,
    White,
    Red,
    Green,
    Blue,
}

impl From<ColorName> for Color {
    fn from(name: ColorName) -> Self {
        Color {
            values: match name {
                ColorName::Black => [0, 0, 0, 255],
                ColorName::White => [255, 255, 255, 255],
                ColorName::Red => [255, 0, 0, 255],
                ColorName::Green => [0, 255, 0, 255],
                ColorName::Blue => [0, 0, 255, 255],
            },
        }
    }
}

impl std::str::FromStr for Color {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match ColorName::from_str(s) {
            Ok(color) => Color::from(color),
            Err(_) => {
                let re = Regex::new(
                    r"^(?:\(([0-9]{1,3}),([0-9]{1,3}),([0-9]{1,3})(?:,([0-9]{1,3}))?\)|#([0-9A-F]{2})([0-9A-F]{2})([0-9A-F]{2})([0-9A-F]{2})?)$",
                )
                .unwrap();

                println!("{:?}", re.captures(s));

                match re.captures(s) {
                    None => return Err("Value is not reckognized as a color value"),
                    Some(capture) => {
                        let mut extract = capture.iter();
                        let complete = extract.next().unwrap().unwrap().as_str();
                        let base = if complete.starts_with("#") { 16 } else { 10 };
                        let mut color_values = extract
                            .filter(|value| value.is_some())
                            .map(|value| {
                                u8::from_str_radix(value.unwrap().as_str(), base)
                                    .expect("All color values should be lower than 255 base 10")
                            })
                            .collect::<Vec<u8>>();
                        if color_values.len() == 3 {
                            color_values.push(255);
                        }

                        Color {
                            values: color_values.try_into().expect(""),
                        }
                    }
                }
            }
        })
    }
}
