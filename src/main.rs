#![feature(allocator_api)]
use byteorder::ByteOrder;
use byteorder::LittleEndian;

use std::alloc::Global;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::str;
use std::time::Instant;
use tracing::debug;
use tracing::info;
mod protos {
    include!(concat!(env!("OUT_DIR"), "/hyperstone.demo.rs"));
}
mod byte_utils;
mod demo_proto;
use crate::demo_proto::get_file_info;
use crate::demo_proto::parse;
use crate::protos::EDemoCommands;

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
    let f = File::open("dota.dem").unwrap();
    let mut reader = BufReader::new(f);
    let mut buffer: Vec<u8, Global> = vec![0; 8];
    reader.read_exact(&mut buffer).unwrap();
    let demo_header = str::from_utf8(&buffer).unwrap();
    debug!("Header {}", demo_header);
    let current_pos = get_file_info(&mut reader) + 4; // go to byte 16
    debug!("current {}", current_pos);
    reader.seek(SeekFrom::Start(current_pos)).unwrap();
    let now = Instant::now();
    loop {
        if parse(&mut reader) == -1 {
            break;
        };
    }
    info!("Elapsed {}", now.elapsed().as_secs());
}
