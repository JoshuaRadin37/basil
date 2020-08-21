use num_traits::{Num, Zero, ToPrimitive, AsPrimitive, Pow};
use std::ops::Add;
use crate::common_types::BasilResult;
use num_traits::real::Real;
use std::f32::consts::PI;
use std::cmp::Ordering;

/// Can represent some sort of coordinate in a 2D space
#[derive(Debug, Clone, PartialEq)]
pub struct Coordinate<N : Num> {
    pub x: N,
    pub y: N
}

impl<N: Num> Coordinate<N> {
    pub fn new(x: N, y: N) -> Self {
        Coordinate { x, y }
    }


}

impl <N : Num + AsPrimitive<f64>> Coordinate<N> {
    pub fn distance_to(&self, other: &Self) -> f64 {
        let x: f64 = (other.x.as_() - self.x.as_()).pow(2);
        let y: f64 = (other.y.as_() - self.y.as_()).pow(2);
        (x + y).sqrt()
    }
}

impl<N : Num> Add for Coordinate<N> {
    type Output = Coordinate<N>;

    fn add(self, rhs: Self) -> Self::Output {
        let x = self.x + rhs.x;
        let y = self.y + rhs.y;
        Coordinate {
            x, y
        }
    }
}

impl<N: Num + Default> Default for Coordinate<N> {
    fn default() -> Self {
        Self {
            x: Default::default(),
            y: Default::default()
        }
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct Line {
    pub p1: Coordinate<f32>,
    pub p2: Coordinate<f32>
}


impl Line {
    pub fn new(p1: Coordinate<f32>, p2: Coordinate<f32>) -> Self {
        Line { p1, p2 }
    }

    /// Creates a line using a starting point, a length, and an angle
    ///
    /// Angle is in radians
    pub fn new_length_angle(base: Coordinate<f32>, length: f32, angle: f32) -> Self {
        let x = angle.cos() * length;
        let y = angle.sin() * length;
        let p2 = base.clone() + Coordinate::new(x, y);
        Line {
            p1: base,
            p2
        }
    }

    /// Gets whether a line is vertical if the line were put on to a grid indexed by integrals
    pub fn is_vertical(&self) -> bool {
        (self.p2.x - self.p1.x) as u32 == 0
    }

    /// Converts a line into an equation in the form y = mx + b
    pub fn as_equation(&self) -> Box<dyn Fn(f32) -> f32> {
        let slope = (self.p2.y - self.p1.y) / (self.p2.x - self.p1.x);
        let b = self.p1.y - slope * self.p1.x;
        Box::new(move |x| slope * x + b)
    }

    pub fn closer_to_zero(&self) -> &Coordinate<f32> {
        let zero = Coordinate::default();

        if self.p1.distance_to(&zero) < self.p2.distance_to(&zero) {
            &self.p1
        } else {
            &self.p2
        }

    }

    pub fn further_from_zero(&self) -> &Coordinate<f32> {
        let zero = Coordinate::default();

        if self.p1.distance_to(&zero) < self.p2.distance_to(&zero) {
            &self.p2
        } else {
            &self.p1
        }

    }

    pub fn translate(&self, x: f32, y: f32) -> Self {
        Self {
            p1: Coordinate {  x: self.p1.x + x, y: self.p1.y + y },
            p2: Coordinate {  x: self.p2.x + x, y: self.p2.y + y }
        }
    }

    pub fn translate_by_line(&self, line: &Self) -> Self {
        let x_change = line.p2.x - line.p1.x;
        let y_change = line.p2.y - line.p1.y;
        self.translate(x_change, y_change)
    }

    pub fn as_pixel_coordinates(&self, line_width: usize) -> Vec<Coordinate<isize>> {
        let mut ret = vec![];


        let line_width_float = line_width as f32 / 2.0;
        let tangent_angle = self.angle() + PI / 2.0;
        let width_line = Line::new_length_angle(Coordinate::default(), line_width_float, tangent_angle);
        let upper_line = self.translate_by_line(&width_line);
        let lower_line = self.translate_by_line(&width_line.flip());



        let (left, right) = if self.p1.x < self.p2.x {
            (&self.p1, &self.p2)
        } else {
            (&self.p2, &self.p1)
        };
        let (low, high) = if self.p1.y < self.p2.y {
            (&self.p1, &self.p2)
        } else {
            (&self.p2, &self.p1)
        };

        let x_range = (left.x as isize - line_width as isize / 2)..=(right.x as isize + line_width as isize / 2);
        let y_range = (low.y as isize - line_width as isize / 2)..=(high.y as isize + line_width as isize / 2);
        for x in x_range {
            for y in y_range.clone() {

                let pixel_coord = Coordinate::new(x, y);
                let flt_coord = Coordinate::new(x as f32 + 0.5, y as f32 + 0.5);

                if flt_coord <= upper_line && flt_coord >= lower_line {
                    ret.push(pixel_coord);
                }
            }
        }

        ret
    }

    pub fn angle(&self) -> f32 {
        let delta_x = self.p2.x - self.p1.x;
        let delta_y = self.p2.y - self.p1.y;

        (delta_y / delta_x).atan()
    }

    pub fn flip(&self) -> Self {
        Self {
            p1: self.p2.clone(),
            p2: self.p1.clone()
        }
    }
}

impl PartialEq<Line> for Coordinate<f32> {
    fn eq(&self, _: &Line) -> bool {
        return false;
    }
}

impl PartialOrd<Line> for Coordinate<f32> {
    fn partial_cmp(&self, other: &Line) -> Option<Ordering> {
        if other.is_vertical() {
            return None;
        }

        let eq = other.as_equation();
        let eq_result = eq(self.x);
        self.y.partial_cmp(&eq_result)
    }
}


pub trait Shape {

    fn get_lines(&self) -> Vec<Line>;
}


#[cfg(test)]
mod test {
    use crate::internal_structures::shapes::{Line, Coordinate};

    #[test]
    fn eq() {
        let line = Line::new(Coordinate::new(0.0, 2.0), Coordinate::new(3.0, 6.5));
        let equation = line.as_equation();
        assert_eq!(equation(10.0), 17.0);
    }

    #[test]
    fn line_pixels() {
        let line = Line::new(Coordinate::new(0.5, 0.5), Coordinate::new(3.5, 1.5));
        let pixels = line.as_pixel_coordinates(1);
        println!("{:?}", pixels);
    }
}

