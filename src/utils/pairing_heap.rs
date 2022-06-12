use std::fmt::Debug;
use crate::utils::log::console_log;

type BoxedNode<'a, V> = Box<Node<'a, V>>;

#[derive(Debug)]
struct Node<'a, V: PartialOrd + Debug> {
    pub value: &'a V,
    left: Option<BoxedNode<'a, V>>,
    next: Option<BoxedNode<'a, V>>,
}

impl<'a, V: PartialOrd + Debug> Node<'a, V> {
    pub fn new(value: &'a V) -> Self {
        Node {
            value,
            left: None,
            next: None
        }
    }
    pub fn value(&self) -> &'a V {
        self.value
    }
    pub fn set_left(&mut self, node: Option<BoxedNode<'a, V>>) {
        self.left = node;
    }
    pub fn set_next(&mut self, node: Option<BoxedNode<'a, V>>) {
        self.next = node;
    }
    pub fn left(self) -> Option<BoxedNode<'a, V>> {
        self.left
    }
    pub fn next(self) -> Option<BoxedNode<'a, V>> {
        self.next
    }
}

#[derive(Debug)]
pub struct PairingHeap<'a, V: PartialOrd + Debug> {
    root: Option<BoxedNode<'a, V>>,
    heap_type: HeapType,
    size: usize,
}

#[derive(Debug)]
enum HeapType {
    Max,
    Min
}

impl<'a, V: PartialOrd + Debug> PairingHeap<'a, V> {
    pub fn min() -> Self {
        Self::new(HeapType::Min)
    }
    pub fn max() -> Self {
        Self::new(HeapType::Max)
    }
    pub fn len(&self) -> usize {
        self.size
    }
    pub fn top(&self) -> Option<&'a V> {
        self.root.as_ref().map(| node | node.value)
    }

    pub fn push(&mut self, value: &'a V)  {
        self.root = if self.root.is_some() {
            let root = self.root.take();
            self.merge(root, Some(Box::new(Node::new(value))))
        } else {
            Some(Box::new(Node::new(value)))
        };
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<&'a V> {
        self.root.take().map(| mut node | {
            self.size -= 1;
            self.root = self.two_pass_merge(node.left.take());
            node.value()
        })
    }

    fn new(heap_type: HeapType) -> Self {
        PairingHeap {
            root: None,
            heap_type,
            size: 0,
        }
    }

    fn compare(&self, a: &BoxedNode<'a, V>, b: &BoxedNode<'a, V>) -> bool {
        match self.heap_type {
            HeapType::Max => a.value > b.value,
            HeapType::Min => a.value < b.value,
        }
    }

    fn add_child(mut parent: BoxedNode<'a, V>, mut child: BoxedNode<'a, V>) -> BoxedNode<'a, V> {
        if parent.left.is_some() {
            child.set_next(parent.left.take());
        }
        parent.set_left(Some(child));
        parent
    }

    fn merge(&mut self, node_a: Option<BoxedNode<'a, V>>, node_b: Option<BoxedNode<'a, V>>) -> Option<BoxedNode<'a, V>> {
        match (node_a, node_b) {
            (Some(a), Some(b)) => Some(if self.compare(&a, &b) {
                Self::add_child(a, b)
            } else {
                Self::add_child(b, a)
            }),
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            _ => None,
        }
    }

    fn two_pass_merge(&mut self, node: Option<BoxedNode<'a, V>>) -> Option<BoxedNode<'a, V>> {
        let mut root = node;
        let mut merged: Option<BoxedNode<'a, V>> = None;

        while let Some(mut parent) = root {
            if let Some(mut child) = parent.next.take() {
                root = child.next.take();
                let children = self.merge(Some(parent), Some(child));
                merged = self.merge(merged, children);
            } else {
                merged = self.merge(merged, Some(parent));
                root = None;
            }
        }
        merged
    }
}

#[cfg(test)]
mod pop {
    use wasm_bindgen_test::{console_log, wasm_bindgen_test};
    use super::PairingHeap;

    #[wasm_bindgen_test]
    fn returns_the_first_value_from_the_heap() {
        let mut heap: PairingHeap<i32> = PairingHeap::min();
        let numbers = vec![4,3,1,2,5,7,6];
        numbers.iter().for_each(| n | {
            let _ = &mut heap.push(n);
        });
        assert_eq!(heap.pop(), Some(&1));
    }

    #[wasm_bindgen_test]
    fn removes_the_first_value_from_heap() {
        let mut heap: PairingHeap<i32> = PairingHeap::min();
        let numbers = vec![4,3,1,2,5,7,6];
        numbers.iter().for_each(| n | {
            let _ = &mut heap.push(n);
        });
        let _ = heap.pop();
        assert_eq!(heap.top(), Some(&2));
    }

    #[wasm_bindgen_test]
    fn returns_none_if_the_heap_is_empty() {
        let mut heap: PairingHeap<i32> = PairingHeap::min();
        assert_eq!(heap.pop(), None);
    }

    #[wasm_bindgen_test]
    fn returns_all_elements_from_smallest_to_largest_in_a_min_heap() {
        let mut heap: PairingHeap<i32> = PairingHeap::min();
        let numbers = vec![4,3,1,2,5,7,6];
        numbers.iter().for_each(| n | {
            let _ = &mut heap.push(n);
        });
        assert_eq!(heap.pop(), Some(&1));
        assert_eq!(heap.pop(), Some(&2));
        assert_eq!(heap.pop(), Some(&3));
        assert_eq!(heap.pop(), Some(&4));
        assert_eq!(heap.pop(), Some(&5));
        assert_eq!(heap.pop(), Some(&6));
        assert_eq!(heap.pop(), Some(&7));
        assert_eq!(heap.pop(), None);
    }

    #[wasm_bindgen_test]
    fn returns_all_elements_from_largest_to_smallest_in_a_max_heap() {
        let mut heap: PairingHeap<i32> = PairingHeap::max();
        let numbers = vec![4,3,1,2,5,7,6];
        numbers.iter().for_each(| n | {
            let _ = &mut heap.push(n);
        });
        assert_eq!(heap.pop(), Some(&7));
        assert_eq!(heap.pop(), Some(&6));
        assert_eq!(heap.pop(), Some(&5));
        assert_eq!(heap.pop(), Some(&4));
        assert_eq!(heap.pop(), Some(&3));
        assert_eq!(heap.pop(), Some(&2));
        assert_eq!(heap.pop(), Some(&1));
        assert_eq!(heap.pop(), None);
    }
}

#[cfg(test)]
mod push {
    use wasm_bindgen_test::{console_log, wasm_bindgen_test};
    use super::PairingHeap;

    #[wasm_bindgen_test]
    fn adds_a_value_to_the_heap() {
        let mut heap: PairingHeap<i32> = PairingHeap::min();
        let value = 1;
        heap.push(&value);
        assert_eq!(heap.top(), Some(&value));
    }

    #[wasm_bindgen_test]
    fn adds_a_higher_item_to_the_heap_behind_a_lower_in_a_min_heap() {
        let mut heap: PairingHeap<i32> = PairingHeap::min();
        let lower = 1;
        let higher = 2;
        heap.push(&lower);
        heap.push(&higher);
        assert_eq!(heap.top(), Some(&lower));
    }

    #[wasm_bindgen_test]
    fn adds_a_higher_item_to_the_heap_before_a_lower_in_a_max_heap() {
        let mut heap: PairingHeap<i32> = PairingHeap::min();
        let lower = 1;
        let higher = 2;
        heap.push(&higher);
        heap.push(&lower);
        assert_eq!(heap.top(), Some(&lower));
    }

    #[wasm_bindgen_test]
    fn adds_a_lower_item_to_the_heap_before_a_higher_in_a_min_heap() {
        let mut heap: PairingHeap<i32> = PairingHeap::min();
        let lower = 1;
        let higher = 2;
        heap.push(&higher);
        heap.push(&lower);
        assert_eq!(heap.top(), Some(&lower));
    }

    #[wasm_bindgen_test]
    fn adds_a_lower_item_to_the_heap_behind_a_higher_in_a_max_heap() {
        let mut heap: PairingHeap<i32> = PairingHeap::max();
        let lower = 1;
        let higher = 2;
        heap.push(&higher);
        heap.push(&lower);
        assert_eq!(heap.top(), Some(&higher));
    }

    #[wasm_bindgen_test]
    fn returns_none_if_the_list_is_empty() {
        let mut heap: PairingHeap<i32> = PairingHeap::min();
        assert_eq!(heap.pop(), None);
    }
}

#[cfg(test)]
mod top {
    use wasm_bindgen_test::{console_log, wasm_bindgen_test};
    use super::PairingHeap;

    #[wasm_bindgen_test]
    fn returns_the_first_value_in_a_heap() {
        let mut heap: PairingHeap<i32> = PairingHeap::min();
        let numbers = vec![4,3,1,2,5,7,6];
        numbers.iter().for_each(| n | {
            let _ = &mut heap.push(n);
        });
        assert_eq!(heap.top(), Some(&1));
    }

    #[wasm_bindgen_test]
    fn returns_none_if_the_heap_is_empty() {
        let mut heap: PairingHeap<i32> = PairingHeap::min();
        assert_eq!(heap.top(), None);
    }
}


#[cfg(test)]
mod len {
    use wasm_bindgen_test::{console_log, wasm_bindgen_test};
    use super::PairingHeap;

    #[wasm_bindgen_test]
    fn returns_the_correct_size_of_a_heap_after_adding_elements() {
        let mut heap: PairingHeap<i32> = PairingHeap::min();
        let numbers = vec![4,3,1,2,5,7,6];
        numbers.iter().for_each(| n | {
            let _ = &mut heap.push(n);
        });
        assert_eq!(heap.len(), numbers.len());
    }

    #[wasm_bindgen_test]
    fn returns_the_correct_size_of_a_heap_after_removing_an_element() {
        let mut heap: PairingHeap<i32> = PairingHeap::min();
        let numbers = vec![4,3,1,2,5,7,6];
        numbers.iter().for_each(| n | {
            let _ = &mut heap.push(n);
        });
        let _ = heap.pop();
        let _ = heap.pop();
        assert_eq!(heap.len(), numbers.len() - 2);
    }
}
