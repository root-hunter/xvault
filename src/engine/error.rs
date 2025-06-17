use std::io;

#[derive(Debug)]
pub enum XVaultError {
    FileNotExists,
    VolumeAlreadyAllocated,
    IO(io::Error),
    Encode(bincode::error::EncodeError),
    Decode(bincode::error::DecodeError),
}