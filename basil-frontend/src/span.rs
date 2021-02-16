use std::borrow::Borrow;
use std::cell::{Cell, Ref, RefCell};
use std::fmt::{Debug, Formatter};
use std::fs::File;
use std::io::{ErrorKind, Read, Seek, SeekFrom};
use std::ops::{Deref, Range};
use std::path::PathBuf;

/// A span represents a place in a file so we don't have to individually save every part of a file
/// repeatedly
#[derive(Debug, Clone)]
pub struct Span {
    file: PathBuf,
    characters: Range<u64>,
    cache: RefCell<String>,
}

impl Span {
    /// Creates a new space
    pub fn new(file: PathBuf, characters: Range<u64>) -> Self {
        Span {
            file,
            characters,
            cache: RefCell::new(String::new()),
        }
    }

    /// Creates a new span inside an old span.
    ///
    /// Returns None if the range in the sub space isn't entirely in the range of the outer span, or
    /// if the ranges are equal.
    pub fn sub_span(&self, sub_characters: Range<u64>) -> Option<Self> {
        if sub_characters == self.characters
            || !(self.characters.contains(&sub_characters.start)
                && self.characters.contains(&sub_characters.end))
        {
            return None;
        }

        Some(Self {
            file: self.file.clone(),
            characters: sub_characters,
            cache: RefCell::new(String::new()),
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
                Err(std::io::Error::new(
                    ErrorKind::InvalidData,
                    "Range out of bounds",
                ))
            } else {
                let found_string = String::from_utf8(buffer)
                    .map_err(|e| std::io::Error::new(ErrorKind::Other, e))?;
                *self.cache.borrow_mut() = found_string;
                Ok(self.cache.borrow())
            }
        } else {
            Err(std::io::Error::new(
                ErrorKind::InvalidData,
                "Range out of bounds",
            ))
        }
    }

    pub fn empty() -> Span {
        Self {
            file: PathBuf::from("empty"),
            characters: 0..1,
            cache: RefCell::new("".to_string()),
        }
    }
}

#[derive(Clone)]
pub struct WithSpan<T>(T, Span);

impl<T> WithSpan<T> {
    pub fn new(ty: T, span: Span) -> Self {
        WithSpan(ty, span)
    }

    pub fn empty(ty: T) -> Self {
        WithSpan(ty, Span::empty())
    }

    pub fn get_object(&self) -> &T {
        &self.0
    }

    pub fn get_span(&self) -> &Span {
        &self.1
    }
}

impl<T: Debug> Debug for WithSpan<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
