//! Tests for writer, tree, and parser.
#![cfg(all(feature = "tree", feature = "writer"))]

use std::{cell::RefCell, io::Cursor, rc::Rc};

use fbxcel::{
    low::FbxVersion, pull_parser::any::AnyParser, tree::v7400::Loader as TreeLoader, tree_v7400,
    writer::v7400::binary::Writer,
};

/// Construct tree, export it to binary, parse it and construct tree, and
/// compare them.
#[test]
fn tree_write_parse_idempotence_v7500() -> Result<(), Box<dyn std::error::Error>> {
    // Construct tree.
    let tree1 = tree_v7400! {
        Node0: {
            Node0_0: {},
            Node0_1: {},
        },
        Node1: [true] {
            Node1_0: (vec![42i32.into(), 1.234f64.into()]) {}
            Node1_1: [&[1u8, 2, 4, 8, 16][..], "Hello, world"] {}
        },
    };

    let mut writer = Writer::new(Cursor::new(Vec::new()), FbxVersion::V7_5)?;
    writer.write_tree(&tree1)?;
    let bin = writer.finalize_and_flush(&Default::default())?.into_inner();

    let mut parser = match AnyParser::from_seekable_reader(Cursor::new(bin))? {
        AnyParser::V7400(parser) => parser,
        _ => panic!("Generated data should be parsable with v7400 parser"),
    };
    let warnings = Rc::new(RefCell::new(Vec::new()));
    parser.set_warning_handler({
        let warnings = warnings.clone();
        move |warning, _pos| {
            warnings.borrow_mut().push(warning);
            Ok(())
        }
    });
    assert_eq!(parser.fbx_version(), FbxVersion::V7_5);

    let (tree2, footer_res) = TreeLoader::new().load(&mut parser)?;

    assert_eq!(warnings.borrow().len(), 0);
    assert!(footer_res.is_ok());

    assert!(tree1.strict_eq(&tree2));

    Ok(())
}
