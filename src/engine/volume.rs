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
pub use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::OpenOptions, io::Write, os::unix::fs::FileExt};
pub use std::{
    fs::{self, File},
    io::{self, Read},
    path::Path,
};
pub use uuid::Uuid;

use crate::engine::{chunk::{self, Chunk, ChunkHandler}, error::XEngineError};

const OFFSET_VOLUME_UID: u64 = 0;
const OFFSET_MAX_SIZE: u64 = OFFSET_VOLUME_UID + 16;
const OFFSET_ACTUAL_SIZE: u64 = OFFSET_MAX_SIZE + 8;
const OFFSET_MAP_OFFSETS: u64 = OFFSET_ACTUAL_SIZE + 8;

//pub type VolumeChunkOffset = [u8; 2];
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ChunkOffset {
    pub start: u64,
    pub end: u64,
}

impl Default for ChunkOffset {
    fn default() -> Self {
        Self { start: 0, end: 0 }
    }
}

pub type VolumeOffsets = HashMap<String, ChunkOffset>;
pub type VolumeChunks = HashMap<String, Chunk>;

#[derive(Decode, Encode)]
struct VolumeWrap {
    #[bincode(with_serde)]
    pub data: Volume,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Volume {
    pub uid: String,
    pub max_size: u64,
    pub path: String,
    pub chunks: VolumeChunks,
    pub offsets: VolumeOffsets,
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

    pub fn build(&mut self) -> Result<&mut Self, XEngineError> {
        assert!(self.max_size > 0, "Volume max_size cannot be 0");
        assert!(!self.path.is_empty(), "Volume path cannot be empty");
        assert!(!self.uid.is_empty(), "Volume uid cannot be empty");

        //self.create_on_disk().unwrap();

        return Ok(self);
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

    pub fn read_uid_from_file(&mut self, file: &File) -> Result<String, XEngineError> {
        let mut buf = [0u8; 16];
        if let Err(err) = file.read_exact_at(&mut buf, OFFSET_VOLUME_UID) {
            return Err(XEngineError::IO(err));
        }

        let volume_uid = Uuid::from_bytes_le(buf);
        let volume_uid = volume_uid.to_string();
        return Ok(volume_uid);
    }

    pub fn set_uid_from_file(&mut self, file: &File) -> Result<(), XEngineError> {
        let volume_uid = self.read_uid_from_file(file)?;

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

    pub fn get_actual_size(&self) -> u64 {
        return self.chunks.len() as u64;
    }

    pub fn read_max_size_from_file(&mut self, file: &File) -> Result<u64, XEngineError> {
        let mut buf = [0u8; 8]; // u64 size
        if let Err(err) = file.read_exact_at(&mut buf, OFFSET_MAX_SIZE) {
            return Err(XEngineError::IO(err));
        }

        let config = bincode::config::standard()
            .with_little_endian()
            .with_fixed_int_encoding();
        let max_size: u64 = bincode::decode_from_slice(&buf, config)
            .map_err(XEngineError::Decode)
            .unwrap()
            .0;

        return Ok(max_size);
    }

    pub fn read_actual_size_from_file(&mut self, file: &File) -> Result<u64, XEngineError> {
        let mut buf = [0u8; 8]; // u64 size
        if let Err(err) = file.read_exact_at(&mut buf, OFFSET_ACTUAL_SIZE) {
            return Err(XEngineError::IO(err));
        }

        let config = bincode::config::standard()
            .with_little_endian()
            .with_fixed_int_encoding();
        let actual_size: u64 = bincode::decode_from_slice(&buf, config)
            .map_err(XEngineError::Decode)
            .unwrap()
            .0;

        return Ok(actual_size);
    }

    pub fn set_max_size(&mut self, max_size: u64) -> &mut Self {
        self.max_size = max_size;
        return self;
    }

    pub fn set_max_size_from_disk(&mut self, file: &File) -> Result<(), XEngineError> {
        let max_size = self.read_max_size_from_file(file)?;
        self.set_max_size(max_size);

        return Ok(());
    }

    pub fn alloc_on_disk(&mut self) -> Result<(), XEngineError> {
        let path_str = self.path.clone();
        let exists = fs::exists(path_str.clone());

        if exists.is_err() {
            return Err(XEngineError::IO(exists.unwrap_err()));
        } else {
            let path = Path::new(&path_str);
            if !path.is_absolute() {
                let path =
                    fs::canonicalize(path_str.clone()).unwrap_or_else(|_| path.to_path_buf());
                let path = path.to_str().unwrap_or("").to_string();

                self.set_path(path);
            }

            let exists = exists.unwrap();

            if !exists {
                let res = OpenOptions::new()
                    .read(true)
                    .write(true)
                    .create(true)
                    .open(path);

                if let Err(err) = res {
                    return Err(XEngineError::IO(err));
                }

                let mut file = res.unwrap();
                let config = bincode::config::standard()
                    .with_little_endian()
                    .with_fixed_int_encoding();

                let volume_uid = Uuid::parse_str(&self.uid).unwrap();
                let volume_uid = volume_uid.to_bytes_le();

                file.write_all(&volume_uid).unwrap();

                let max_size =
                    bincode::encode_to_vec(&self.max_size, config).map_err(XEngineError::Encode)?;

                let actual_size = self.get_actual_size();
                let actual_size =
                    bincode::encode_to_vec(&actual_size, config).map_err(XEngineError::Encode)?;

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
                return Err(XEngineError::VolumeAlreadyAllocated);
            }
        }
    }

    pub fn write_offsets_to_file(&self, file: &File) -> Result<(), XEngineError> {
        let config = bincode::config::standard()
            .with_little_endian()
            .with_fixed_int_encoding();

        let mut index = OFFSET_MAP_OFFSETS;
        let count = self.offsets.len() as u64;

        // Write the actual size of the offsets map
        let actual_size = bincode::encode_to_vec(count, config)
            .map_err(XEngineError::Encode)
            .unwrap();
        if let Err(err) = file.write_all_at(&actual_size, OFFSET_ACTUAL_SIZE) {
            return Err(XEngineError::IO(err));
        }


        for (uid, offset) in &self.offsets {
            let uid_bytes = Uuid::parse_str(uid).unwrap().to_bytes_le();
            let offset_start = bincode::encode_to_vec(offset.start, config)
                .map_err(XEngineError::Encode)
                .unwrap();
            let offset_end = bincode::encode_to_vec(offset.end, config)
                .map_err(XEngineError::Encode)
                .unwrap();

            if let Err(err) = file.write_all_at(&uid_bytes, index) {
                return Err(XEngineError::IO(err));
            }

            index += 16; // UID size

            if let Err(err) = file.write_all_at(&offset_start, index) {
                return Err(XEngineError::IO(err));
            }

            index += 8; // start u64 size

            if let Err(err) = file.write_all_at(&offset_end, index) {
                return Err(XEngineError::IO(err));
            }

            index += 8; // end u64 size
        }

        Ok(())
    }

    pub fn read_offsets_from_file(&mut self, file: &File) -> Result<VolumeOffsets, XEngineError> {
        let mut offsets = VolumeOffsets::new();
        
        let actual_size = self.read_actual_size_from_file(file)?;

        let mut index = OFFSET_MAP_OFFSETS;
        let mut chunk_uid_buf = [0u8; 16];
        let mut chunk_start = [0u8; 16];
        let mut chunk_end = [0u8; 16];

        let config = bincode::config::standard()
            .with_little_endian()
            .with_fixed_int_encoding();

        for i in 0..actual_size {
            file.read_at(&mut chunk_uid_buf, index).unwrap();

            index += 16; // UID size

            file.read_exact_at(&mut chunk_start, index).unwrap();
            index += 8; // u64 size

            file.read_exact_at(&mut chunk_end, index).unwrap();
            index += 8; // u64 size

            let chunk_uid = Uuid::from_bytes_le(chunk_uid_buf).to_string();
            let chunk_start: u64 = bincode::decode_from_slice(&chunk_start, config)
                .map_err(XEngineError::Decode)
                .unwrap()
                .0;

            let chunk_end: u64 = bincode::decode_from_slice(&chunk_end, config)
                .map_err(XEngineError::Decode)
                .unwrap()
                .0;

            offsets.insert(chunk_uid, ChunkOffset { start: chunk_start, end: chunk_end });
        }

        return Ok(offsets);
    }


    pub fn set_offsets_from_file(&mut self, file: &File) -> Result<(), XEngineError> {
        let offsets = self.read_offsets_from_file(file)?;
        self.offsets = offsets.clone();
        return Ok(());
    }
    
}

impl ChunkHandler for Volume {
    fn get_chunk(&mut self, uuid: String) -> Option<&Chunk> {
        // TODO Lettura del chunk in base all'offset
        return self.chunks.get(&uuid);
    }

    fn add_chunk(&mut self, chunk: Chunk) -> Option<String> {
        let chunk_uid = chunk.uid.clone();
        self.chunks.insert(chunk_uid, chunk);

        return Some(self.uid.clone());
    }

    fn is_full(self) -> bool {
        return self.chunks.len() >= self.max_size as usize;
    }

    fn get_chunk_v2(&mut self, file: &File, uuid: String) -> Result<Option<Chunk>, XEngineError> {
        let offset = self.offsets.get(&uuid);

        if offset.is_none() {
            return Ok(None);
        } else {
            let offset = offset.unwrap();
            let chunk_len = offset.end - offset.start;
            let mut buf = vec![0u8; chunk_len as usize];
            file.read_at(buf.as_mut_slice(), offset.start).unwrap();

            let buf_len = buf.len();
            let chunk = Chunk {
                uid: uuid,
                data: buf,
                length: Some(chunk_len as usize),
            };

            return Ok(Some(chunk));
        }
    }

    fn add_chunk_v2(&mut self, file: &File, chunk: Chunk) -> Result<Option<String>, XEngineError> {
        let chunk_uid = chunk.uid.clone();

        let offsets = self.offsets.clone();
        let head_chunks = offsets
            .values()
            .map(|x| x.clone().end)
            .max()
            .unwrap_or(OFFSET_MAP_OFFSETS);

        println!("Head chunks offset: {}", head_chunks);

        let chunk_offset = ChunkOffset {
            start: head_chunks,
            end: head_chunks + chunk.data.len() as u64,
        };

        //TODO Update offset map on disk and update actual size on disk
        self.offsets.insert(chunk_uid.clone(), chunk_offset);

        file.write_all_at(&chunk.data, head_chunks).unwrap();

        self.chunks.insert(chunk_uid, chunk);

        return Ok(Some(self.uid.clone()));
    }
}
