use std::{fs::File, io::BufReader, path::PathBuf};

use fbxcel::pull_parser;

fn main() {
    env_logger::init();

    let path = match std::env::args_os().nth(1) {
        Some(v) => PathBuf::from(v),
        None => {
            eprintln!("Usage: dump-pull-parser-events <FBX_FILE>");
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
            parser.set_warning_handler(|w, pos| {
                eprintln!("WARNING: {} (pos={:?})", w, pos);
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
    mut attrs: pull_parser::v7400::Attributes<'_, R>,
) -> pull_parser::Result<()>
where
    R: pull_parser::ParserSource,
{
    use fbxcel::{
        low::v7400::AttributeValue, pull_parser::v7400::attribute::loaders::DirectLoader,
    };

    while let Some(attr) = attrs.load_next(DirectLoader)? {
        let type_ = attr.type_();
        indent(depth);
        match attr {
            AttributeValue::Bool(_) => println!("Attribute: {:?}", attr),
            AttributeValue::I16(_) => println!("Attribute: {:?}", attr),
            AttributeValue::I32(_) => println!("Attribute: {:?}", attr),
            AttributeValue::I64(_) => println!("Attribute: {:?}", attr),
            AttributeValue::F32(_) => println!("Attribute: {:?}", attr),
            AttributeValue::F64(_) => println!("Attribute: {:?}", attr),
            AttributeValue::ArrBool(v) => println!("Attribute: type={:?}, len={}", type_, v.len()),
            AttributeValue::ArrI32(v) => println!("Attribute: type={:?}, len={}", type_, v.len()),
            AttributeValue::ArrI64(v) => println!("Attribute: type={:?}, len={}", type_, v.len()),
            AttributeValue::ArrF32(v) => println!("Attribute: type={:?}, len={}", type_, v.len()),
            AttributeValue::ArrF64(v) => println!("Attribute: type={:?}, len={}", type_, v.len()),
            AttributeValue::Binary(v) => println!("Attribute: type={:?}, len={}", type_, v.len()),
            AttributeValue::String(v) => println!("Attribute: type={:?}, len={}", type_, v.len()),
        }
    }

    Ok(())
}

fn dump_v7400_attributes_type<R>(
    depth: usize,
    mut attrs: pull_parser::v7400::Attributes<'_, R>,
) -> pull_parser::Result<()>
where
    R: pull_parser::ParserSource,
{
    use self::pull_parser::v7400::attribute::loaders::TypeLoader;

    while let Some(type_) = attrs.load_next(TypeLoader).unwrap() {
        indent(depth);
        println!("Attribute: {:?}", type_);
    }

    Ok(())
}

fn dump_v7400_attributes_full<R>(
    depth: usize,
    mut attrs: pull_parser::v7400::Attributes<'_, R>,
) -> pull_parser::Result<()>
where
    R: pull_parser::ParserSource,
{
    use fbxcel::{
        low::v7400::AttributeValue, pull_parser::v7400::attribute::loaders::DirectLoader,
    };

    while let Some(attr) = attrs.load_next(DirectLoader)? {
        let type_ = attr.type_();
        indent(depth);
        match attr {
            AttributeValue::Bool(_) => println!("Attribute: {:?}", attr),
            AttributeValue::I16(_) => println!("Attribute: {:?}", attr),
            AttributeValue::I32(_) => println!("Attribute: {:?}", attr),
            AttributeValue::I64(_) => println!("Attribute: {:?}", attr),
            AttributeValue::F32(_) => println!("Attribute: {:?}", attr),
            AttributeValue::F64(_) => println!("Attribute: {:?}", attr),
            AttributeValue::ArrBool(v) => println!(
                "Attribute: type={:?}, len={}, value={:?}",
                type_,
                v.len(),
                v
            ),
            AttributeValue::ArrI32(v) => println!(
                "Attribute: type={:?}, len={}, value={:?}",
                type_,
                v.len(),
                v
            ),
            AttributeValue::ArrI64(v) => println!(
                "Attribute: type={:?}, len={}, value={:?}",
                type_,
                v.len(),
                v
            ),
            AttributeValue::ArrF32(v) => println!(
                "Attribute: type={:?}, len={}, value={:?}",
                type_,
                v.len(),
                v
            ),
            AttributeValue::ArrF64(v) => println!(
                "Attribute: type={:?}, len={}, value={:?}",
                type_,
                v.len(),
                v
            ),
            AttributeValue::Binary(v) => println!(
                "Attribute: type={:?}, len={}, value={:?}",
                type_,
                v.len(),
                v
            ),
            AttributeValue::String(v) => println!(
                "Attribute: type={:?}, len={}, value={:?}",
                type_,
                v.len(),
                v
            ),
        }
    }

    Ok(())
}
