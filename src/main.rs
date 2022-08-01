#![feature(allocator_api)]
#![feature(buf_read_has_data_left)]
#![feature(exclusive_range_pattern)]
use std::alloc::Global;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::str;
use std::time::Instant;
use tracing::debug;
use tracing::info;

mod byte_utils;
mod demo_proto;
mod packet_proto;
use crate::demo_proto::get_file_info;

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
    let f = File::open("dota.dem").unwrap();
    let mut reader = BufReader::new(f);
    
}
