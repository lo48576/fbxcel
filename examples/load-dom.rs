use std::{fs::File, io::BufReader, path::PathBuf};

use fbxcel::{
    dom,
    pull_parser::any::{from_seekable_reader, AnyParser},
};

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

    match from_seekable_reader(reader).expect("Failed to create parser") {
        AnyParser::V7400(mut parser) => {
            let version = parser.fbx_version();
            println!("FBX version: {}.{}", version.major(), version.minor());
            parser.set_warning_handler(|w, pos| {
                eprintln!("WARNING: {} (pos={:?})", w, pos);
                Ok(())
            });
            let doc = dom::v7400::Loader::new()
                .load_from_parser(&mut parser)
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
        parser => panic!(
            "Unsupported by this example: fbx_version={:?}",
            parser.fbx_version()
        ),
    }
}
