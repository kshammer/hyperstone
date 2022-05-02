#![feature(allocator_api)]
use byteorder::ByteOrder;
use byteorder::LittleEndian;
use prost::Message;

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
    let mut count: HashMap<EDemoCommands, u32> = HashMap::new();
    loop {
        let peek = byte_utils::read_segment(&mut reader);
        let demo_command = EDemoCommands::from_i32(peek.message_type as i32).unwrap();
        match demo_command {
            EDemoCommands::DemError => {
                *count.entry(EDemoCommands::DemError).or_insert(0) += 1;
                debug!("Error tick");
                break;
            }
            EDemoCommands::DemStop => {
                *count.entry(EDemoCommands::DemStop).or_insert(0) += 1;
                debug!("End of file");
                break;
            }
            EDemoCommands::DemFileHeader => {
                *count.entry(EDemoCommands::DemFileHeader).or_insert(0) += 1;
                debug!("Header");
                let header = protos::CDemoFileHeader::decode(peek.message).unwrap();
                // println!("map_name {}", header.map_name().to_string());
            }
            EDemoCommands::DemFileInfo => {
                *count.entry(EDemoCommands::DemFileInfo).or_insert(0) += 1;
                debug!("File Info");
                let file_info = protos::CDemoFileInfo::decode(peek.message).unwrap();
            }
            EDemoCommands::DemSyncTick => {
                *count.entry(EDemoCommands::DemSyncTick).or_insert(0) += 1;
                debug!("Sync Tick");
                let sync_tick = protos::CDemoSyncTick::decode(peek.message).unwrap();
            }
            EDemoCommands::DemSendTables => {
                *count.entry(EDemoCommands::DemSendTables).or_insert(0) += 1;
                debug!("Send tables");
                let send_tables = protos::CDemoSendTables::decode(peek.message).unwrap();
            }
            EDemoCommands::DemClassInfo => {
                *count.entry(EDemoCommands::DemClassInfo).or_insert(0) += 1;
                debug!("Class info");
                let class_info = protos::CDemoClassInfo::decode(peek.message).unwrap();
            }
            EDemoCommands::DemStringTables => {
                *count.entry(EDemoCommands::DemStringTables).or_insert(0) += 1;
                debug!("String tables");
                let string_tables = protos::CDemoStringTables::decode(peek.message).unwrap();
            }
            EDemoCommands::DemPacket => {
                *count.entry(EDemoCommands::DemPacket).or_insert(0) += 1;
                debug!("Packet");
                let dem_packet = protos::CDemoPacket::decode(peek.message).unwrap();
            }
            EDemoCommands::DemSignonPacket => {
                *count.entry(EDemoCommands::DemSignonPacket).or_insert(0) += 1;
                // no proto
                debug!("Signon packet");
            }
            EDemoCommands::DemConsoleCmd => {
                *count.entry(EDemoCommands::DemConsoleCmd).or_insert(0) += 1;
                debug!("Console command");
                let console_command = protos::CDemoConsoleCmd::decode(peek.message).unwrap();
            }
            EDemoCommands::DemCustomData => {
                *count.entry(EDemoCommands::DemCustomData).or_insert(0) += 1;
                debug!("Custom Data");
                let custom_data = protos::CDemoCustomData::decode(peek.message).unwrap();
            }
            EDemoCommands::DemCustomDataCallbacks => {
                *count
                    .entry(EDemoCommands::DemCustomDataCallbacks)
                    .or_insert(0) += 1;
                debug!("Custom data call back");
                let callback = protos::CDemoCustomDataCallbacks::decode(peek.message).unwrap();
            }
            EDemoCommands::DemUserCmd => {
                *count.entry(EDemoCommands::DemUserCmd).or_insert(0) += 1;
                debug!("User command");
                let user_command = protos::CDemoUserCmd::decode(peek.message).unwrap();
            }
            EDemoCommands::DemFullPacket => {
                *count.entry(EDemoCommands::DemFullPacket).or_insert(0) += 1;
                debug!("Full packet");
                let full_packet = protos::CDemoFullPacket::decode(peek.message).unwrap();
            }
            EDemoCommands::DemSaveGame => {
                *count.entry(EDemoCommands::DemSaveGame).or_insert(0) += 1;
                debug!("Save game");
                let save_game = protos::CDemoSaveGame::decode(peek.message).unwrap();
            }
            EDemoCommands::DemSpawnGroups => {
                *count.entry(EDemoCommands::DemSpawnGroups).or_insert(0) += 1;
                debug!("Spawn groups");
                let spawn_groups = protos::CDemoSpawnGroups::decode(peek.message).unwrap();
            }
            EDemoCommands::DemMax => {
                *count.entry(EDemoCommands::DemMax).or_insert(0) += 1;
                debug!("Max");
            }
            EDemoCommands::DemIsCompressedS1 => {
                *count.entry(EDemoCommands::DemIsCompressedS1).or_insert(0) += 1;
                debug!("Compressed s1");
            }
            EDemoCommands::DemIsCompressedS2 => {
                *count.entry(EDemoCommands::DemIsCompressedS2).or_insert(0) += 1;
                debug!("Compressed s2");
            }
        }
    }
    info!("Elapsed {}", now.elapsed().as_secs());
    for (key, value) in &count {
        info!("{:?}: {}", key, value);
    }
}



fn get_file_info(reader: &mut BufReader<File>) -> u64 {
    let mut gio_buffer: Vec<u8, Global> = vec![0; 4];
    reader.read_exact(&mut gio_buffer).unwrap();

    let gio = LittleEndian::read_u32(&gio_buffer);

    let current_pos = reader.stream_position().unwrap();
    reader.seek(SeekFrom::Start(gio.into())).unwrap();

    let peek = byte_utils::read_segment(reader);

    let file_info = protos::CDemoFileInfo::decode(peek.message).unwrap();

    debug!("Playback time {}", file_info.playback_time.unwrap());
    debug!("Playback ticks {}", file_info.playback_ticks.unwrap());
    debug!("Playback frames {}", file_info.playback_frames.unwrap());
    debug!(
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
        debug!("{} {}", player.player_name(), player.hero_name());
    }

    current_pos
}

