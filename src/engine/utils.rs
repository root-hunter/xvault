/*
Copyright (C) 2025 Antonio Ricciardi

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use std::str::FromStr;

use crate::engine::{error::XEngineError, volume::ChunkOffset};
use bincode::config::{Configuration, LittleEndian};
use uuid::Uuid;

const UID_LEN: usize = 16; // UUID size in bytes

pub type Number = u64;

pub fn get_bincode_config() -> Configuration<LittleEndian, bincode::config::Fixint> {
    return bincode::config::standard()
        .with_fixed_int_encoding()
        .with_little_endian();
}

pub fn decode_number(
    buf: &[u8],
    config: &Configuration<LittleEndian, bincode::config::Fixint>,
) -> Result<Number, XEngineError> {

    let value = bincode::decode_from_slice(buf, *config);

    if let Err(e) = value {
        return Err(XEngineError::Decode(e));
    }
    let value = value.unwrap().0;

    //println!("Decoding: {:?} = {}", buf, value);

    return Ok(value);
}

pub fn encode_number(
    value: Number,
    config: Configuration<LittleEndian, bincode::config::Fixint>,
) -> Result<Vec<u8>, XEngineError> {
    let number_emcoded= bincode::encode_to_vec(value, config)
        .map_err(XEngineError::Encode)
        .unwrap();
    //println!("Encoding: {:?} = {}", number_emcoded, value);

    return Ok(number_emcoded);
}

pub fn decode_uuid(buf: [u8; UID_LEN]) -> Uuid {
    return Uuid::from_bytes_le(buf);
}

pub fn encode_uuid(
    uid: Uuid,
) -> Result<[u8; 16], XEngineError> {
    let buf = uid.to_bytes_le();
    
    //println!("Encoding UUID: {:?} = {}", buf, uid);

    return Ok(buf);
}

pub fn decode_uuid_to_string(buf: [u8; UID_LEN]) -> String {
    let uid = decode_uuid(buf).to_string(); 
    
    //println!("Decoding UUID: {:?} = {}", buf, uid);

    return uid;
}

pub fn encode_uuid_from_string(uid: String) -> Result<[u8; 16], XEngineError> {
    let uid = Uuid::from_str(&uid).unwrap();
    let buf = uid.to_bytes_le();

    //println!("Encoding UUID: {:?} = {}", buf, uid);
    
    return Ok(buf);
}

pub struct ParseOffsetMapElem {
    pub uid: String,
    pub offset: ChunkOffset,
}

pub fn parse_offset_map_elem(
    buf: &[u8],
    config: Configuration<LittleEndian, bincode::config::Fixint>,
) -> Result<ParseOffsetMapElem, XEngineError> {
    let index = UID_LEN as usize;

    let chunk_uid_bytes = &buf[..index];
    let uid = decode_uuid_to_string(chunk_uid_bytes.try_into().unwrap());

    let chunk_start_bytes = &buf[index..(index + 8)];
    let chunk_start = decode_number(chunk_start_bytes, &config)?;

    let chunk_end_bytes = &buf[(index + 8)..(index + 16)];
    let chunk_end = decode_number(chunk_end_bytes, &config)?;

    let offset = ChunkOffset {
        start: chunk_start,
        end: chunk_end,
    };

    return Ok(ParseOffsetMapElem { uid, offset });
}
