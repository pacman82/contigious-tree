//! Write trees to any `io::Write` depth first. Read trees into contigious blocks of memory.

use std::{
    io::{self, Write},
    marker::PhantomData,
    mem::size_of,
    ops::Deref,
};

pub type TreeSize = u64;

const TREE_SIZE_SIZE: usize = size_of::<TreeSize>();

pub trait Node {
    type Value;

    fn write_value<W>(writer: &mut W, value: &Self::Value) -> io::Result<usize>
    where
        W: Write;

    fn read_value(bytes: &[u8]) -> (usize, Self::Value);
}

pub struct TreeBuilder<N, W> {
    _node_type: PhantomData<N>,
    open_node_sizes: Vec<TreeSize>,
    writer: W,
}

impl<N, W> TreeBuilder<N, W> {
    pub fn new(writer: W) -> Self {
        Self {
            _node_type: PhantomData,
            open_node_sizes: Vec::new(),
            writer,
        }
    }

    pub fn add_node(&mut self, value: &N::Value, num_children: usize) -> io::Result<()>
    where
        N: Node,
        W: Write,
    {
        // All previous children have been written and are immediate predecessors to this node.
        // Layout: children, value, totalsize
        let size_value: TreeSize = N::write_value(&mut self.writer, value)?.try_into().unwrap();
        let size_children: TreeSize = self
            .open_node_sizes
            .drain((self.open_node_sizes.len() - num_children)..)
            .sum();
        let total_size = size_value + size_children;
        self.writer.write_all(&total_size.to_le_bytes())?;
        self.open_node_sizes.push(total_size);
        Ok(())
    }
}

/// An owned tree, which is stored in contigious memory. Fast traversal and query times.
pub struct Tree<N> {
    _node_type: PhantomData<N>,
    bytes: Vec<u8>,
}

impl<N> Tree<N> {
    pub fn new(bytes: Vec<u8>) -> Tree<N> {
        Tree {
            _node_type: PhantomData,
            bytes,
        }
    }

    pub fn as_tree_slice(&self) -> &TreeSlice<N> {
        TreeSlice::from_slice(&self.bytes)
    }
}

impl<N> Deref for Tree<N> {
    type Target = TreeSlice<N>;

    fn deref(&self) -> &Self::Target {
        self.as_tree_slice()
    }
}

pub struct TreeSlice<N> {
    _node_type: PhantomData<N>,
    bytes: [u8],
}

impl<N> TreeSlice<N> {
    pub fn from_slice(slice: &[u8]) -> &Self {
        let ptr: *const [u8] = slice;
        unsafe { &*(ptr as *const TreeSlice<N>) }
    }

    pub fn value(&self) -> (N::Value, Branches<'_, N>)
    where
        N: Node,
    {
        let total_size = self.bytes.len();
        let (size_value, value) = N::read_value(&self.bytes[..(total_size - TREE_SIZE_SIZE)]);
        let branches = Branches {
            _node_type: PhantomData,
            bytes: &self.bytes[..(total_size - TREE_SIZE_SIZE - size_value)],
        };
        (value, branches)
    }
}

pub struct Branches<'a, N> {
    _node_type: PhantomData<N>,
    bytes: &'a [u8],
}

impl<'a, N: 'a> Iterator for Branches<'a, N> {
    type Item = &'a TreeSlice<N>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.bytes.is_empty() {
            None
        } else {
            let total_size = self.bytes.len();
            let tree_size_bytes: &[u8; TREE_SIZE_SIZE] = self.bytes
                [(total_size - TREE_SIZE_SIZE)..]
                .try_into()
                .unwrap();
            let tree_size = TreeSize::from_le_bytes(*tree_size_bytes);
            let tree_size: usize = tree_size.try_into().unwrap();
            let tree_slice = TreeSlice::from_slice(&self.bytes[..total_size - tree_size - TREE_SIZE_SIZE]);
            Some(tree_slice)
        }
    }
}
