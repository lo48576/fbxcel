//! Writer test.
#![cfg(feature = "writer")]

use std::{cell::RefCell, io::Cursor, iter, rc::Rc};

use fbxcel::{
    low::FbxVersion,
    pull_parser::{
        any::{from_seekable_reader, AnyParser},
        v7400::{Attributes, Event, Parser},
        Error as ParseError, ParserSource,
    },
    writer::v7400::binary::{FbxFooter, Writer},
};

const MAGIC: &[u8] = b"Kaydara FBX Binary  \x00\x1a\x00";

const CUSTOM_UNKNOWN1: [u8; 16] = [
    0xff, 0xbe, 0xad, 0x0c, 0xdb, 0xca, 0xd9, 0x68, 0xb7, 0x76, 0xf5, 0x84, 0x13, 0xf2, 0x21, 0x70,
];

const UNKNOWN3: [u8; 16] = [
    0xf8, 0x5a, 0x8c, 0x6a, 0xde, 0xf5, 0xd9, 0x7e, 0xec, 0xe9, 0x0c, 0xe3, 0x75, 0x8f, 0x29, 0x0b,
];

#[test]
fn empty_write_v7400() -> Result<(), Box<dyn std::error::Error>> {
    let mut dest = Vec::new();
    let cursor = Cursor::new(&mut dest);
    let writer = Writer::new(cursor, FbxVersion::V7_4)?;
    let footer = FbxFooter {
        unknown1: Some(&CUSTOM_UNKNOWN1),
        padding_len: Default::default(),
        unknown2: None,
        unknown3: None,
    };
    writer.finalize_and_flush(&footer)?;

    let expected = {
        let raw_ver = 7400u32;
        let mut vec = Vec::new();
        // Header.
        {
            // Magic.
            vec.extend(MAGIC);
            // Version.
            vec.extend(&raw_ver.to_le_bytes());
        }
        // No nodes.
        {
            // End of implicit root.
            {
                vec.extend(iter::repeat(0).take(4 * 3 + 1));
            }
        }
        // Footer.
        {
            // Footer: unknown1.
            vec.extend(&CUSTOM_UNKNOWN1);
            // Footer: padding.
            {
                let len = vec.len().wrapping_neg() % 16;
                assert_eq!((vec.len() + len) % 16, 0);
                vec.extend(iter::repeat(0).take(len));
            }
            // Footer: unknown2.
            vec.extend(&[0; 4]);
            // Footer: FBX version.
            vec.extend(&raw_ver.to_le_bytes());
            // Footer: 120 zeroes.
            vec.extend(iter::repeat(0).take(120));
            // Footer: unknown3.
            vec.extend(&UNKNOWN3);
        }
        vec
    };

    assert_eq!(dest.len() % 16, 0);
    assert_eq!(dest, expected);

    let mut parser = match from_seekable_reader(Cursor::new(dest))? {
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
    assert_eq!(parser.fbx_version(), FbxVersion::V7_4);

    {
        let footer_res = expect_fbx_end(&mut parser)?;
        let footer = footer_res?;
        assert_eq!(footer.unknown1, CUSTOM_UNKNOWN1);
        assert_eq!(footer.unknown2, [0u8; 4]);
        assert_eq!(footer.unknown3, UNKNOWN3);
    }

    assert_eq!(warnings.borrow().len(), 0);

    Ok(())
}

#[cfg(feature = "tree")]
#[test]
fn tree_write_7500() -> Result<(), Box<dyn std::error::Error>> {
    use fbxcel::tree_v7400;

    let tree = tree_v7400! {
        Node0: {
            Node0_0: {},
            Node0_1: {},
        },
        Node1: [true] {
            Node1_0: (vec![42i32.into(), 3.14f64.into()]) {}
            Node1_1: [&[1u8, 2, 4, 8, 16][..], "Hello, world"] {}
        },
    };

    let mut dest = Vec::new();
    let cursor = Cursor::new(&mut dest);
    let mut writer = Writer::new(cursor, FbxVersion::V7_5)?;
    let footer = FbxFooter {
        unknown1: Some(&CUSTOM_UNKNOWN1),
        padding_len: Default::default(),
        unknown2: None,
        unknown3: None,
    };
    writer.write_tree(&tree)?;
    writer.finalize_and_flush(&footer)?;

    let mut parser = match from_seekable_reader(Cursor::new(dest))? {
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

    {
        let attrs = expect_node_start(&mut parser, "Node0")?;
        assert_eq!(attrs.total_count(), 0);
    }
    {
        let attrs = expect_node_start(&mut parser, "Node0_0")?;
        assert_eq!(attrs.total_count(), 0);
    }
    expect_node_end(&mut parser)?;
    {
        let attrs = expect_node_start(&mut parser, "Node0_1")?;
        assert_eq!(attrs.total_count(), 0);
    }
    expect_node_end(&mut parser)?;
    expect_node_end(&mut parser)?;
    {
        let attrs = expect_node_start(&mut parser, "Node1")?;
        assert_eq!(attrs.total_count(), 1);
    }
    {
        let attrs = expect_node_start(&mut parser, "Node1_0")?;
        assert_eq!(attrs.total_count(), 2);
    }
    expect_node_end(&mut parser)?;
    {
        let attrs = expect_node_start(&mut parser, "Node1_1")?;
        assert_eq!(attrs.total_count(), 2);
    }
    expect_node_end(&mut parser)?;
    expect_node_end(&mut parser)?;

    {
        let footer_res = expect_fbx_end(&mut parser)?;
        let footer = footer_res?;
        assert_eq!(footer.unknown1, CUSTOM_UNKNOWN1);
        assert_eq!(footer.unknown2, [0u8; 4]);
        assert_eq!(footer.unknown3, UNKNOWN3);
    }

    assert_eq!(warnings.borrow().len(), 0);

    Ok(())
}

#[test]
fn macro_v7400_idempotence() -> Result<(), Box<dyn std::error::Error>> {
    use fbxcel::write_v7400_binary;

    let version = FbxVersion::V7_4;
    let mut writer = Writer::new(std::io::Cursor::new(Vec::new()), version)?;

    write_v7400_binary!(
        writer=writer,
        tree={
            Node0: {
                Node0_0: {},
                Node0_1: {},
            },
            Node1: [true] {
                Node1_0: (vec![42i32.into(), 3.14f64.into()]) {}
                Node1_1: [&[1u8, 2, 4, 8, 16][..], "Hello, world"] {}
            },
        },
    )?;
    let bin = writer.finalize_and_flush(&Default::default())?.into_inner();

    let mut parser = match from_seekable_reader(Cursor::new(bin))? {
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
    assert_eq!(parser.fbx_version(), version);

    {
        let attrs = expect_node_start(&mut parser, "Node0")?;
        assert_eq!(attrs.total_count(), 0);
    }
    {
        let attrs = expect_node_start(&mut parser, "Node0_0")?;
        assert_eq!(attrs.total_count(), 0);
    }
    expect_node_end(&mut parser)?;
    {
        let attrs = expect_node_start(&mut parser, "Node0_1")?;
        assert_eq!(attrs.total_count(), 0);
    }
    expect_node_end(&mut parser)?;
    expect_node_end(&mut parser)?;
    {
        let attrs = expect_node_start(&mut parser, "Node1")?;
        assert_eq!(attrs.total_count(), 1);
    }
    {
        let attrs = expect_node_start(&mut parser, "Node1_0")?;
        assert_eq!(attrs.total_count(), 2);
    }
    expect_node_end(&mut parser)?;
    {
        let attrs = expect_node_start(&mut parser, "Node1_1")?;
        assert_eq!(attrs.total_count(), 2);
    }
    expect_node_end(&mut parser)?;
    expect_node_end(&mut parser)?;

    {
        let footer_res = expect_fbx_end(&mut parser)?;
        assert!(footer_res.is_ok());
    }

    assert_eq!(warnings.borrow().len(), 0);

    Ok(())
}

fn expect_node_start<'a, R: ParserSource + std::fmt::Debug>(
    parser: &'a mut Parser<R>,
    name: &str,
) -> Result<Attributes<'a, R>, Box<dyn std::error::Error>> {
    match parser.next_event()? {
        Event::StartNode(node) => {
            assert_eq!(node.name(), name);
            Ok(node.attributes())
        }
        ev => panic!("Unexpected event: {:?}", ev),
    }
}

fn expect_node_end<R: ParserSource + std::fmt::Debug>(
    parser: &mut Parser<R>,
) -> Result<(), Box<dyn std::error::Error>> {
    match parser.next_event()? {
        Event::EndNode => Ok(()),
        ev => panic!("Unexpected event: {:?}", ev),
    }
}

fn expect_fbx_end<R: ParserSource + std::fmt::Debug>(
    parser: &mut Parser<R>,
) -> Result<Result<Box<fbxcel::low::v7400::FbxFooter>, ParseError>, Box<dyn std::error::Error>> {
    match parser.next_event()? {
        Event::EndFbx(footer_res) => Ok(footer_res),
        ev => panic!("Unexpected event: {:?}", ev),
    }
}
