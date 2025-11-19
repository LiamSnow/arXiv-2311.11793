pub mod binary;
pub mod fibonacci;
pub mod wsb;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Item {
    pub vertex: usize,
    pub distance: usize,
}

pub trait MinHeap {
    fn with_capacity(capacity: usize) -> Self;

    fn is_empty(&self) -> bool;

    fn insert(&mut self, item: Item);

    fn delete_min(&mut self) -> Option<Item>;

    fn find_min(&self) -> Option<&Item>;

    fn decrease_key(&mut self, vertex: usize, new_key: usize);
}
