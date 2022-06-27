use std::io::{BufReader, Read};

use bytes::Bytes;

pub struct Peek {
    pub _tick: u32,
    pub message_type: u32,
    pub _tell: u64,
    pub _size: u32,
    pub message: Bytes,
}



pub fn parse<R>(reader: &mut BufReader<R>) where R:Read{
    
}