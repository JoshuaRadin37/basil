use std::cell::{Ref, RefMut};
use std::ops::{Deref, DerefMut};

/*
pub trait RefChain {
    type Target;

    fn new<I: IntoIterator<Item = Self::Target>>(iter: I) -> Self;
    fn push(&mut self, reference: Self::Target);
    fn peek(&self) -> Option<Self::Target>;
}

pub struct ReferenceChain<'a, T> {
    buffer: Vec<Ref<'a, T>>,
}

impl<'a, T> RefChain for ReferenceChain<'a, T> {
    type Target = Ref<'a, T>;

    fn new<I: IntoIterator<Item = Ref<'a, T>>>(buffer: I) -> Self {
        ReferenceChain {
            buffer: buffer.into_iter().collect(),
        }
    }

    fn push(&mut self, reference: Ref<'a, T>) {
        self.buffer.push(reference);
    }

    fn peek(&self) -> Option<Ref<'a, T>> {
        self.buffer.last().map(|d| clone())
    }
}

impl<'a, T> Deref for ReferenceChain<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &*self.peek().unwrap().deref()
    }
}

pub struct MutReferenceChain<'a, T> {
    buffer: Vec<RefMut<'a, T>>,
}

impl<'a, T> RefChain for MutReferenceChain<'a, T> {
    type Target = RefMut<'a, T>;

    fn new<I: IntoIterator<Item = Self::Target>>(iter: I) -> Self {
        Self {
            buffer: iter.into_iter().collect(),
        }
    }

    fn push(&mut self, reference: Self::Target) {
        self.buffer.push(reference)
    }

    fn peek(&self) -> Option<Self::Target> {
        self.buffer.last().map(|d| *d)
    }
}

impl<'a, T> Deref for MutReferenceChain<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &*self.peek().unwrap().deref()
    }
}

impl<'a, T> DerefMut for MutReferenceChain<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.peek().unwrap().deref_mut()
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn ref_cells() {}
}


 */
