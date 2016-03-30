
use std;

macro_rules! init_array(
    ($ty:ty, $len:expr, $val:expr) => (
        {
            let mut array: [$ty; $len] = unsafe { std::mem::uninitialized() };
            for i in array.iter_mut() {
                unsafe { ::std::ptr::write(i, $val); }
            }
            array
        }
    )
);

const B: usize = 6;
pub const CAPACITY: usize = 2 * B - 1;

struct LeafNode<K, V> {
    keys: [Option<K>; CAPACITY],
    vals: [Option<V>; CAPACITY],
    //parent: &InternalNode<K, V>,
    //parent_idx: u16,
    len: u16,
}

impl<K, V> LeafNode<K, V> {
    fn new() -> Self {
        LeafNode {
            keys: init_array!(Option<K>, CAPACITY, None),
            vals: init_array!(Option<V>, CAPACITY, None),
            len: 0,
        }
    }
}

struct InternalNode<K, V> {
    data: LeafNode<K, V>,
    edges: [Option<BoxedNode<K, V>>; 2 * B],
}

impl<K, V> InternalNode<K, V> {
    fn new() -> Self {
        InternalNode {
            data: LeafNode::new(),
            edges: init_array!(Option<BoxedNode<K, V>>, 2 * B, None),
        }
    }
}

enum Node<K, V> {
    Leaf(LeafNode<K, V>),
    Internal(InternalNode<K, V>)
}

type BoxedNode<K, V> = Box<Node<K, V>>;

pub struct Root<K, V> {
    node: BoxedNode<K, V>,
    height: usize
}
