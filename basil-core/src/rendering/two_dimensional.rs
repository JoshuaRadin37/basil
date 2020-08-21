use crate::rendering::colors::RGBAColor;
use std::num::NonZeroU32;
use crate::common_types::BasilResult;
use crate::internal_structures::shapes::{Coordinate, Line};

pub trait Graphics2D {

    fn set_color(&mut self, color: RGBAColor);
    fn set_stroke_size(&mut self, size: NonZeroU32);

    fn draw_pixel(&self, pixel: Coordinate<usize>) -> BasilResult;
    fn draw_line(&self, base: Coordinate<usize>, line: Line) -> BasilResult {
        if line.is_vertical() {

        } else {

        }
        Ok(())
    }
}