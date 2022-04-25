#![feature(allocator_api)]
pub mod protos {
    include!(concat!(env!("OUT_DIR"), "/hyperstone.demo.rs"));
}
use byteorder::ByteOrder;
use byteorder::LittleEndian;
use bytes::Buf;
use bytes::Bytes;
use prost::Message;
use snap::read::FrameDecoder;
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
    reader.read_exact(&mut buffer).unwrap();
    let demo_header = str::from_utf8(&buffer).unwrap();
    println!("{}", demo_header);

    let mut gio_buffer: Vec<u8, Global> = vec![0; 4];
    reader.read_exact(&mut gio_buffer).unwrap();

    let gio = LittleEndian::read_u32(&gio_buffer);
    println!("gio {}", gio);

    let _current_pos = reader.stream_position().unwrap();
    // println!("current {}", current_pos);
    reader.seek(SeekFrom::Start(gio.into())).unwrap();

    let peek = read(&mut reader);

    // verify compressed data 
    let mut buf:Vec<u8, Global> = vec![0; peek.size.try_into().unwrap()];
    FrameDecoder::new(peek.message.reader()).read_exact(&mut buf).unwrap();
    println!("{:?}", buf); 

    let file_info = protos::CDemoFileInfo::decode(Bytes::from(buf)).unwrap();
    println!("Playback time {}", file_info.playback_time.unwrap());
    println!("Playback ticks {}", file_info.playback_ticks.unwrap());
    println!("Playback frames {}", file_info.playback_frames.unwrap());
}

// fn parse(message: Bytes, message_type: usize) {}

struct Peek {
    tick: u32,
    message_type: u32,
    tell: u64,
    size: u32,
    message: Bytes,
    compression: bool,
}

fn read(reader: &mut BufReader<File>) -> Peek {
    let mut kind = read_varint(reader);
    let compressed = if kind & EDemoCommands::DemIsCompressedS2 as u32 == 0 {
        false
    } else {
        true
    }; // not equal is 0 equal is value
    println!("compressed {}", compressed);

    // deal with compression
    kind = if compressed {
        kind & !(EDemoCommands::DemIsCompressedS2 as u32)
    } else {
        kind
    };
    println!("kind {}", kind);

    let tick = read_varint(reader);
    let size = read_varint(reader);
    println!("tick {}", tick);
    println!("size {}", size);

    // skipping just reading message directly without checking type
    // if kind in IMPL_BY_KIND:
    //  message = self.io.read(size)
    let message = read_message(reader, size);
    println!("Message {:X?}", message);
    let tell = reader.stream_position().unwrap(); // position of reader
    println!("Tell {}", tell);
    Peek {
        tick: tick,
        message_type: kind,
        tell: tell,
        size: size,
        message: message,
        compression: compressed,
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

// uint32 ReadVarInt32( const std::string& buf, size_t& index )
// {
// 	uint32 b;
// 	int count = 0;
// 	uint32 result = 0;

// 	do
// 	{
// 		if ( count == 5 )
// 		{
// 			// If we get here it means that the fifth bit had its
// 			// high bit set, which implies corrupt data.
// 			assert( 0 );
// 			return result;
// 		}
// 		else if ( index >= buf.size() )
// 		{
// 			assert( 0 );
// 			return result;
// 		}

// 		b = buf[ index++ ];
// 		result |= ( b & 0x7F ) << ( 7 * count );
// 		++count;
// 	} while ( b & 0x80 );

// 	return result;
// }

// python version incorrect
// fn read_varint(reader: &mut BufReader<File>) -> u32 {
//     let mut size = 0;
//     let mut value = 0 as u32;
//     let mut shift = 0;
//     let vi_shift = 7;
//     let vi_mask = (1 << 32) - 1 as u32;
//     loop {
//         let mut varintbuf: Vec<u8, Global> = vec![0, 1];
//         let byte = reader.read(&mut varintbuf).unwrap();
//         size += 1;
//         value |= (varintbuf[0] as u32 & 0x7f) << shift;
//         shift += vi_shift;
//         if (varintbuf[0] & 0x80) == 0 {
//             value &= vi_mask;
//             return value.into();
//         }
//     }
// }

fn read_message(reader: &mut BufReader<File>, size: u32) -> Bytes {
    let mut message: Vec<u8, Global> = vec![0; size.try_into().unwrap()];
    reader.read_exact(&mut message).unwrap();
    Bytes::from(message)
}
