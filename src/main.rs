pub mod items {
    include!(concat!(env!("OUT_DIR"), "/hyperstone.demo.rs"));
}
use std::path::Path;
use std::fs;
use std::str;


fn main() {
    let path = Path::new("dota.dem");
    let bytes = fs::read(path).unwrap().into_iter();
    let first = bytes.take(8).collect::<Vec<_>>();
    let demo_header = str::from_utf8(&first).unwrap();
    println!("{}", demo_header);

    // let gio = ;
    // 
    // iterate over every tick 
    // check if tick is synthetic 
    // raise based on events matching 
}