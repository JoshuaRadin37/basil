use std::borrow::Borrow;
use std::error::Error;
use std::fmt::Debug;

/// Represents an immutable dimensions of an object, can be seen as a rectangle
#[derive(Debug, PartialEq)]
pub struct Dimension {
    width: usize,
    height: usize
}

impl Dimension {
    /// Creates a new Dimensions object with width and height
    pub fn new(width: usize, height: usize) -> Self {
        Dimension { width, height }
    }

    /// Gets the area of the rectangle represented by this object
    pub fn get_area(&self) -> usize {
        self.height * self.width
    }

    /// Gets the width
    pub fn width(&self) -> usize {
        self.width
    }

    /// Gets the height
    pub fn height(&self) -> usize {
        self.height
    }

    /// Creates a new Dimensions object, where the width and height are multiplied by width_change and height change, respectively.
    ///
    /// Rounding of the new dimension follows standard floating point rounding requirements
    /// #Example
    ///```
    /// use basil_core::common_types::Dimension;
    /// let dim = Dimension::new(100, 100);
    /// let dim2 = dim.resize(2.0, 2.0);
    /// assert_eq!(dim2.width(), 200);
    /// assert_eq!(dim2.height(), 200);
    ///```
    pub fn resize(&self, width_change: f32, height_change: f32) -> Self {
        Self::new((self.width as f32 * width_change) as usize, (self.height as f32 * height_change) as usize)
    }

    /// Alters a Dimensions object, where the width and height are multiplied by width_change and height change, respectively.
    ///
    /// Rounding of the new dimension follows standard floating point rounding requirements
    /// #Example
    ///```
    /// use basil_core::common_types::Dimension;
    /// let dim = Dimension::new(100, 100);
    /// let dim2 = dim.into_resize(2.0, 2.0);
    /// assert_eq!(dim2.width(), 200);
    /// assert_eq!(dim2.height(), 200);
    ///```
    pub fn into_resize(self, width_change: f32, height_change: f32) -> Self {
        self.resize(width_change, height_change)
    }
}

#[derive(Debug)]
pub enum BasilError {
    InnerError(Box<dyn Error>),
    CloseOperationFailed,
    HexCodeParseError,
    InvalidHexCode,
    DrawError
}

impl<E: 'static + Error> From<E> for BasilError {
    fn from(e: E) -> Self {
        Self::InnerError(Box::new(e))
    }
}



pub type BasilResult<T = ()> = std::result::Result<T, BasilError>;
