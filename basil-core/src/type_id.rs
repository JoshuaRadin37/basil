#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum TypeId {
    /// For primitives
    Implicit(Implicit),
    /// For objects
    Explicit(Explicit),
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Implicit(u64);

impl Implicit {
    pub(crate) fn new(id: u64) -> Self {
        Implicit(id)
    }
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Explicit(pub u64);

impl Explicit {
    pub fn new(id: u64) -> Self {
        Explicit(id)
    }
}

impl From<Implicit> for TypeId {
    fn from(i: Implicit) -> Self {
        TypeId::Implicit(i)
    }
}

impl From<Explicit> for TypeId {
    fn from(e: Explicit) -> Self {
        TypeId::Explicit(e)
    }
}
