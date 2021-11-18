use std::iter::{Skip};

pub struct RangedIterator<A: Iterator> {
    parent: A,
    remaining: usize
}

impl<A: Iterator> RangedIterator<A> {
    pub fn new (parent: A, from: usize, to: usize) -> RangedIterator<Skip<A>> {
        RangedIterator { parent: parent.skip(from), remaining: to - from }
    }

    pub fn limit (parent: A, len: usize) -> RangedIterator<A> {
        RangedIterator { parent, remaining: len }
    }
}

impl<A: Iterator> Iterator for RangedIterator<A> {
    type Item = A::Item;

    fn next (&mut self) -> Option<Self::Item> {
        if self.remaining <= 0 {
            return None
        }

        self.remaining -= 1;
        self.parent.next()
    }
}