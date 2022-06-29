use contigious_tree::{TreeBuilder, TreeVec, U8, LeI32};

#[test]
fn leaf() {
    // Given
    let mut persistence = Vec::<u8>::new();

    // When
    let mut builder = TreeBuilder::<LeI32, _>::new(&mut persistence);
    builder.write_node(&42, 0).unwrap();
    builder.finish().unwrap();
    let tree = TreeVec::<LeI32>::new(persistence);
    let (value, mut branches) = tree.read_node();

    // Then
    assert_eq!(42, value);
    assert!(branches.next().is_none());
}

#[test]
fn root_node_with_two_children() {
    // Given
    let mut persistence = Vec::<u8>::new();

    // When
    let mut builder = TreeBuilder::<U8, _>::new(&mut persistence);
    // First child
    builder.write_node(&1, 0).unwrap();
    // Second child
    builder.write_node(&2, 0).unwrap();
    // Parent
    builder.write_node(&3, 2).unwrap();
    builder.finish().unwrap();
    // Read tree
    let tree = TreeVec::<U8>::new(persistence);

    // Then
    let (value, mut branches) = tree.read_node();
    assert_eq!(3, value);
    let second = branches.next().unwrap();
    let first = branches.next().unwrap();
    assert!(branches.next().is_none());
    let (value, mut branches) = second.read_node();
    assert_eq!(2, value);
    assert!(branches.next().is_none());
    let (value, mut branches) = first.read_node();
    assert_eq!(1, value);
    assert!(branches.next().is_none())
}

#[test]
fn three_successive_nodes() {
    // Given
    let mut persistence = Vec::<u8>::new();

    // When
    let mut builder = TreeBuilder::<U8, _>::new(&mut persistence);
    // First child
    builder.write_node(&1, 0).unwrap();
    // Second child
    builder.write_node(&2, 1).unwrap();
    // Parent
    builder.write_node(&3, 1).unwrap();
    builder.finish().unwrap();
    // Read tree
    let tree = TreeVec::<U8>::new(persistence);

    // Then
    let (value, mut branches) = tree.read_node();
    assert_eq!(3, value);
    let tree_slice = branches.next().unwrap();
    assert!(branches.next().is_none());
    let (value, mut branches) = tree_slice.read_node();
    assert_eq!(2, value);
    let tree_slice = branches.next().unwrap();
    assert!(branches.next().is_none());
    let (value, mut branches) = tree_slice.read_node();
    assert_eq!(1, value);
    assert!(branches.next().is_none())
}
