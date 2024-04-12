/*
	heap
	This question requires you to implement a binary heap function
*/


use std::cmp::Ord;
use std::default::Default;


pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default + Ord
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

   

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        //TODO
        //堆中的self.items是从1开始做索引的。
        self.items.push(value);

        let mut idx = self.count + 1;
        while idx > 1 {
            let p_idx = self.parent_idx(idx);
            if (self.comparator)(&self.items[idx], &self.items[p_idx])  {
                self.items.swap(p_idx, idx);
                idx = p_idx;
            } else {
                break;
            }
        }
        
        self.count += 1;
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        //TODO
		let left_idx = self.left_child_idx(idx);
        let right_idx = self.right_child_idx(idx);

        if left_idx > self.count && right_idx > self.count {
            panic!("left: {}, right: {}, length: {}, all index out of bound.", left_idx, right_idx, self.len());
        } else if left_idx > self.count && right_idx <= self.count {
            return right_idx;
        } else if left_idx <= self.count && right_idx > self.count {
            return left_idx;
        }

        if (self.comparator)(&self.items[left_idx], &self.items[right_idx]) {
            left_idx
        } else {
            right_idx
        }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default + Copy + Ord,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        //TODO
        if self.count > 0 {
            let res = self.items[1];

            self.items.swap(1, self.count);
            self.count -= 1;
            
            let mut idx = 1;
            while idx <= self.count {
                let mut small_child_idx = 0;
                if self.right_child_idx(idx) <= self.count {
                    small_child_idx = self.smallest_child_idx(idx);
                } else {
                    break;
                }

                if (self.comparator)(&self.items[small_child_idx], &self.items[idx]) {
                    self.items.swap(small_child_idx, idx);
                    idx = small_child_idx;
                } else {
                    break;
                }
            }

            self.items.pop();
            Some(res)
        } else {
            None
        }
            
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        // heap.next();
        // for v in &heap.items {
        //     println!("{}", v);
        // }
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        // for _ in (0..4) {
        //     println!("{}", heap.next().unwrap());
        // }
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}