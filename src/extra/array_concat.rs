use std::{iter::Iterator};

pub struct IteratorConcat<T, A: Iterator<Item = T>, B: Iterator<Item = T>> {
    first: A,
    last: B,
    first_done: bool
}

impl<T, A: Iterator<Item = T>, B: Iterator<Item = T>> IteratorConcat<T,A,B> {
    pub fn new (first: A, last: B) -> IteratorConcat<T,A,B> {
        IteratorConcat { first, last, first_done: false }
    }
}

impl<T, A: Iterator<Item = T>, B: Iterator<Item = T>> Iterator for IteratorConcat<T,A,B>  {
    type Item = T;

    fn next (&mut self) -> Option<T> {
        if self.first_done {
            return self.last.next();
        } else {
            let val = self.first.next();
            match val {
                Some(x) => return Some(x),
                None => {
                    self.first_done = true;
                    return self.last.next();
                }
            }
        }
    }
}