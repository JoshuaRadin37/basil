use crate::common_types::{BasilResult, BasilError};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RGBColor {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

impl RGBColor {

    pub fn from_hex_code<S : AsRef<str>>(s: S) -> BasilResult<Self> {
        let string = s.as_ref();
        if let Some('#') = string.chars().nth(0) {
            let hex_str = &string[1..];
            let hex: u32 = u32::from_str_radix(hex_str, 16)?;
            if hex <= 0xFFFFFF {
                Ok(RGBColor::from(hex))
            } else {
                Err(BasilError::InvalidHexCode)
            }
        } else {
            Err(BasilError::HexCodeParseError)
        }
    }

    pub fn from_integer(i: u32) -> BasilResult<Self> {
        if i <= 0xFFFFFF {
            Ok(RGBColor::from(i))
        } else {
            Err(BasilError::InvalidHexCode)
        }
    }

    pub const unsafe fn from_integer_unchecked(hex: u32) -> Self {
        let r = ((hex >> 16) & 0xFF) as u8;
        let g = ((hex >> 8) & 0xFF) as u8;
        let b = (hex & 0xFF) as u8;
        RGBColor {
            r,
            g,
            b
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RGBAColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8
}

impl From<RGBColor> for RGBAColor {
    fn from(color: RGBColor) -> Self {
        let RGBColor { r, g, b} = color;
        RGBAColor {
            r,
            g,
            b,
            a: 0
        }
    }
}

impl From<u32> for RGBColor {
    fn from(hex: u32) -> Self {
        let r = ((hex >> 16) & 0xFF) as u8;
        let g = ((hex >> 8) & 0xFF) as u8;
        let b = (hex & 0xFF) as u8;
        RGBColor {
            r,
            g,
            b
        }
    }
}

impl Default for RGBColor {
    fn default() -> Self {
        BLACK.clone()
    }
}

impl Default for RGBAColor {
    fn default() -> Self {
        BLACK.clone().into()
    }
}

pub static AQUA: RGBColor = unsafe { RGBColor::from_integer_unchecked(0x00FFFF) };
pub static BLACK: RGBColor = unsafe { RGBColor::from_integer_unchecked(0) };
pub static BLUE: RGBColor = unsafe { RGBColor::from_integer_unchecked(0x0000FF) };
pub static FUCHSIA: RGBColor = unsafe { RGBColor::from_integer_unchecked(0xFF00FF) };
pub static GRAY: RGBColor = unsafe { RGBColor::from_integer_unchecked(0x808080) };
pub static GREEN: RGBColor = unsafe { RGBColor::from_integer_unchecked(0x008000) };
pub static LIME: RGBColor = unsafe { RGBColor::from_integer_unchecked(0x00FF00) };
pub static MAROON: RGBColor = unsafe { RGBColor::from_integer_unchecked(0x800000) };
pub static NAVY: RGBColor = unsafe { RGBColor::from_integer_unchecked(0x000080) };
pub static OLIVE: RGBColor = unsafe { RGBColor::from_integer_unchecked(0x808000) };
pub static PURPLE: RGBColor = unsafe { RGBColor::from_integer_unchecked(0x800080) };
pub static RED: RGBColor = unsafe { RGBColor::from_integer_unchecked(0xFF0000) };
pub static SILVER: RGBColor = unsafe { RGBColor::from_integer_unchecked(0xC0C0C0) };
pub static TEAL: RGBColor = unsafe { RGBColor::from_integer_unchecked(0x008080) };
pub static WHITE: RGBColor = unsafe { RGBColor::from_integer_unchecked(0xFFFFFF) };
pub static YELLOW: RGBColor = unsafe { RGBColor::from_integer_unchecked(0xFFF000) };

#[cfg(test)]
mod test {
    use crate::rendering::colors::{RGBColor, OLIVE};

    #[test]
    fn parse_strings() {
        let color = RGBColor::from_hex_code("#808000").unwrap();
        assert_eq!(color, OLIVE);
        RGBColor::from_hex_code("#90009009").unwrap_err();
        RGBColor::from_hex_code("bazimba").unwrap_err();
    }

    #[test]
    fn valid_hex() {
        RGBColor::from_integer(0xFF00FF).unwrap();
        RGBColor::from_integer(0xFF00FF1).unwrap_err();
    }
}

