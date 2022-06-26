# Contigious tree

[![Docs](https://docs.rs/contigious-tree/badge.svg)](https://docs.rs/contigious-tree/)
[![Licence](https://img.shields.io/crates/l/contigious-tree)](https://github.com/pacman82/contigious-tree/blob/main/License)
[![Crates.io](https://img.shields.io/crates/v/contigious-tree)](https://crates.io/crates/contigious-tree)

Write and read tree graphs to and from contigious blocks of memory.

## Usage

Consider the following tree



We write trees in a depth first manner. With each subrtee written, before the parent node which owns it.

```rust
use contigious_tree::{TreeVec, TreeBuilder, U8};

// Let's write and read the following tree:
// (1) root
//  ├── (2)
//  └── (3)
//       └── (4)

// Any io::Write, will do for writing
let mut persistence = Vec::<u8>::new();

let mut builder = TreeBuilder::<U8, _>::new(&mut persistence);
// Build tree depth first. For each node pass a reference to the value and the number of preceding
// direct children.
builder.write_node(&4, 0).unwrap();
builder.write_node(&3, 1).unwrap();
builder.write_node(&2, 0).unwrap();
builder.write_node(&1, 2).unwrap();

// ...

// Read tree
let tree = TreeVec::<U8>::new(persistence);
// Read value of tree root, and iterate over direct children
let (value, mut branches) = tree.read_node();
assert_eq!(1, value);
let first = branches.next().unwrap();
let second = branches.next().unwrap();
assert!(branches.next().is_none());
// First branch has value 2 and no children
let (value, mut branches) = first.read_node();
assert_eq!(2, value);
assert!(branches.next().is_none());
// Second branch has value 3 and one child with value 4
let (value, mut branches) = second.read_node();
assert_eq!(3, value);
assert_eq!(4, branches.next().unwrap().read_node().0);
```
