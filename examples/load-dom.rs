use std::{fs::File, io::BufReader, path::PathBuf};

use fbxcel::{dom, tree::any::AnyTree};

pub fn main() {
    env_logger::init();

    let path = match std::env::args_os().nth(1) {
        Some(v) => PathBuf::from(v),
        None => {
            eprintln!("Usage: load-dom <FBX_FILE>");
            std::process::exit(1);
        }
    };
    let file = File::open(path).expect("Failed to open file");
    let reader = BufReader::new(file);

    match AnyTree::from_seekable_reader(reader).expect("Failed to load tree") {
        AnyTree::V7400(tree, _footer) => {
            let doc = dom::v7400::Loader::new()
                .load_from_tree(tree)
                .expect("Failed to load FBX DOM");
            println!("Loaded FBX DOM successfully");
            for scene in doc.scenes() {
                println!("Scene object: object_id={:?}", scene.object_id());
                let root_id = scene
                    .root_object_id()
                    .expect("Failed to get root object ID");
                println!("\tRoot object ID: {:?}", root_id);
            }
        }
        _ => panic!("FBX version unsupported by this example"),
    }
}
