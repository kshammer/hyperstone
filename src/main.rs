#![feature(allocator_api)]
pub mod protos {
    include!(concat!(env!("OUT_DIR"), "/hyperstone.demo.rs"));
}
use std::alloc::Global;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::str;


use crate::protos::EDemoCommands;

fn main() {


    let f = File::open("dota.dem").unwrap();
    let mut reader = BufReader::new(f);
    let mut buffer: Vec<u8, Global> = vec![0; 8];
    reader.read(&mut buffer).unwrap();
    let demo_header = str::from_utf8(&buffer).unwrap();
    println!("{}", demo_header);



    let mut gio_buffer: Vec<u8, Global> = vec![0; 4];
    reader.read(&mut gio_buffer).unwrap(); // python  (b'qI\x9e\x04')  // rust [71, 49, 9E, 04] // looks close enough
    // println!("hex {:02X?}", gio_buffer);
    let gio = (0..4).fold(0u32, |sum, i| {
        // println!("i {}", i);
        // println!("buff {}", gio_buffer[i]);
        let out = u32::from(gio_buffer[i]) << (i * 8);
        // println!("out {}", out);
        out + sum
    }); // 77482353
    println!("sum {}", gio);




    let current_pos = reader.stream_position().unwrap();
    // println!("current {}", current_pos);
    let _res = reader.seek(SeekFrom::Start(gio.into())).unwrap();
    
    let kind = read_varint(&mut reader);
    println!("kind {}", kind); 
    let compressed = if kind & EDemoCommands::DemIsCompressedS2 as usize == 0 {false} else {true} ; // not equal is 0 equal is value
    println!("compressed {}", compressed); 
    // kind = (kind & ~pb_d.DEM_IsCompressed) if comp else kind  // todo
    let tick = read_varint(&mut reader);
    let size = read_varint(&mut reader);
    println!("tick {}", tick);
    println!("size {}", size); 

    // skipping just reading message directly without checking type 
    // if kind in IMPL_BY_KIND:
    //  message = self.io.read(size)
    let message = read_message(&mut reader, size);
    let tell = reader.stream_position().unwrap();  // tell 

}

fn read_varint(reader: &mut BufReader<File>) -> usize{
    let mut size = 0;
    let mut value = 0; 
    let mut shift = 0; 
    let vi_shift = 7;
    let vi_mask = (1 << 32) - 1;
    loop {
        let mut varintbuf: Vec<u8, Global> = vec![0, 1];
        let byte = reader.read(&mut varintbuf).unwrap();  
        size += 1; 
        value |= (byte & 0x7f) << shift; 
        shift += vi_shift; 
        if (byte & 0x80) == 0{
            value &= vi_mask; 
            return value; 
        }
    }

}

fn read_message(reader: &mut BufReader<File>, size: usize) -> Vec<u8> {
    let mut message: Vec<u8, Global> = vec![0, size.try_into().unwrap()]; 
    let byte = reader.read(&mut message).unwrap();
    message 
}