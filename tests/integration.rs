use contigious_tree::{TreeBuilder, Node};

#[test]
fn leaf() {
    struct PlainInt;

    impl Node for PlainInt {
        type Value = i32;

        fn write_value<W>(writer: &mut W, value: &Self::Value) -> std::io::Result<()> {
            todo!()
        }
    }

    let mut builder = TreeBuilder::<PlainInt>::new();
    builder.add_node(&42, 0).unwrap();
}