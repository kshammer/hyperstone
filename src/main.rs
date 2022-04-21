#![feature(allocator_api)]
pub mod items {
    include!(concat!(env!("OUT_DIR"), "/hyperstone.demo.rs"));
}
use std::alloc::Global;
use std::fs;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::path::Path;
use std::str;

use bytes::Buf;

fn main() {


    let f = File::open("dota.dem").unwrap();
    let mut reader = BufReader::new(f);
    let mut buffer: Vec<u8, Global> = vec![0; 8];
    reader.read(&mut buffer).unwrap();
    let demo_header = str::from_utf8(&buffer).unwrap();
    println!("{}", demo_header);



    let mut gio_buffer: Vec<u8, Global> = vec![0; 4];
    reader.read(&mut gio_buffer).unwrap(); // python  (b'qI\x9e\x04')  // rust [71, 49, 9E, 04] // looks close enough
    println!("hex {:02X?}", gio_buffer);
    let gio = (0..4).fold(0u32, |sum, i| {
        println!("i {}", i);
        println!("buff {}", gio_buffer[i]);
        let out = u32::from(gio_buffer[i]) << (i * 8);
        println!("out {}", out);
        out + sum
    }); // 77482353
    println!("sum {}", gio);

    let current_pos = reader.stream_position().unwrap();
    println!("current {}", current_pos);
    reader.seek(SeekFrom::Start(gio.into()));
}
