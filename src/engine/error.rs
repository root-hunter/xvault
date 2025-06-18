use std::io;

#[derive(Debug)]
pub enum XEngineError {
    FileNotExists,
    VolumeAlreadyAllocated,
    InvalidUuid,
    IO(io::Error),
    Encode(bincode::error::EncodeError),
    Decode(bincode::error::DecodeError),
}