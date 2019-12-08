use std::iter::FromIterator;

#[derive(Clone, Debug)]
struct Node<T> {
    value: T,
    rank: u32,
    left: Box<MinHeap<T>>,
    right: Box<MinHeap<T>>,
}

impl<T: Clone + Ord> Node<T> {
    pub fn merge(&self, other: &Self) -> Self {
        let (min, max) = if self.value < other.value {
            (self, other)
        } else {
            (other, self)
        };

        let right = min.right.merge_data(max);
        let right_rank = right.rank;
        let right = Box::new(MinHeap::from_node(right));
        let value = min.value.clone();

        match &(*min.left).node {
            Some(left) => {
                if left.rank < right_rank {
                    Self {
                        value,
                        rank: left.rank + 1,
                        left: right,
                        right: min.left.clone(),
                    }
                } else {
                    Self {
                        value,
                        rank: right_rank + 1,
                        left: min.left.clone(),
                        right,
                    }
                }
            }
            None => Self {
                value,
                rank: 1,
                left: right,
                right: min.left.clone(),
            },
        }
    }
}

#[derive(Clone, Debug)]
pub struct MinHeap<T> {
    node: Option<Node<T>>,
}

impl<T> Default for MinHeap<T> {
    fn default() -> Self {
        MinHeap { node: None }
    }
}

impl<T: Clone + Ord> MinHeap<T> {
    pub fn new(value: T) -> Self {
        MinHeap {
            node: Some(Node {
                value,
                rank: 1,
                left: Box::new(MinHeap::default()),
                right: Box::new(MinHeap::default()),
            }),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.node.is_none()
    }

    fn from_node(node: Node<T>) -> Self {
        Self { node: Some(node) }
    }

    fn merge_data(&self, other: &Node<T>) -> Node<T> {
        self.node
            .as_ref()
            .map(|data| data.merge(other))
            .unwrap_or_else(|| other.clone())
    }

    pub fn merge(&self, other: &Self) -> Self {
        match (&self.node, &other.node) {
            (None, x) | (x, None) => MinHeap { node: x.clone() },
            (Some(left), Some(right)) => MinHeap::from_node(left.merge(right)),
        }
    }

    pub fn insert(&self, value: T) -> Self {
        self.merge(&Self::new(value))
    }

    pub fn pop(&self) -> (Self, Option<T>) {
        match &self.node {
            Some(node) => (node.left.merge(&node.right), Some(node.value.clone())),
            None => (Self::default(), None),
        }
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            state: self.clone(),
        }
    }
}

impl<T: Clone + Ord> FromIterator<T> for MinHeap<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut ret = Self::default();

        for item in iter {
            ret = ret.insert(item);
        }

        ret
    }
}

pub struct Iter<T> {
    state: MinHeap<T>,
}

impl<T: Clone + Ord> Iterator for Iter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let (state, ret) = self.state.pop();
        self.state = state;
        ret
    }
}

#[test]
fn insert() {
    let heap = MinHeap::from_iter([4, 5, 6, 8].iter().cloned());

    assert_eq!(heap.node.as_ref().unwrap().value, 4);
    assert_eq!(heap.node.as_ref().unwrap().rank, 2);
}

#[test]
fn pop() {
    let heap = MinHeap::from_iter([4, 1, 2, 3].iter().cloned());
    let mut iter = heap.iter();

    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), Some(3));
    assert_eq!(iter.next(), Some(4));
    assert_eq!(iter.next(), None);
}
