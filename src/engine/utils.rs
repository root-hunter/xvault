use crate::engine::error::XEngineError;
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

pub fn parse_uuid(
    buf: [u8; UID_LEN],
) -> Uuid {
    return Uuid::from_bytes_le(buf);
}

pub fn parse_uuid_to_string(
    buf: [u8; UID_LEN],
) -> String {
    return parse_uuid(buf).to_string();
}