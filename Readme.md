# Contigious tree

[![Docs](https://docs.rs/contigious-tree/badge.svg)](https://docs.rs/contigious-tree/)
[![Licence](https://img.shields.io/crates/l/contigious-tree)](https://github.com/pacman82/contigious-tree/blob/main/License)
[![Crates.io](https://img.shields.io/crates/v/contigious-tree)](https://crates.io/crates/contigious-tree)

Write and read tree graphs to and from contigious blocks of memory.

## About

A useful tree representation for situations there you want to serialize / deserialize a tree, or query it very quickly. Do not use this crate if you need to change your tree frequently. The implementation is generic over the value type associated with their nodes and their binary representation.

## Usage

Consider this tree:

```
(1) root
 ├── (2)
 └── (3)
      └── (4)
```

### Writing

We write trees in a depth first manner. With each subrtee written, before the parent node which owns it.

```rust
use contigious_tree::{TreeBuilder, LeI32};

/// Value type is a singend 32 Bit integer with a little endian representation.
type Node = LeI32;

// Any io::Write, will do for writing
let mut persistence = Vec::<u8>::new();

let mut builder = TreeBuilder::<Node, _>::new(&mut persistence);
// Build tree depth first. For each node pass a reference to the value and the number of preceding
// direct children.
builder.write_node(&4, 0).unwrap();
builder.write_node(&3, 1).unwrap();
builder.write_node(&2, 0).unwrap();
builder.write_node(&1, 2).unwrap();
```

### Reading

```rust
use contigious_tree::{TreeVec, LeI32};

let persistence: Vec<u8> = { /*... load tree from stoarge ...*/};

/// Value type is a singend 32 Bit integer with a little endian representation.
type Node = LeI32;

let tree = TreeVec::<Node>::new(persistence);
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
