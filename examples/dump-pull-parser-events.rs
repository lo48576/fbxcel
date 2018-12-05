use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use fbxcel::pull_parser;

fn main() {
    env_logger::init();

    let path = match std::env::args_os().nth(1) {
        Some(v) => PathBuf::from(v),
        None => {
            eprintln!("Usage: list-parser-events <FBX_FILE>");
            std::process::exit(1);
        }
    };
    let file = File::open(path).expect("Failed to open file");
    let mut reader = BufReader::new(file);

    let header =
        fbxcel::low::FbxHeader::read_fbx_header(&mut reader).expect("Failed to load FBX header");

    println!(
        "FBX version: {}.{}",
        header.version().major(),
        header.version().minor()
    );

    let parser_version = header.parser_version().expect("Unsupported FBX version");
    match parser_version {
        pull_parser::ParserVersion::V7400 => {
            let mut parser = pull_parser::v7400::from_seekable_reader(header, reader)
                .expect("Should never fail: Unsupported FBX verison");
            parser.set_warning_handler(|w| {
                eprintln!("WARNING: {}", w);
                Ok(())
            });
            dump_fbx_7400(parser).expect("Failed to parse FBX file");
        }
    }
}

fn indent(depth: usize) {
    print!("{:depth$}", "", depth = depth * 4);
}

fn dump_fbx_7400<R: pull_parser::ParserSource>(
    mut parser: pull_parser::v7400::Parser<R>,
) -> pull_parser::Result<()> {
    let mut depth = 0;

    /// Dump format of node attributes.
    enum AttrsDumpFormat {
        /// Type only.
        Type,
        /// Value for primitive types, length for array, binary, and string.
        Length,
        /// Values for all types.
        ///
        /// Not recommended because the output might be quite large.
        Full,
    }

    let attrs_dump_format = match std::env::var("DUMP_ATTRIBUTES").as_ref().map(AsRef::as_ref) {
        Ok("length") => AttrsDumpFormat::Length,
        Ok("full") => AttrsDumpFormat::Full,
        _ => AttrsDumpFormat::Type,
    };

    loop {
        use self::pull_parser::v7400::*;

        match parser.next_event()? {
            Event::StartNode(start) => {
                indent(depth);
                println!("Node start: {:?}", start.name());
                depth += 1;

                let attrs = start.attributes();
                match attrs_dump_format {
                    AttrsDumpFormat::Type => dump_v7400_attributes_type(depth, attrs)?,
                    AttrsDumpFormat::Length => dump_v7400_attributes_length(depth, attrs)?,
                    AttrsDumpFormat::Full => dump_v7400_attributes_full(depth, attrs)?,
                }
            }
            Event::EndNode => {
                depth -= 1;
                indent(depth);
                println!("Node end");
            }
            Event::EndFbx(footer_res) => {
                println!("FBX end");
                match footer_res {
                    Ok(footer) => println!("footer: {:#?}", footer),
                    Err(e) => println!("footer has an error: {:?}", e),
                }
                break;
            }
        }
    }

    Ok(())
}

fn dump_v7400_attributes_length<R>(
    depth: usize,
    mut attrs: pull_parser::v7400::Attributes<R>,
) -> pull_parser::Result<()>
where
    R: pull_parser::ParserSource,
{
    use self::pull_parser::v7400::attribute::{visitor::DirectVisitor, DirectAttributeValue};

    while let Some(attr) = attrs.visit_next(DirectVisitor)? {
        let type_ = attr.type_();
        indent(depth);
        match attr {
            DirectAttributeValue::Bool(_) => println!("Attribute: {:?}", attr),
            DirectAttributeValue::I16(_) => println!("Attribute: {:?}", attr),
            DirectAttributeValue::I32(_) => println!("Attribute: {:?}", attr),
            DirectAttributeValue::I64(_) => println!("Attribute: {:?}", attr),
            DirectAttributeValue::F32(_) => println!("Attribute: {:?}", attr),
            DirectAttributeValue::F64(_) => println!("Attribute: {:?}", attr),
            DirectAttributeValue::ArrBool(v) => {
                println!("Attribute: type={:?}, len={}", type_, v.len())
            }
            DirectAttributeValue::ArrI32(v) => {
                println!("Attribute: type={:?}, len={}", type_, v.len())
            }
            DirectAttributeValue::ArrI64(v) => {
                println!("Attribute: type={:?}, len={}", type_, v.len())
            }
            DirectAttributeValue::ArrF32(v) => {
                println!("Attribute: type={:?}, len={}", type_, v.len())
            }
            DirectAttributeValue::ArrF64(v) => {
                println!("Attribute: type={:?}, len={}", type_, v.len())
            }
            DirectAttributeValue::Binary(v) => {
                println!("Attribute: type={:?}, len={}", type_, v.len())
            }
            DirectAttributeValue::String(v) => {
                println!("Attribute: type={:?}, len={}", type_, v.len())
            }
        }
    }

    Ok(())
}

fn dump_v7400_attributes_type<R>(
    depth: usize,
    mut attrs: pull_parser::v7400::Attributes<R>,
) -> pull_parser::Result<()>
where
    R: pull_parser::ParserSource,
{
    use self::pull_parser::v7400::attribute::visitor::TypeVisitor;

    while let Some(type_) = attrs.visit_next(TypeVisitor).unwrap() {
        indent(depth);
        println!("Attribute: {:?}", type_);
    }

    Ok(())
}

fn dump_v7400_attributes_full<R>(
    depth: usize,
    mut attrs: pull_parser::v7400::Attributes<R>,
) -> pull_parser::Result<()>
where
    R: pull_parser::ParserSource,
{
    use self::pull_parser::v7400::attribute::{visitor::DirectVisitor, DirectAttributeValue};

    while let Some(attr) = attrs.visit_next(DirectVisitor)? {
        let type_ = attr.type_();
        indent(depth);
        match attr {
            DirectAttributeValue::Bool(_) => println!("Attribute: {:?}", attr),
            DirectAttributeValue::I16(_) => println!("Attribute: {:?}", attr),
            DirectAttributeValue::I32(_) => println!("Attribute: {:?}", attr),
            DirectAttributeValue::I64(_) => println!("Attribute: {:?}", attr),
            DirectAttributeValue::F32(_) => println!("Attribute: {:?}", attr),
            DirectAttributeValue::F64(_) => println!("Attribute: {:?}", attr),
            DirectAttributeValue::ArrBool(v) => println!(
                "Attribute: type={:?}, len={}, value={:?}",
                type_,
                v.len(),
                v
            ),
            DirectAttributeValue::ArrI32(v) => println!(
                "Attribute: type={:?}, len={}, value={:?}",
                type_,
                v.len(),
                v
            ),
            DirectAttributeValue::ArrI64(v) => println!(
                "Attribute: type={:?}, len={}, value={:?}",
                type_,
                v.len(),
                v
            ),
            DirectAttributeValue::ArrF32(v) => println!(
                "Attribute: type={:?}, len={}, value={:?}",
                type_,
                v.len(),
                v
            ),
            DirectAttributeValue::ArrF64(v) => println!(
                "Attribute: type={:?}, len={}, value={:?}",
                type_,
                v.len(),
                v
            ),
            DirectAttributeValue::Binary(v) => println!(
                "Attribute: type={:?}, len={}, value={:?}",
                type_,
                v.len(),
                v
            ),
            DirectAttributeValue::String(v) => println!(
                "Attribute: type={:?}, len={}, value={:?}",
                type_,
                v.len(),
                v
            ),
        }
    }

    Ok(())
}
