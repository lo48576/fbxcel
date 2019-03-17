use std::{fs::File, io::BufReader, path::PathBuf};

use fbxcel::{pull_parser, tree};

pub fn main() {
    env_logger::init();

    let path = match std::env::args_os().nth(1) {
        Some(v) => PathBuf::from(v),
        None => {
            eprintln!("Usage: load-tree <FBX_FILE>");
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
            let tree_loader = tree::v7400::Loader::new();
            let (tree, footer) = tree_loader
                .load(&mut parser)
                .expect("Failed to load FBX data tree");
            println!("tree = {:#?}", tree);
            println!("footer = {:#?}", footer);
        }
    }
}
