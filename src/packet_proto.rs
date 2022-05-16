use std::io::{BufRead, BufReader, Read, Seek};

use hyperstone_proto::dota_proto::*;

use tracing::debug;

use crate::byte_utils::{get_message, read_varint, Peek};

pub fn parse_packet<R>(reader: &mut BufReader<R>)
where
    R: Read,
{
    let mut messages: Vec<Peek> = vec![];
    while reader.has_data_left().unwrap() {
        messages.push(read_packet_segment(reader));
        // next sort messages based on demo_packet.go OMEGALUL
    }
    for message in messages {

        decode_packet(message); 
        
    }


}

pub fn decode_packet(message: Peek ) {

    let message_type = message.message_type as i32; 
    match message_type { 
        0..14 => {todo!() },
        40..73 => {todo!()},
        101..153 => {todo!()},
        200..213 => {todo!()},
        464..613 => {todo!()},
        i32::MIN..=-1_i32 | 13_i32..=i32::MAX => todo!(),
       

    }
    // let demo_command = EDemoCommands::from_i32(message.message_type as i32).unwrap();

    // network base types + dota_netmessages + usermessages + game events + dota_usermessages

    // NetMessages 0-13
    // SvcMessages 40-72
    // EBaseUserMessages 101-152 ;)
    // EBaseGameEvents 200-212 // check the go code again 
    // EDotaUserMessages 464 - 612

}

pub fn read_packet_segment<R>(reader: &mut BufReader<R>) -> Peek
where
    R: Read,
{
    let mut kind = read_varint(reader).unwrap(); // might need to be smaller t := int32(r.readUBitVar())
    debug!("kind {}", kind);

    let size = read_varint(reader).unwrap();
    debug!("size {}", size);

    let message = get_message(reader, size, false);

    Peek {
        _tick: 0,
        message_type: kind,
        _tell: 0,
        _size: size,
        message: message,
    }
}
