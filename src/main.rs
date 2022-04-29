#![feature(allocator_api)]
pub mod protos {
    include!(concat!(env!("OUT_DIR"), "/hyperstone.demo.rs"));
}
use byteorder::ByteOrder;
use byteorder::LittleEndian;
use bytes::Bytes;
use prost::Message;
use snap::raw::Decoder;
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
    let current_pos = get_file_info(&mut reader);
    println!("current {}", current_pos);
    reader.seek(SeekFrom::Start(current_pos)).unwrap();

    loop {
        let peek = read_tick(&mut reader);
        let demo_command = EDemoCommands::from_i32(peek.message_type as i32).unwrap();
        match demo_command {
            EDemoCommands::DemError => {
                println!("Error tick");
                break;
            }
            EDemoCommands::DemStop => {
                println!("End of file");
                break;
            }
            EDemoCommands::DemFileHeader => {
                println!("Header");
                let header = protos::CDemoFileHeader::decode(peek.message).unwrap();
                println!("map_name {}", header.map_name().to_string());
            }
            EDemoCommands::DemFileInfo => {
                println!("File Info");
                let file_info = protos::CDemoFileInfo::decode(peek.message).unwrap();
                println!("Playback time {}", file_info.playback_time.unwrap());
                println!("Playback ticks {}", file_info.playback_ticks.unwrap());
                println!("Playback frames {}", file_info.playback_frames.unwrap());
            }
            EDemoCommands::DemSyncTick => {
                println!("Sync Tick");
                let sync_tick = protos::CDemoSyncTick::decode(peek.message).unwrap();
            }
            EDemoCommands::DemSendTables => {
                println!("Send tables?");
                let send_tables = protos::CDemoSendTables::decode(peek.message).unwrap();
            }
            EDemoCommands::DemClassInfo => {
                // look into this
                println!("Class info");
                let class_info = protos::CDemoClassInfo::decode(peek.message).unwrap();
            }
            EDemoCommands::DemStringTables => {
                // most likely item information
                println!("String tables");
                let string_tables = protos::CDemoStringTables::decode(peek.message);
                match string_tables {
                    Ok(_) => println!("AAA"),
                    Err(error) => println!("decode error {}", error),
                }
            }
            EDemoCommands::DemPacket => {
                println!("Packet");
                let dem_packet = protos::CDemoPacket::decode(peek.message).unwrap();
            }
            EDemoCommands::DemSignonPacket => {
                // no proto
                println!("Signon packet");
            }
            EDemoCommands::DemConsoleCmd => {
                println!("Console command");
                let console_command = protos::CDemoConsoleCmd::decode(peek.message).unwrap();
            }
            EDemoCommands::DemCustomData => {
                println!("Custom Data");
                let custom_data = protos::CDemoCustomData::decode(peek.message).unwrap();
            }
            EDemoCommands::DemCustomDataCallbacks => {
                println!("Custom data call back");
                let callback = protos::CDemoCustomDataCallbacks::decode(peek.message).unwrap();
            }
            EDemoCommands::DemUserCmd => {
                println!("User command");
                let user_command = protos::CDemoUserCmd::decode(peek.message).unwrap();
            }
            EDemoCommands::DemFullPacket => {
                println!("Full packet");
                let full_packet = protos::CDemoFullPacket::decode(peek.message).unwrap();
            }
            EDemoCommands::DemSaveGame => {
                println!("Save game");
                let save_game = protos::CDemoSaveGame::decode(peek.message).unwrap();
            }
            EDemoCommands::DemSpawnGroups => {
                println!("Spawn groups");
                let spawn_groups = protos::CDemoSpawnGroups::decode(peek.message).unwrap();
            }
            EDemoCommands::DemMax => {
                println!("Max");
            }
            EDemoCommands::DemIsCompressedS1 => {
                println!("Compressed s1");
            }
            EDemoCommands::DemIsCompressedS2 => {
                println!("Compressed s2");
            }
        }
    }
}

struct Peek {
    tick: u32,
    message_type: u32,
    tell: u64,
    size: u32,
    message: Bytes,
}

fn get_file_info(reader: &mut BufReader<File>) -> u64 {
    let mut gio_buffer: Vec<u8, Global> = vec![0; 4];
    reader.read_exact(&mut gio_buffer).unwrap();

    let gio = LittleEndian::read_u32(&gio_buffer);

    let current_pos = reader.stream_position().unwrap();
    reader.seek(SeekFrom::Start(gio.into())).unwrap();

    let peek = read_tick(reader);

    let file_info = protos::CDemoFileInfo::decode(peek.message).unwrap();

    println!("Playback time {}", file_info.playback_time.unwrap());
    println!("Playback ticks {}", file_info.playback_ticks.unwrap());
    println!("Playback frames {}", file_info.playback_frames.unwrap());
    println!(
        "Match_id {}",
        file_info
            .game_info
            .as_ref()
            .unwrap()
            .dota
            .as_ref()
            .unwrap()
            .match_id
            .unwrap()
    );

    for player in file_info
        .game_info
        .as_ref()
        .unwrap()
        .dota
        .as_ref()
        .unwrap()
        .player_info
        .iter()
    {
        println!("{} {}", player.player_name(), player.hero_name());
    }

    current_pos
}

fn read_tick(reader: &mut BufReader<File>) -> Peek {
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
    let message = get_message(reader, size, compressed);

    let tell = reader.stream_position().unwrap(); // position of reader
    println!("Tell {}", tell);
    Peek {
        tick: tick,
        message_type: kind,
        tell: tell,
        size: size,
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
    println!("Message {:x?}", bytes);
    bytes
}

fn decompress(compressed: &Vec<u8>) -> Vec<u8> {
    let mut decoder = Decoder::new();
    decoder.decompress_vec(compressed).unwrap()
}
