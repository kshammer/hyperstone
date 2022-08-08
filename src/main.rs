#![feature(allocator_api)]
#![feature(buf_read_has_data_left)]
#![feature(exclusive_range_pattern)]
use std::fs::File;
use std::io::BufReader;

use hyperstone::parse;

mod byte_utils;
mod demo_proto;
mod packet_proto;

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
    let f = File::open("dota.dem").unwrap();
    let mut reader = BufReader::new(f);
    parse(& mut reader);
    
}
