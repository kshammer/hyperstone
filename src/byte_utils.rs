use snap::raw::Decoder;
use std::alloc::Global;
use std::io::{BufReader, Seek, Read};
use std::fs::File;
use bytes::Bytes;
use tracing::debug;

use crate::protos::EDemoCommands;

pub struct Peek {
    pub _tick: u32,
    pub message_type: u32,
    pub _tell: u64,
    pub _size: u32,
    pub message: Bytes,
}

pub fn read_segment(reader: &mut BufReader<File>) -> Peek {
    let mut kind = read_varint(reader);
    let compressed = if kind & EDemoCommands::DemIsCompressedS2 as u32 == 0 {
        false
    } else {
        true
    }; // not equal is 0 equal is value
    debug!("compressed {}", compressed);

    // deal with compression
    kind = if compressed {
        kind & !(EDemoCommands::DemIsCompressedS2 as u32)
    } else {
        kind
    };
    debug!("kind {}", kind);

    let tick = read_varint(reader);
    let size = read_varint(reader);
    debug!("tick {}", tick);
    debug!("size {}", size);

    let message = get_message(reader, size, compressed);

    let tell = reader.stream_position().unwrap(); // position of reader
    debug!("Tell {}", tell);
    Peek {
        _tick: tick,
        message_type: kind,
        _tell: tell,
        _size: size,
        message: message,
    }
}

// Also add the error handling
// might be correct based on valve version
fn read_varint(reader: &mut BufReader<File>) -> u32 {
    let mut count = 0;
    let mut result = 0 as u32;
    loop {
        let mut varintbuf: Vec<u8, Global> = vec![0; 1];
        reader.read_exact(&mut varintbuf).unwrap();
        result |= (varintbuf[0] as u32 & 0x7f) << (7 * count);
        count += 1;
        if varintbuf[0] & 0x80 == 0 {
            return result.into();
        }
    }
}

fn get_message(reader: &mut BufReader<File>, size: u32, compressed: bool) -> Bytes {
    let mut message: Vec<u8, Global> = vec![0; size.try_into().unwrap()];
    reader.read_exact(&mut message).unwrap();
    if compressed {
        message = decompress(&message);
    }
    let bytes = Bytes::from(message);
    bytes
}

fn decompress(compressed: &Vec<u8>) -> Vec<u8> {
    let mut decoder = Decoder::new();
    decoder.decompress_vec(compressed).unwrap()
}
