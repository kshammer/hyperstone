use std::io::{BufReader, BufRead, Read, Seek};

use tracing::debug;

use crate::byte_utils::{Peek, read_varint, get_message};



pub fn parse_packet <R>(reader: &mut BufReader<R>) where R: Read, R:Seek {

    let mut messages:Vec<Peek> = vec![];
    while reader.has_data_left().unwrap() {
        messages.push(read_packet_segment(reader));
        // next sort messages based on demo_packet.go 
    }


}

pub fn match_packet(){
    // network base types + dota_netmessages + usermessages + game events + dota_usermessages
    // missing 72 look to update 
    // missing 152
    // 600 cases =( 
}


pub fn read_packet_segment <R>(reader: &mut BufReader<R>) -> Peek  where R: Read, R: Seek  {
    let mut kind = read_varint(reader).unwrap(); // might need to be smaller t := int32(r.readUBitVar())
    debug!("kind {}", kind);
    
    let size = read_varint(reader).unwrap();
    debug!("size {}", size);

    let message = get_message(reader, size, false);

    let tell = reader.stream_position().unwrap(); // position of reader
    debug!("Tell {}", tell);
    Peek {
        _tick: 0,
        message_type: kind,
        _tell: tell,
        _size: size,
        message: message,
    }
}