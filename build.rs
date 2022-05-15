use std::fs;
extern crate prost_build;

fn main() {
    let paths = fs::read_dir("src/proto")
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|e| e.is_file())
        .collect::<Vec<_>>();

    // println!("{:?}", paths);
    let out = prost_build::compile_protos(&paths, &["src/proto"]);

    match out {
        Ok(_) => {}
        Err(e) => {
            println!("{}", e);
        }
    }
}
