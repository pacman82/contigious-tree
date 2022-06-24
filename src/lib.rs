//! Write trees to any `io::Write` depth first. Read trees into contigious blocks of memory.

use std::{
    io::{self, Write},
    marker::PhantomData,
    mem::size_of,
    ops::Deref,
};

/// Used to store the binary sizes of [`Tree`]s and [`TreeSlice`]s in bytes. This would usually be
/// done utilizing `usize`, yet the size of `usize` is platform dependend. Since part of the appeal
/// of a serializable tree data structure is to store it to a filesystem and load it, it seems
/// beneficial to fix this to 64Bit on any platform to not introduce a dependency of the fileformat
/// to the platform it has been generated on.
pub type TreeSize = u64;

/// Helpful if we want to extract a value of [`TreeSize`] out of a raw binary representation of
/// binary slices or in calculating the size of a subtree.
const TREE_SIZE_SIZE: usize = size_of::<TreeSize>();

/// [`Tree`] is generic over the value types associated with each node. Furthermore it is also
/// generic about the way these are serialized. E.g. A value type of `i64` could be stored in
/// little endian, big endian or a bitpacked representation. This allows us to adapt the tree to a
/// wide variaty of usecases.
pub trait Node {
    /// The value type associated with each node in the tree.
    type Value;

    /// Writes the value, so [`Self::read_value`] can extract it again. In case of success the
    /// number of bytes written is returned.
    fn write_value<W>(writer: &mut W, value: &Self::Value) -> io::Result<usize>
    where
        W: Write;

    /// Reads the value from a raw binary representation. Reads the value from the back of the
    /// passed slice.
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

    pub fn write_node(&mut self, value: &N::Value, num_children: usize) -> io::Result<()>
    where
        N: Node,
        W: Write,
    {
        // All previous children have been written and are immediate predecessors to this node.
        // Layout: children, value, totalsize
        let size_value: TreeSize = N::write_value(&mut self.writer, value)? as TreeSize;
        let size_children: TreeSize = self
            .open_node_sizes
            .drain((self.open_node_sizes.len() - num_children)..)
            .sum();
        let total_size = size_value + size_children;
        self.writer.write_all(&total_size.to_le_bytes())?;
        // We write the size, without the size of the size value itself. However, then accounting
        // for all the childern it must of course be added.
        self.open_node_sizes.push(total_size + TREE_SIZE_SIZE as TreeSize);
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

    pub fn read_node(&self) -> (N::Value, Branches<'_, N>)
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
            let tree_size = TreeSize::from_le_bytes(*tree_size_bytes) as usize;
            let (remainder, tree_slice) = self.bytes.split_at(total_size - tree_size - TREE_SIZE_SIZE);
            let tree_slice = TreeSlice::from_slice(tree_slice);

            // Advance iterator by assigning all bytes **not** part of the tree slice just returned.
            self.bytes = remainder;

            Some(tree_slice)
        }
    }
}
