#![cfg(feature = "writer")]

use fbxcel::pull_parser::{
    v7400::{Attributes, Event, Parser},
    Error as ParseError, ParserSource,
};

pub const MAGIC: &[u8] = b"Kaydara FBX Binary  \x00\x1a\x00";

pub const CUSTOM_UNKNOWN1: [u8; 16] = [
    0xff, 0xbe, 0xad, 0x0c, 0xdb, 0xca, 0xd9, 0x68, 0xb7, 0x76, 0xf5, 0x84, 0x13, 0xf2, 0x21, 0x70,
];

pub const UNKNOWN3: [u8; 16] = [
    0xf8, 0x5a, 0x8c, 0x6a, 0xde, 0xf5, 0xd9, 0x7e, 0xec, 0xe9, 0x0c, 0xe3, 0x75, 0x8f, 0x29, 0x0b,
];

pub fn expect_node_start<'a, R: ParserSource + std::fmt::Debug>(
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

pub fn expect_node_end<R: ParserSource + std::fmt::Debug>(
    parser: &mut Parser<R>,
) -> Result<(), Box<dyn std::error::Error>> {
    match parser.next_event()? {
        Event::EndNode => Ok(()),
        ev => panic!("Unexpected event: {:?}", ev),
    }
}

pub fn expect_fbx_end<R: ParserSource + std::fmt::Debug>(
    parser: &mut Parser<R>,
) -> Result<Result<Box<fbxcel::low::v7400::FbxFooter>, ParseError>, Box<dyn std::error::Error>> {
    match parser.next_event()? {
        Event::EndFbx(footer_res) => Ok(footer_res),
        ev => panic!("Unexpected event: {:?}", ev),
    }
}
