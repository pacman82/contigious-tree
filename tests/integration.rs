use std::io::Write;

use contigious_tree::{Node, Tree, TreeBuilder};

#[test]
fn leaf() {
    // Given
    let mut persistence = Vec::<u8>::new();

    // When
    let mut builder = TreeBuilder::<PlainInt, _>::new(&mut persistence);
    builder.add_node(&42, 0).unwrap();
    let tree = Tree::<PlainInt>::new(persistence);
    let (value, mut branches) = tree.value();

    // Then
    assert_eq!(42, value);
    assert!(branches.next().is_none());
}

#[test]
fn root_node_with_two_children() {
    // Given
    let mut persistence = Vec::<u8>::new();

    // When
    let mut builder = TreeBuilder::<PlainU8, _>::new(&mut persistence);
    // First child
    builder.add_node(&1, 0).unwrap();
    // Second child
    builder.add_node(&2, 0).unwrap();
    // Parent
    builder.add_node(&3, 2).unwrap();
    eprintln!("PERSISTENCE: {:?}", persistence);
    // Read tree
    let tree = Tree::<PlainU8>::new(persistence);

    // Then
    let (value, mut branches) = tree.value();
    assert_eq!(3, value);
    let second = branches.next().unwrap();
    let first = branches.next().unwrap();
    assert!(branches.next().is_none());
    let (value, mut branches) = second.value();
    assert_eq!(2, value);
    assert!(branches.next().is_none());
    let (value, mut branches) = first.value();
    assert_eq!(1, value);
    assert!(branches.next().is_none())
}

struct PlainInt;

impl Node for PlainInt {
    type Value = i32;

    fn write_value<W>(writer: &mut W, value: &Self::Value) -> std::io::Result<usize>
    where
        W: Write,
    {
        let bytes = value.to_le_bytes();
        writer.write_all(&bytes)?;
        Ok(bytes.len()) // Should always be 4
    }

    fn read_value(bytes: &[u8]) -> (usize, i32) {
        let total_len = bytes.len();
        let last_four_bytes: &[u8; 4] = bytes[(total_len - 4)..].try_into().unwrap();
        (4, i32::from_le_bytes(*last_four_bytes))
    }
}

struct PlainU8;

impl Node for PlainU8 {
    type Value = u8;

    fn write_value<W>(writer: &mut W, value: &Self::Value) -> std::io::Result<usize>
    where
        W: Write,
    {
        let bytes = value.to_le_bytes();
        writer.write_all(&bytes)?;
        Ok(bytes.len()) // Should always be 1
    }

    fn read_value(bytes: &[u8]) -> (usize, u8) {
        let total_len = bytes.len();
        let last_four_bytes: &[u8; 1] = bytes[(total_len - 1)..].try_into().unwrap();
        (1, u8::from_le_bytes(*last_four_bytes))
    }
}
