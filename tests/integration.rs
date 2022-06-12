use std::io::Write;

use contigious_tree::{Node, Tree, TreeBuilder};

#[test]
fn leaf() {
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
            let total_len = dbg!(bytes).len();
            let last_four_bytes: &[u8; 4] = bytes[(total_len - 4)..].try_into().unwrap();
            (4, i32::from_le_bytes(*last_four_bytes))
        }
    }

    let mut persistence = Vec::<u8>::new();
    let mut builder = TreeBuilder::<PlainInt, _>::new(&mut persistence);
    builder.add_node(&42, 0).unwrap();
    eprintln!("PERSISTENCE: {:?}", persistence);
    let tree = Tree::<PlainInt>::new(persistence);
    let (value, mut branches) = tree.value();

    assert_eq!(42, value);
    assert!(branches.next().is_none());
}
