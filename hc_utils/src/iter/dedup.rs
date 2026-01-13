use std::hash::Hash;

use crate::small::SmallSet;


pub struct Dedup<I: Iterator> where I::Item: Hash + Eq + Clone {
    iterator: I,
    set: SmallSet<I::Item>
}

impl<I: Iterator> Iterator for Dedup<I> where I::Item: Hash + Eq + Clone{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iterator.next() {
            Some(val) => {
                if self.set.contains(&val) {
                    self.next()
                } else {
                    self.set.insert(val.clone());
                    Some(val)
                }
            },
            None => None
        }
    }
}

pub trait Deduped where Self: Iterator + Sized, Self::Item: Hash + Eq + Clone {
    fn dedup(self) -> Dedup<Self>;
}

impl<I: Iterator> Deduped for I where Self::Item: Hash + Eq + Clone {
    fn dedup(self) -> Dedup<Self> {
        Dedup { iterator: self, set: SmallSet::new() }
    }
}
