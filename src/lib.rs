//! Write trees to any `io::Write` depth first. Read trees into contigious blocks of memory.

use std::{marker::PhantomData, io};

pub trait Node {
    type Value;
    
    fn write_value<W>(writer: &mut W, value: &Self::Value) -> io::Result<()>;
}

pub struct TreeBuilder<N>{
    _nodes: PhantomData<N>
}

impl<N> TreeBuilder<N> {

    pub fn new() -> Self {
        Self{
            _nodes: PhantomData
        }
    }

    pub fn add_node(&mut self, value: &N::Value, num_children: usize) -> io::Result<()> where N: Node {
        Ok(())
    }
}

/// An owned tree, which is stored in contigious memory. Fast traversal and query times.
pub struct Tree;

