use std::{fs::File, io::BufReader, path::PathBuf};

use fbxcel::{
    pull_parser::any::{from_seekable_reader, AnyParser},
    tree,
};

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
    let reader = BufReader::new(file);

    match from_seekable_reader(reader).expect("Failed to create parser") {
        AnyParser::V7400(mut parser) => {
            let version = parser.fbx_version();
            println!("FBX version: {}.{}", version.major(), version.minor());
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
        parser => panic!(
            "Unsupported by this example: fbx_version={:?}",
            parser.fbx_version()
        ),
    }
}
