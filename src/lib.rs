#![feature(allocator_api)]
#![feature(buf_read_has_data_left)]
#![feature(exclusive_range_pattern)]
use std::io::{BufReader, Read, SeekFrom, Seek};
use std::alloc::Global;
use std::str;
use std::time::Instant;
mod byte_utils;
mod demo_proto;
mod packet_proto;

use crate::demo_proto::parse_replay;


use bytes::Bytes;

pub struct Peek {
    pub _tick: u32,
    pub message_type: u32,
    pub _tell: u64,
    pub _size: u32,
    pub message: Bytes,
}



pub fn parse<R>(reader: &mut BufReader<R>) where R:Read, R:Seek{
    let mut buffer: Vec<u8, Global> = vec![0; 8];
    reader.read_exact(&mut buffer).unwrap();
    let demo_header = str::from_utf8(&buffer).unwrap();
    // Add check if header is a replay file 
    // go to byte 16
    reader.seek(SeekFrom::Start(16)).unwrap();
    let now = Instant::now();
    loop {
        if parse_replay(reader) == -1 {
            break;
        };
    }

}