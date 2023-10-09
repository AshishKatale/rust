use std::{fmt::Display, num::ParseIntError, u8};

pub struct Hex(String);
pub struct RGB(u8, u8, u8);

impl RGB {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        RGB(r, g, b)
    }
}

impl Hex {
    pub fn new(str: &str) -> Hex {
        Hex(str.to_owned())
    }
}

impl Display for RGB {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "rgb({}, {}, {})", self.0, self.1, self.2)
    }
}

impl Display for Hex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<RGB> for Hex {
    fn from(rgb: RGB) -> Self {
        Hex(format!("#{:<02X}{:<02X}{:<02X}", rgb.0, rgb.1, rgb.2))
    }
}

impl TryFrom<Hex> for RGB {
    type Error = String;

    fn try_from(hex: Hex) -> Result<Self, Self::Error> {
        if hex.0.len() != 7 {
            return Err(String::from("hex str must be in #xxxxxx format"));
        }

        let to_str_error = |e: ParseIntError| e.to_string();

        let r = u8::from_str_radix(&hex.0[1..3], 16).map_err(to_str_error)?;
        let g = u8::from_str_radix(&hex.0[3..5], 16).map_err(to_str_error)?;
        let b = u8::from_str_radix(&hex.0[5..7], 16).map_err(to_str_error)?;

        Ok(RGB(r, g, b))
    }
}

#[cfg(test)]
mod test {
    use super::{Hex, RGB};

    #[test]
    fn hex_to_rgb() {
        let h = Hex::new("#AABBCC");
        let r: Result<RGB, _> = h.try_into();
        if let Ok(rgb) = r {
            assert_eq!(rgb.to_string(), "rgb(170, 187, 204)");
        }

        let h = Hex::new("#AABCC");
        let r: Result<RGB, _> = h.try_into();
        if let Err(e) = r {
            assert_eq!(e.to_string(), "hex str must be in #xxxxxx format");
        }

        let h = Hex::new("#AAxBCC");
        let r: Result<RGB, _> = h.try_into();
        if let Err(rgb) = r {
            assert_eq!(rgb.to_string(), "invalid digit found in string");
        }
    }

    #[test]
    fn rgb_to_hex() {
        let r = RGB::new(01, 101, 201);
        let h: Hex = r.into();
        assert_eq!(h.to_string(), "#0165C9");
    }
}
