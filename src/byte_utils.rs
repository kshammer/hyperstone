use bytes::Bytes;
use snap::raw::Decoder;
use std::alloc::Global;
use std::fs::File;
use std::io::{BufReader, Read, Seek};
use tracing::debug;

use hyperstone_proto::dota_proto::*;

pub struct Peek {
    pub _tick: u32,
    pub message_type: u32,
    pub _tell: u64,
    pub _size: u32,
    pub message: Bytes,
}

pub struct ParseError {
    pub error: String,
}

pub fn read_segment(reader: &mut BufReader<File>) -> Result<Peek, ParseError> {
    let mut kind = match read_varint(reader) {
        Ok(val) => val,
        Err(_) => {
            return Err(ParseError {
                error: "Error maybe end of file ?".to_string(),
            })
        }
    };
    let compressed = if kind & EDemoCommands::DemIsCompressed as u32 == 0 {
        false
    } else {
        true
    }; // not equal is 0 equal is value
    debug!("compressed {}", compressed);

    // deal with compression
    kind = if compressed {
        kind & !(EDemoCommands::DemIsCompressed as u32)
    } else {
        kind
    };
    debug!("kind {}", kind);

    let tick = match read_varint(reader) {
        Ok(val) => val,
        Err(_) => {
            return Err(ParseError {
                error: "Error maybe end of file ?".to_string(),
            })
        }
    };
    let size = match read_varint(reader) {
        Ok(val) => val,
        Err(_) => {
            return Err(ParseError {
                error: "Error maybe end of file ?".to_string(),
            })
        }
    };
    debug!("tick {}", tick);
    debug!("size {}", size);

    let message = get_message(reader, size, compressed);

    let tell = reader.stream_position().unwrap(); // position of reader
    debug!("Tell {}", tell);
    Ok(Peek {
        _tick: tick,
        message_type: kind,
        _tell: tell,
        _size: size,
        message: message,
    })
}

// Also add the error handling
// might be correct based on valve version
pub fn read_varint<R>(reader: &mut BufReader<R>) -> Result<u32, i32>
where
    R: std::io::Read,
{
    let mut count = 0;
    let mut result = 0 as u32;
    loop {
        let mut varintbuf: Vec<u8, Global> = vec![0; 1];
        match reader.read_exact(&mut varintbuf) {
            Ok(_) => {
                result |= (varintbuf[0] as u32 & 0x7f) << (7 * count);
                count += 1;
                if varintbuf[0] & 0x80 == 0 {
                    return Ok(result.into());
                }
            }
            Err(_) => {
                debug!("End of file ");
                return Err(-1);
            }
        }
    }
}

// Also add the error handling
// might be correct based on valve version
pub fn read_signvarint<R>(reader: &mut BufReader<R>) -> Result<i32, i32>
where
    R: std::io::Read,
{
    let mut count = 0;
    let mut result = 0 as i32;
    loop {
        let mut varintbuf: Vec<u8, Global> = vec![0; 1];
        match reader.read_exact(&mut varintbuf) {
            Ok(_) => {
                result |= (varintbuf[0] as i32 & 0x7f) << (7 * count);
                count += 1;
                if varintbuf[0] & 0x80 == 0 {
                    return Ok(result.into());
                }
            }
            Err(_) => {
                debug!("End of file ");
                return Err(-1);
            }
        }
    }
}

pub fn get_message<R>(reader: &mut BufReader<R>, size: u32, compressed: bool) -> Bytes
where
    R: std::io::Read,
{
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
