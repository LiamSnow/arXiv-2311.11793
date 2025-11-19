use crate::heaps::{Item, MinHeap};

pub struct BinaryHeap {
    items: Vec<Item>,
    /// vertex -> item index
    lut: Vec<Option<usize>>,
}

impl MinHeap for BinaryHeap {
    fn with_capacity(capacity: usize) -> Self {
        Self {
            items: Vec::with_capacity(capacity),
            lut: vec![None; capacity],
        }
    }

    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    fn insert(&mut self, item: Item) {
        if item.vertex >= self.lut.len() {
            self.lut.resize(item.vertex + 1, None);
        }

        self.lut[item.vertex] = Some(self.items.len());
        self.items.push(item);
        self.bubble_up(self.items.len() - 1);
    }

    fn delete_min(&mut self) -> Option<Item> {
        if self.items.is_empty() {
            return None;
        }

        let min_item = self.items[0];
        self.lut[min_item.vertex] = None;

        if self.items.len() == 1 {
            self.items.pop();
            return Some(min_item);
        }

        let last = self.items.pop().unwrap();
        self.items[0] = last;
        self.lut[last.vertex] = Some(0);
        self.bubble_down(0);

        Some(min_item)
    }

    fn find_min(&self) -> Option<&Item> {
        self.items.first()
    }

    fn decrease_key(&mut self, vertex: usize, new_key: usize) {
        let Some(&Some(item_index)) = self.lut.get(vertex) else {
            return;
        };

        self.items[item_index].distance = new_key;
        self.bubble_up(item_index);
    }
}

impl BinaryHeap {
    // for item at i
    //  - parent = (i-1)/2
    //  - left child = 2*i + 1
    //  - right child = 2*i + 2

    fn bubble_up(&mut self, mut i: usize) {
        while i > 0 {
            let parent_i = (i - 1) >> 1;

            if self.items[parent_i].distance <= self.items[i].distance {
                break;
            }

            let child_vertex = self.items[i].vertex;
            let parent_vertex = self.items[parent_i].vertex;

            self.items.swap(i, parent_i);
            self.lut[child_vertex] = Some(parent_i);
            self.lut[parent_vertex] = Some(i);

            i = parent_i;
        }
    }

    fn bubble_down(&mut self, mut i: usize) {
        loop {
            let left_i = (i << 1) + 1;
            let right_i = left_i + 1;
            let mut smallest = i;

            if let Some(left_item) = self.items.get(left_i)
                && left_item.distance < self.items[smallest].distance
            {
                smallest = left_i;
            }

            if let Some(right_item) = self.items.get(right_i)
                && right_item.distance < self.items[smallest].distance
            {
                smallest = right_i;
            }

            if smallest == i {
                break;
            }

            let parent_vertex = self.items[i].vertex;
            let child_vertex = self.items[smallest].vertex;

            self.items.swap(i, smallest);
            self.lut[parent_vertex] = Some(smallest);
            self.lut[child_vertex] = Some(i);

            i = smallest;
        }
    }
}
