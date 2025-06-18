use crate::engine::{error::XEngineError, volume::ChunkOffset};
use bincode::config::{Configuration, LittleEndian};
use uuid::Uuid;

const UID_LEN: usize = 16; // UUID size in bytes

pub type Number = u64;

pub fn get_bincode_coinfig() -> Configuration<LittleEndian, bincode::config::Fixint> {
    return bincode::config::standard()
        .with_fixed_int_encoding()
        .with_little_endian();
}

pub fn parse_number(
    buf: &[u8],
    config: &Configuration<LittleEndian, bincode::config::Fixint>,
) -> Result<Number, XEngineError> {
    let value = bincode::decode_from_slice(buf, *config);

    if let Err(e) = value {
        return Err(XEngineError::Decode(e));
    }

    return Ok(value.unwrap().0);
}

pub fn parse_uuid(buf: [u8; UID_LEN]) -> Uuid {
    return Uuid::from_bytes_le(buf);
}

pub fn parse_uuid_to_string(buf: [u8; UID_LEN]) -> String {
    return parse_uuid(buf).to_string();
}

pub struct ParseOffsetMapElem {
    pub uid: String,
    pub offset: ChunkOffset,
}

pub fn parse_offset_map_elem(
    buf: &[u8],
    config: Configuration<LittleEndian, bincode::config::Fixint>  
) -> Result<ParseOffsetMapElem, XEngineError> {
    let index = UID_LEN as usize;

    let chunk_uid_bytes = &buf[..index];
    let uid = parse_uuid_to_string(chunk_uid_bytes.try_into().unwrap());

    let chunk_start_bytes = &buf[index..(index + 8)];
    let chunk_start = parse_number(chunk_start_bytes, &config)?;

    let chunk_end_bytes = &buf[(index + 8)..(index + 16)];
    let chunk_end = parse_number(chunk_end_bytes, &config)?;

    let offset = ChunkOffset {
        start: chunk_start,
        end: chunk_end,
    };

    return Ok(ParseOffsetMapElem { uid, offset });
}
