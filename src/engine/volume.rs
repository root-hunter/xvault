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

pub use bincode::{Decode, Encode};
use serde::de::value;
pub use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::OpenOptions, io::Write, os::unix::fs::FileExt};
pub use std::{
    fs::{self, File},
    io::{self, Read},
    path::Path,
};
pub use uuid::Uuid;

use crate::engine::{chunk::{Chunk, ChunkHandler}, volume};

const OFFSET_VOLUME_UID: u64 = 0;
const OFFSET_MAX_SIZE: u64 = OFFSET_VOLUME_UID + 16;
const OFFSET_ACTUAL_SIZE: u64 = OFFSET_MAX_SIZE + 8;

//pub type VolumeChunkOffset = [u8; 2];
pub type ChunkOffset = [u8; 2];
pub type VolumeOffsets = HashMap<String, ChunkOffset>;
pub type VolumeChunks = HashMap<String, Chunk>;

#[derive(Debug)]
pub enum Error {
    FileNotExists,
    VolumeAlreadyAllocated,
    IO(io::Error),
    Encode(bincode::error::EncodeError),
    Decode(bincode::error::DecodeError),
}

#[derive(Decode, Encode)]
struct VolumeWrap {
    #[bincode(with_serde)]
    pub data: Volume,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Volume {
    pub uid: String,
    pub path: String,
    pub chunks: VolumeChunks,
    pub offsets: VolumeOffsets,
    pub max_size: u64,
}

impl Default for Volume {
    fn default() -> Self {
        Self {
            uid: Default::default(),
            path: Default::default(),
            max_size: Default::default(),
            chunks: Default::default(),
            offsets: Default::default(),
        }
    }
}

impl Volume {
    pub fn new() -> Self {
        return Self::default();
    }

    pub fn set_uid_from_device(&mut self, device_uid: String) -> &mut Self {
        assert!(!device_uid.is_empty(), "Device UID cannot be empty");

        let path_str = self.path.clone();
        assert!(!path_str.is_empty(), "Volume path cannot be empty");

        let device_uid = Uuid::parse_str(&device_uid).unwrap();
        let volume_uid = Uuid::new_v5(&device_uid, path_str.as_bytes());

        self.uid = volume_uid.to_string();
        return self;
    }

    pub fn set_uid(&mut self, volume_uid: String) -> &mut Self {
        assert!(!volume_uid.is_empty(), "Device UID cannot be empty");
        self.uid = volume_uid;
        return self;
    }

    pub fn set_uid_from_disk(&mut self) -> Result<(), Error> {
        let file = OpenOptions::new().read(true).open(&self.path);

        if let Err(err) = file {
            return Err(Error::IO(err));
        }

        let file = file.unwrap();
        let mut buf = [0u8; 16];
        if let Err(err) = file.read_exact_at(&mut buf, OFFSET_VOLUME_UID) {
            return Err(Error::IO(err));
        }

        let volume_uid = Uuid::from_bytes_le(buf);
        let volume_uid = volume_uid.to_string();

        self.set_uid(volume_uid);
        return Ok(());
    }

    pub fn set_path(&mut self, path: String) -> &mut Self {
        let path = Path::new(&path);
        if !path.is_absolute() {
            let path = fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf());
            let path = path.to_str().unwrap_or("").to_string();

            self.path = path;
        }
        return self;
    }

    pub fn set_max_size(&mut self, max_size: u64) -> &mut Self {
        self.max_size = max_size;
        return self;
    }

    pub fn set_max_size_from_disk(&mut self) -> Result<(), Error> {
        let file = OpenOptions::new().read(true).open(&self.path);

        if let Err(err) = file {
            return Err(Error::IO(err));
        }

        let file = file.unwrap();
        let mut buf = [0u8; 8]; // u64 size
        if let Err(err) = file.read_exact_at(&mut buf, OFFSET_MAX_SIZE) {
            return Err(Error::IO(err));
        }

        let config = bincode::config::standard()
            .with_little_endian()
            .with_fixed_int_encoding();
        let max_size: u64 = bincode::decode_from_slice(&buf, config)
            .map_err(Error::Decode)
            .unwrap()
            .0;

        self.set_max_size(max_size);

        return Ok(());
    }

    pub fn build(&mut self) -> Result<&mut Self, io::Error> {
        assert!(self.max_size > 0, "Volume max_size cannot be 0");
        assert!(!self.path.is_empty(), "Volume path cannot be empty");
        assert!(!self.uid.is_empty(), "Volume uid cannot be empty");

        //self.create_on_disk().unwrap();

        return Ok(self);
    }

    pub fn get_max_size(&self) -> usize {
        return self.chunks.len();
    }

    pub fn write_map_offsets(&mut self) -> Result<(), Error> {
        let path_str = self.path.clone();
        let exists = fs::exists(path_str.clone());

        if exists.is_err() {
            return Err(Error::IO(exists.unwrap_err()));
        } else {
            if exists.unwrap() {
                let res = OpenOptions::new().read(true).write(true).open(path_str);

                if let Err(err) = res {
                    return Err(Error::IO(err));
                }

                let mut file = res.unwrap();
                let config = bincode::config::standard()
                    .with_little_endian()
                    .with_fixed_int_encoding();

                for (uid, offset) in &self.offsets {
                    let uid_bytes = bincode::encode_to_vec(uid, config).map_err(Error::Encode)?;
                    let offset_bytes =
                        bincode::encode_to_vec(offset, config).map_err(Error::Encode)?;

                    file.write_all(&uid_bytes).unwrap();
                    file.write_all(&offset_bytes).unwrap();
                }

                return Ok(());
            } else {
                return Err(Error::FileNotExists);
            }
        }
    }

    pub fn alloc_on_disk(&mut self) -> Result<(), Error> {
        let path_str = self.path.clone();
        let exists = fs::exists(path_str.clone());

        if exists.is_err() {
            return Err(Error::IO(exists.unwrap_err()));
        } else {
            let path = Path::new(&path_str);
            if !path.is_absolute() {
                let path =
                    fs::canonicalize(path_str.clone()).unwrap_or_else(|_| path.to_path_buf());
                let path = path.to_str().unwrap_or("").to_string();

                self.set_path(path);
            }

            if !exists.unwrap() {
                let res = OpenOptions::new()
                    .read(true)
                    .write(true)
                    .create(true)
                    .open(path);

                if let Err(err) = res {
                    return Err(Error::IO(err));
                }


                let mut file = res.unwrap();
                let config = bincode::config::standard()
                    .with_little_endian()
                    .with_fixed_int_encoding();

                let volume_uid = Uuid::parse_str(&self.uid).unwrap();
                let volume_uid = volume_uid.to_bytes_le();

                file.write_all(&volume_uid).unwrap();

                let max_size =
                    bincode::encode_to_vec(&self.max_size, config).map_err(Error::Encode)?;

                let actual_size = self.get_max_size();
                let actual_size =
                    bincode::encode_to_vec(&actual_size, config).map_err(Error::Encode)?;

                println!("max_size length: {}", max_size.len());
                println!("actual_size length: {}", actual_size.len());

                file.write_all(&max_size).unwrap();
                file.write_all(&actual_size).unwrap();

                // Chunks(10) = 8 + 8 + (10 × 32) + (10 × 4096) = 41296

                // ~ 320:1 volume file ratio for each 320 GB of data 1 GB of volume file is overhead

                let map_offsets_elem = [255u8; 16 + 16]; // (UID [u8; 16] (16 bytes) + offset{start u64 (8 bytes) + end u64 (8 bytes)})
                for i in 0..self.max_size {
                    file.write_all(&map_offsets_elem).unwrap();
                }

                let chunk_elem = [255u8; 4096];
                for i in 0..self.max_size {
                    file.write_all(&chunk_elem).unwrap();
                }

                return Ok(());
            } else {
                return Err(Error::VolumeAlreadyAllocated);
            }
        }
    }

    fn exists(&mut self) -> Result<bool, Error> {
        let exists = fs::exists(self.path.clone());

        if let Ok(exists) = exists {
            return Ok(exists);
        } else {
            return Err(Error::IO(exists.unwrap_err()));
        }
    }
}

impl ChunkHandler for Volume {
    fn get_chunk(&mut self, uuid: String) -> Option<&Chunk> {
        // TODO Lettura del chunk in base all'offset
        return self.chunks.get(&uuid);
    }

    fn add_chunk(&mut self, chunk: Chunk) -> Option<String> {
        // TODO Inserimento nel giusto offset

        self.chunks.insert(chunk.uid.clone(), chunk);
        return Some(self.uid.clone());
    }

    fn is_full(self) -> bool {
        return self.chunks.len() >= self.max_size as usize;
    }
}
