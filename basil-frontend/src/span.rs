use std::path::PathBuf;
use std::ops::{Range, Deref};
use std::fs::File;
use std::io::{Seek, SeekFrom, ErrorKind, Read};
use std::cell::{Cell, Ref, RefCell};
use std::borrow::Borrow;

/// A span represents a place in a file so we don't have to individually save every part of a file
/// repeatedly
#[derive(Debug)]
pub struct Span {
    file: PathBuf,
    characters: Range<u64>,
    cache: RefCell<String>
}

impl Span {

    /// Creates a new space
    pub fn new(file: PathBuf, characters: Range<u64>) -> Self {
        Span { file, characters, cache: RefCell::new(String::new()) }
    }

    /// Creates a new span inside an old span.
    ///
    /// Returns None if the range in the sub space isn't entirely in the range of the outer span, or
    /// if the ranges are equal.
    pub fn sub_span(&self, sub_characters: Range<u64>) -> Option<Self> {
        if sub_characters == self.characters ||
            !(self.characters.contains(&sub_characters.start) && self.characters.contains(&sub_characters.end))
            {
            return None;
        }

        Some(Self {
            file: self.file.clone(),
            characters: sub_characters,
            cache: RefCell::new(String::new())
        })
    }

    /// Gets the associated string from this span
    pub fn get_string(&self) -> std::io::Result<Ref<String>> {
        let mut file = File::open(&self.file)?;
        if file.seek(SeekFrom::Start(self.characters.start))? == self.characters.start {
            let count = self.characters.clone().count();
            let mut buffer = vec![0u8; count];
            let read = file.read(&mut buffer)?;
            if read < count {
                Err(std::io::Error::new(ErrorKind::InvalidData, "Range out of bounds"))
            } else {
                let found_string =
                    String::from_utf8(buffer)
                        .map_err(|e| std::io::Error::new(ErrorKind::Other, e))?;
                *self.cache.borrow_mut() = found_string;
                Ok(self.cache.borrow())
            }
        } else {
            Err(std::io::Error::new(ErrorKind::InvalidData, "Range out of bounds"))
        }
    }

}

