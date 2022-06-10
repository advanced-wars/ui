type BoxedNode<'a, V> = Box<Node<'a, V>>;

struct Node<'a, V: PartialOrd> {
    pub value: &'a V,
    left: Option<BoxedNode<'a, V>>,
    next: Option<BoxedNode<'a, V>>,
}

impl<'a, V: PartialOrd> Node<'a, V> {
    pub fn set_left(&mut self, node: BoxedNode<'a, V>) {
        self.left = Some(node);
    }

    pub fn set_next(&mut self, node: BoxedNode<'a, V>) {
        self.next = Some(node);
    }

    pub fn left(self) -> Option<BoxedNode<'a, V>> {
        self.left
    }

    pub fn next(self) -> Option<BoxedNode<'a, V>> {
        self.next
    }
}

impl<'a, V: PartialOrd> Node<'a, V> {
    pub fn new(value: &'a V) -> Self {
        Node {
            value,
            left: None,
            next: None
        }
    }
}

pub struct PairingHeap<'a, V: PartialOrd> {
    root: Option<BoxedNode<'a, V>>,
    heap_type: HeapType,
}

enum HeapType {
    Max,
    Min
}

impl<'a, V: PartialOrd> PairingHeap<'a, V> {
    fn new(heap_type: HeapType) -> Self {
        PairingHeap {
            root: None,
            heap_type
        }
    }

    pub fn min() -> Self {
        Self::new(HeapType::Min)
    }

    pub fn max() -> Self {
        Self::new(HeapType::Max)
    }

    fn add_child(parent: Option<BoxedNode<'a, V>>, mut child: BoxedNode<'a, V>) -> BoxedNode<'a, V> {
        if let Some(node) = parent {
            child.set_next(node);
        }
        child
    }

    pub fn add(mut self, value: &'a V)  {
        self.root = Some(Self::add_child(self.root, Box::new(Node::new(value))));
    }

    pub fn top(self) -> Option<&'a V> {
        if let Some(node) = self.root {
            Some(node.value)
        } else {
            None
        }
    }

    fn merge(&mut self, node_a: Option<BoxedNode<'a, V>>, node_b: Option<BoxedNode<'a, V>>) -> Option<BoxedNode<'a, V>> {
        match (node_a, node_b) {
            (Some(a), Some(b)) => {
                let comparison = match self.heap_type {
                    HeapType::Max => a.value > b.value,
                    HeapType::Min => a.value < b.value,
                };
                Some(if comparison {
                    Self::add_child(Some(a), b)
                } else {
                    Self::add_child(Some(b), a)
                })
            },
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (None, None) => None,
        }
    }
}