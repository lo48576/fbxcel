//! Tests for writer, tree, and parser.
#![cfg(all(feature = "tree", feature = "writer"))]

use std::{cell::RefCell, io::Cursor, iter, rc::Rc};

use fbxcel::{
    low::FbxVersion,
    pull_parser::{
        any::{from_seekable_reader, AnyParser},
        error::Warning,
    },
};

use self::v7400::writer::{
    expect_fbx_end, expect_node_end, expect_node_start, CUSTOM_UNKNOWN1, MAGIC, UNKNOWN3,
};

mod v7400;

/// Parses a node which lacks necessary node end marker.
#[test]
fn missing_node_end_marker() -> Result<(), Box<dyn std::error::Error>> {
    let data = {
        let raw_ver = 7400_u32;
        let mut vec = Vec::new();
        // Header.
        {
            // Magic.
            vec.extend(MAGIC);
            // Version.
            vec.extend(raw_ver.to_le_bytes());
        }
        // Nodes.
        {
            // Container node.
            {
                const CONTAINER: &[u8] = b"Container";
                let container_start = vec.len();
                // End offset.
                vec.extend([0; 4]);
                // Number of node properties.
                vec.extend([0; 4]);
                // Length of node properties in bytes.
                vec.extend([0; 4]);
                // Node name length.
                vec.push(CONTAINER.len() as u8);
                // Node name.
                vec.extend(CONTAINER);

                // Invalid node.
                {
                    const INVALID_NODE: &[u8] = b"InvalidNode";
                    let invalid_node_start = vec.len();
                    // End offset.
                    vec.extend([0; 4]);
                    // Number of node properties.
                    vec.extend([0; 4]);
                    // Length of node properties in bytes.
                    vec.extend([0; 4]);
                    // Node name length.
                    vec.push(INVALID_NODE.len() as u8);
                    // Node name.
                    vec.extend(INVALID_NODE);
                    let end_pos = (vec.len() as u32).to_le_bytes();
                    vec[invalid_node_start..(invalid_node_start + 4)].copy_from_slice(&end_pos);
                }

                // Node end marker.
                vec.extend([0; 13]);
                let end_pos = (vec.len() as u32).to_le_bytes();
                vec[container_start..(container_start + 4)].copy_from_slice(&end_pos);
            }
            // End of implicit root.
            {
                vec.extend(iter::repeat(0).take(4 * 3 + 1));
            }
        }
        // Footer.
        {
            // Footer: unknown1.
            vec.extend(CUSTOM_UNKNOWN1);
            // Footer: padding.
            {
                let len = vec.len().wrapping_neg() % 16;
                assert_eq!((vec.len() + len) % 16, 0);
                vec.extend(iter::repeat(0).take(len));
            }
            // Footer: unknown2.
            vec.extend([0; 4]);
            // Footer: FBX version.
            vec.extend(raw_ver.to_le_bytes());
            // Footer: 120 zeroes.
            vec.extend(iter::repeat(0).take(120));
            // Footer: unknown3.
            vec.extend(UNKNOWN3);
        }
        vec
    };

    assert_eq!(data.len() % 16, 0);

    let mut parser = match from_seekable_reader(Cursor::new(data))? {
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
        let attrs = expect_node_start(&mut parser, "Container")?;
        assert_eq!(attrs.total_count(), 0);
    }
    {
        let attrs = expect_node_start(&mut parser, "InvalidNode")?;
        assert_eq!(attrs.total_count(), 0);
    }
    expect_node_end(&mut parser)?;
    expect_node_end(&mut parser)?;

    let _: Box<fbxcel::low::v7400::FbxFooter> = expect_fbx_end(&mut parser)??;

    match &warnings.borrow()[..] {
        [Warning::MissingNodeEndMarker] => {}
        v => panic!("Unexpected warnings: {:?}", v),
    }

    Ok(())
}

/// Parses a node which has extra node end marker.
#[test]
fn extra_node_end_marker() -> Result<(), Box<dyn std::error::Error>> {
    let data = {
        let raw_ver = 7400_u32;
        let mut vec = Vec::new();
        // Header.
        {
            // Magic.
            vec.extend(MAGIC);
            // Version.
            vec.extend(raw_ver.to_le_bytes());
        }
        // Nodes.
        {
            // Container node.
            {
                const CONTAINER: &[u8] = b"Container";
                let container_start = vec.len();
                // End offset.
                vec.extend([0; 4]);
                // Number of node properties.
                vec.extend([0; 4]);
                // Length of node properties in bytes.
                vec.extend([0; 4]);
                // Node name length.
                vec.push(CONTAINER.len() as u8);
                // Node name.
                vec.extend(CONTAINER);

                // Invalid node.
                {
                    const INVALID_NODE: &[u8] = b"InvalidNode";
                    let invalid_node_start = vec.len();
                    // End offset.
                    vec.extend([0; 4]);
                    // Number of node properties.
                    vec.extend(1_u32.to_le_bytes());
                    // Length of node properties in bytes.
                    vec.extend(2_u32.to_le_bytes());
                    // Node name length.
                    vec.push(INVALID_NODE.len() as u8);
                    // Node name.
                    vec.extend(INVALID_NODE);
                    // An attribute.
                    vec.extend([b'C', b'T']);
                    // Extra node end marker.
                    vec.extend([0; 13]);
                    let end_pos = (vec.len() as u32).to_le_bytes();
                    vec[invalid_node_start..(invalid_node_start + 4)].copy_from_slice(&end_pos);
                }

                // Node end marker.
                vec.extend([0; 13]);
                let end_pos = (vec.len() as u32).to_le_bytes();
                vec[container_start..(container_start + 4)].copy_from_slice(&end_pos);
            }
            // End of implicit root.
            {
                vec.extend(iter::repeat(0).take(4 * 3 + 1));
            }
        }
        // Footer.
        {
            // Footer: unknown1.
            vec.extend(CUSTOM_UNKNOWN1);
            // Footer: padding.
            {
                let len = vec.len().wrapping_neg() % 16;
                assert_eq!((vec.len() + len) % 16, 0);
                vec.extend(iter::repeat(0).take(len));
            }
            // Footer: unknown2.
            vec.extend([0; 4]);
            // Footer: FBX version.
            vec.extend(raw_ver.to_le_bytes());
            // Footer: 120 zeroes.
            vec.extend(iter::repeat(0).take(120));
            // Footer: unknown3.
            vec.extend(UNKNOWN3);
        }
        vec
    };

    assert_eq!(data.len() % 16, 0);

    let mut parser = match from_seekable_reader(Cursor::new(data))? {
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
        let attrs = expect_node_start(&mut parser, "Container")?;
        assert_eq!(attrs.total_count(), 0);
    }
    {
        let attrs = expect_node_start(&mut parser, "InvalidNode")?;
        assert_eq!(attrs.total_count(), 1);
    }
    expect_node_end(&mut parser)?;
    expect_node_end(&mut parser)?;

    let _: Box<fbxcel::low::v7400::FbxFooter> = expect_fbx_end(&mut parser)??;

    match &warnings.borrow()[..] {
        [Warning::ExtraNodeEndMarker] => {}
        v => panic!("Unexpected warnings: {:?}", v),
    }

    Ok(())
}
