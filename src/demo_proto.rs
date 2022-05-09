use byteorder::{ByteOrder, LittleEndian};
use bytes::Bytes;
use prost::Message;
use prost_types::Any;
use tracing::{debug, info};

use crate::{
    byte_utils,
    protos::{
        CDemoClassInfo, CDemoCustomData, CDemoCustomDataCallbacks, CDemoFileHeader, CDemoFileInfo,
        CDemoFullPacket, CDemoPacket, CDemoSaveGame, CDemoSendTables, CDemoSpawnGroups,
        CDemoStringTables, CDemoSyncTick, CDemoUserCmd, EDemoCommands, self, 
    },
};
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
    let demo_command = EDemoCommands::from_i32(peek.message_type as i32).unwrap();
    match demo_command {
        EDemoCommands::DemError => {
            debug!("Error tick");
        }
        EDemoCommands::DemStop => {
            debug!("End of Demo");
        }
        EDemoCommands::DemFileHeader => {
            debug!("Header");
            // not a lot of interesting information in here, mostly meta data about the demo file.
            // let header = protos::CDemoFileHeader::decode(peek.message).unwrap();
        }
        EDemoCommands::DemFileInfo => {
            debug!("File Info");
            // ref get_file_info for how to investigate, information about players and meta data about the match.
            // let file_info = protos::CDemoFileInfo::decode(peek.message).unwrap();
        }
        EDemoCommands::DemSyncTick => {
            debug!("Sync Tick");
            // nothing interesting in here right now.
        }
        EDemoCommands::DemSendTables => {
            // something to be parsed
            debug!("Send tables");
            let send_tables = protos::CDemoSendTables::decode(peek.message).unwrap();
            let cool_bytes = Bytes::from(send_tables.data.unwrap());
            let message = protos::CsvcMsgFlattenedSerializer::decode(cool_bytes).unwrap();
            // let cap = reader.capacity();
            // info!("Table size {}", cap);
            // let buf_size = byte_utils::read_varint(&mut reader).unwrap();
            // let mut buf: Vec<u8, Global> = vec![0; buf_size.try_into().unwrap()];
            // reader.read_exact(&mut buf);
            // let cool = Any {
            //     type_url: "path/hyperstone.dota_netmessages.CsvcMsgFlattenedSerializer".to_string(),
            //     value: buf
            // };
            // let mut cool_buf: Vec<u8> = vec![];
            // docs don't explain how to deserialize https://docs.rs/prost-types/latest/prost_types/struct.Any.html
            // look into this https://github.com/fdeantoni/prost-wkt
        }
        EDemoCommands::DemClassInfo => {
            debug!("Class info");
            // let class_info = protos::CDemoClassInfo::decode(peek.message).unwrap();
            // let holder = class_info.classes;
            // for cool in holder {
            //     info!("Network name {}", cool.network_name());
            //     info!("Table name {}", cool.table_name());
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
            // let string_tables = protos::CDemoStringTables::decode(peek.message).unwrap();
            // let holder = string_tables.tables;
            // for hold in holder {
            //     info!("Table name {}", hold.table_name());
            //     for hol in hold.items {
            //         info!("String {}", hol.str());
            //         info!("Bytes {:?}", hol.data());

            //     }
            // }
        }
        EDemoCommands::DemPacket => {
            debug!("Packet");
            let dem_packet = protos::CDemoPacket::decode(peek.message).unwrap();
        }
        EDemoCommands::DemSignonPacket => {
            // no proto
            debug!("Signon packet");
        }
        EDemoCommands::DemConsoleCmd => {
            debug!("Console command");
            let console_command = protos::CDemoConsoleCmd::decode(peek.message).unwrap();
        }
        EDemoCommands::DemCustomData => {
            debug!("Custom Data");
            let custom_data = protos::CDemoCustomData::decode(peek.message).unwrap();
        }
        EDemoCommands::DemCustomDataCallbacks => {
            debug!("Custom data call back");
            let callback = protos::CDemoCustomDataCallbacks::decode(peek.message).unwrap();
        }
        EDemoCommands::DemUserCmd => {
            debug!("User command");
            let user_command = protos::CDemoUserCmd::decode(peek.message).unwrap();
        }
        EDemoCommands::DemFullPacket => {
            debug!("Full packet");
            let full_packet = protos::CDemoFullPacket::decode(peek.message).unwrap();
        }
        EDemoCommands::DemSaveGame => {
            debug!("Save game");
            let save_game = protos::CDemoSaveGame::decode(peek.message).unwrap();
        }
        EDemoCommands::DemSpawnGroups => {
            debug!("Spawn groups");
            let spawn_groups = protos::CDemoSpawnGroups::decode(peek.message).unwrap();
        }
        EDemoCommands::DemMax => {
            debug!("Max");
        }
        EDemoCommands::DemIsCompressedS1 => {
            debug!("Compressed s1");
        }
        EDemoCommands::DemIsCompressedS2 => {
            debug!("Compressed s2");
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

#[derive(Clone, Debug)]
pub enum DemoMessage {
    DemError(),
    DemStop(),
    DemFileHeader(CDemoFileHeader),
    DemFileInfo(CDemoFileInfo),
    DemSyncTick(CDemoSyncTick),
    DemSendTables(CDemoSendTables),
    DemClassInfo(CDemoClassInfo),
    DemStringTables(CDemoStringTables),
    DemPacket(CDemoPacket),
    DemSignonPacket(),
    DemConsoleCmd(),
    DemCustomData(CDemoCustomData),
    DemCustomDataCallbacks(CDemoCustomDataCallbacks),
    DemUserCmd(CDemoUserCmd),
    DemFullPacket(CDemoFullPacket),
    DemSaveGame(CDemoSaveGame),
    DemSpawnGroups(CDemoSpawnGroups),
    DemMax(),
    DemIsCompressedS1(),
    DemIsCompressedS2(),
}
