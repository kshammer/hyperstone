pub mod items {
    include!(concat!(env!("OUT_DIR"), "/hyperstone.demo.rs"));
}

fn main() {
    println!("Hello, world!");
}