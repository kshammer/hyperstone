use byteorder::{ByteOrder, LittleEndian};
use bytes::{Buf, Bytes};
use hyperstone_proto::dota_proto::*;
use prost::Message;
use tracing::{debug, info};

use crate::{byte_utils, packet_proto::parse_packet};
use std::{
    alloc::Global,
    fs::File,
    io::{BufReader, Read, Seek, SeekFrom},
};

pub fn parse(reader: &mut BufReader<File>) -> i32 {
    let peek = match byte_utils::read_segment(reader) {
        Ok(peek) => peek,
        Err(_) => return -1,
    };
    debug!("tick {}", peek._tick);
    debug!("Message {}", peek.message_type);
    let demo_command = EDemoCommands::from_i32(peek.message_type as i32).unwrap();
    match demo_command {
        EDemoCommands::DemError => {
            debug!("Error tick");
        }
        EDemoCommands::DemStop => {
            debug!("End of Demo");
        }
        EDemoCommands::DemFileHeader => {
            debug!("File Header");
            // not a lot of interesting information in here, mostly meta data about the demo file.
            // let header =  CDemoFileHeader::decode(peek.message).unwrap();
        }
        EDemoCommands::DemFileInfo => {
            debug!("File Info");
            // ref get_file_info for how to investigate, information about players and meta data about the match.
            // let file_info =  CDemoFileInfo::decode(peek.message).unwrap();
        }
        EDemoCommands::DemSyncTick => {
            debug!("Sync Tick");
            // nothing interesting in here right now.
        }
        EDemoCommands::DemSendTables => {
            // something to be parsed
            debug!("Send tables");
            let send_tables = CDemoSendTables::decode(peek.message).unwrap();
            let cool_bytes = Bytes::from(send_tables.data.unwrap());
            // read one varint off the bytes
            let mut reader = BufReader::new(cool_bytes.reader());
            let size = byte_utils::read_varint(&mut reader).unwrap();
            let pog = byte_utils::get_message(&mut reader, size, false);
            let message = CsvcMsgFlattenedSerializer::decode(pog).unwrap();
            // for symbol in message.symbols {
            //    println!("{}", symbol);
            // }
        }
        EDemoCommands::DemClassInfo => {
            debug!("Class info");
            // let class_info =  CDemoClassInfo::decode(peek.message).unwrap();
            // let holder = class_info.classes;
            // for cool in holder {
            //     debug!("Network name {}", cool.network_name());
            //     debug!("Table name {}", cool.table_name());
            // }
            // names of entities in the demo
            //  INFO hyperstone::demo_proto: Table name
            //  INFO hyperstone::demo_proto: Network name CDOTA_Unit_Hero_Sniper
            //  INFO hyperstone::demo_proto: Table name
            //  INFO hyperstone::demo_proto: Network name CDOTA_Unit_Hero_Spectre
            //  INFO hyperstone::demo_proto: Table name
            //  INFO hyperstone::demo_proto: Network name CDOTA_Unit_Hero_SpiritBreaker
            //  INFO hyperstone::demo_proto: Table name
        }
        EDemoCommands::DemStringTables => {
            debug!("String tables");
            // let string_tables =  CDemoStringTables::decode(peek.message).unwrap();
            // let holder = string_tables.tables;
            // for hold in holder {
            //     debug!("Table name {}", hold.table_name());
            //     for hol in hold.items {
            //         debug!("String {}", hol.str());
            //         debug!("Bytes {:?}", hol.data());

            //     }
            // }
        }
        EDemoCommands::DemPacket => {
            info!("Packet");
            info!("tick {}", peek._tick);
            info!("size {}", peek._size);
            let dem_packet = CDemoPacket::decode(peek.message).unwrap();
            let cool_bytes = Bytes::from(dem_packet.data.unwrap());
            info!("{:?}", cool_bytes.len());
            let mut reader = BufReader::new(cool_bytes.reader());
            parse_packet(&mut reader);
        }
        EDemoCommands::DemSignonPacket => {
            // no proto
            debug!("Signon packet");
        }
        EDemoCommands::DemConsoleCmd => {
            debug!("Console command");
            let console_command = CDemoConsoleCmd::decode(peek.message).unwrap();
        }
        EDemoCommands::DemCustomData => {
            debug!("Custom Data");
            let custom_data = CDemoCustomData::decode(peek.message).unwrap();
        }
        EDemoCommands::DemCustomDataCallbacks => {
            debug!("Custom data call back");
            let callback = CDemoCustomDataCallbacks::decode(peek.message).unwrap();
        }
        EDemoCommands::DemUserCmd => {
            debug!("User command");
            let user_command = CDemoUserCmd::decode(peek.message).unwrap();
        }
        EDemoCommands::DemFullPacket => {
            debug!("Full packet");
            let full_packet = CDemoFullPacket::decode(peek.message).unwrap();
        }
        EDemoCommands::DemSaveGame => {
            debug!("Save game");
            let save_game = CDemoSaveGame::decode(peek.message).unwrap();
        }
        EDemoCommands::DemSpawnGroups => {
            debug!("Spawn groups");
            let spawn_groups = CDemoSpawnGroups::decode(peek.message).unwrap();
        }
        EDemoCommands::DemMax => {
            debug!("Max");
        }
        EDemoCommands::DemIsCompressed => {
            debug!("Compressed");
        }
    }
    return 1;
}

pub fn get_file_info(reader: &mut BufReader<File>) -> u64 {
    let mut gio_buffer: Vec<u8, Global> = vec![0; 4];
    reader.read_exact(&mut gio_buffer).unwrap();

    let gio = LittleEndian::read_u32(&gio_buffer);

    let current_pos = reader.stream_position().unwrap();
    reader.seek(SeekFrom::Start(gio.into())).unwrap();

    let peek = {
        let this = byte_utils::read_segment(reader);
        match this {
            Ok(t) => t,
            Err(e) => todo!(),
        }
    };

    let file_info = CDemoFileInfo::decode(peek.message).unwrap();

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
