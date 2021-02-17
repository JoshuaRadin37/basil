use std::borrow::Borrow;
use std::cell::{Cell, Ref, RefCell};
use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind, Read, Seek, SeekFrom};
use std::ops::{Deref, Range};
use std::path::PathBuf;

/// A span represents a place in a file so we don't have to individually save every part of a file
/// repeatedly
#[derive(Debug, Clone)]
pub struct Span {
    file: PathBuf,
    start: LineColumn,
    end: LineColumn,
    cache: RefCell<Option<String>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LineColumn {
    pub line: usize,
    pub column: usize,
}

impl LineColumn {
    pub fn new(line: usize, column: usize) -> Self {
        LineColumn { line, column }
    }
}

impl Ord for LineColumn {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.line.cmp(&other.line) {
            Ordering::Less => Ordering::Less,
            Ordering::Equal => self.column.cmp(&other.column),
            Ordering::Greater => Ordering::Greater,
        }
    }
}

impl PartialOrd for LineColumn {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Span {
    /// Creates a new space
    pub fn new(file: PathBuf, start: LineColumn, end: LineColumn) -> Self {
        Span {
            file,
            start,
            end,
            cache: Default::default(),
        }
    }

    /// Creates a new span inside an old span.
    ///
    /// Returns None if the range in the sub space isn't entirely in the range of the outer span, or
    /// if the ranges are equal.
    pub fn sub_span(&self, start: LineColumn, end: LineColumn) -> Option<Self> {
        if end < start {
            panic!("Can't create a span where the end is before the start");
        }
        if (self.start == start && self.end == end) || !(start >= self.start && end <= self.end) {
            return None;
        }

        Some(Self {
            file: self.file.clone(),
            start,
            end,
            cache: Default::default(),
        })
    }

    /// Gets the associated string from this span
    pub fn get_string(&self) -> std::io::Result<String> {
        if self.cache.borrow().is_none() {
            let mut file = File::open(&self.file)?;
            let mut buf_reader = BufReader::new(file);
            for _ in 1..self.start.line {
                let mut _buf = String::new();
                buf_reader.read_line(&mut _buf);
            }

            buf_reader.seek(SeekFrom::Current(self.start.column as i64))?;
            let mut ret = String::new();
            let mut current = self.start;
            while current <= self.end {
                let mut line = String::new();
                buf_reader.read_line(&mut line)?;
                for char in line.chars() {
                    ret.push(char);
                    if char == '\n' {
                        current.line += 1;
                        current.column = 1;
                    } else {
                        current.column += 1;
                    }
                }
            }

            *self.cache.borrow_mut() = Some(ret);
        }

        let borrow = self.cache.borrow();
        Ok(borrow.as_ref().unwrap().clone())
    }

    pub fn empty() -> Span {
        Self {
            file: PathBuf::from("none"),
            start: LineColumn::new(1, 0),
            end: LineColumn::new(1, 0),
            cache: Default::default(),
        }
    }
    pub fn file(&self) -> &PathBuf {
        &self.file
    }
    pub fn start(&self) -> LineColumn {
        self.start
    }
    pub fn end(&self) -> LineColumn {
        self.end
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
