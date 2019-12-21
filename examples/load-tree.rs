use std::{fs::File, io::BufReader, path::PathBuf};

use fbxcel::tree::any::AnyTree;

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

    match AnyTree::from_seekable_reader(reader).expect("Failed to load tree") {
        AnyTree::V7400(fbx_version, tree, footer) => {
            println!("FBX version = {:#?}", fbx_version);
            println!("tree = {:#?}", tree);
            println!("footer = {:#?}", footer);
        }
        _ => panic!("FBX version unsupported by this example"),
    }
}
