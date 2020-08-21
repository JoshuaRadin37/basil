use crate::rendering::colors::RGBAColor;
use std::num::NonZeroU32;

pub struct Stroke {
    color: RGBAColor,
    stroke_size: NonZeroU32
}

impl Stroke {
    pub fn new(color: RGBAColor, stroke_size: NonZeroU32) -> Self {
        Stroke { color, stroke_size }
    }

    pub fn color(&self) -> &RGBAColor {
        &self.color
    }

    pub fn color_mut(&mut self) -> &mut RGBAColor {
        &mut self.color
    }

    pub fn stroke_size(&self) -> &NonZeroU32 {
        &self.stroke_size
    }

    pub fn stroke_size_mut(&mut self) -> &mut NonZeroU32 {
        &mut self.stroke_size
    }
}

impl Default for Stroke {
    fn default() -> Self {
        Self::new(Default::default(), NonZeroU32::new(1).unwrap())
    }
}

