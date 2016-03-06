use std::num::Zero;

pub trait Indexer: PartialOrd + Zero {
    fn as_usize(self) -> usize;
}

impl Indexer for isize {
    fn as_usize(self) -> usize {
        self as usize
    }
}

impl Indexer for usize {
    fn as_usize(self) -> usize {
        self
    }
}

impl Indexer for i32 {
    fn as_usize(self) -> usize {
        self as usize
    }
}
