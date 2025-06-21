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
use std::{collections::HashMap, fs::OpenOptions, io::{Seek, SeekFrom, Write}, os::unix::fs::FileExt, vec};
pub use std::{
    fs::{self, File},
    io::{self, Read},
    path::Path,
};
pub use uuid::Uuid;

use crate::engine::{
    chunk::{Chunk, ChunksHandler, CHUNK_SIZE},
    error::XEngineError,
    utils::{decode_number, decode_uuid_to_string, encode_number, encode_uuid_from_string, get_bincode_config, parse_offset_map_elem},
};

const UID_LEN: u64 = 16; // UUID size in bytes
const OFFSET_VOLUME_UID: u64 = 0;

const MAX_SIZE_LEN: u64 = 8; //u64 size
const OFFSET_MAX_SIZE: u64 = OFFSET_VOLUME_UID + UID_LEN;

const ACTUAL_SIZE_LEN: u64 = 8; //u64 size
const OFFSET_ACTUAL_SIZE: u64 = OFFSET_MAX_SIZE + MAX_SIZE_LEN;

const MAP_OFFSETS_ELEM_CHUNK_UID_LEN: u64 = 16; // UUID size in bytes
const MAP_OFFSETS_ELEM_OFFSET_START_LEN: u64 = 8; // u64 size
const MAP_OFFSETS_ELEM_OFFSET_END_LEN: u64 = 8; // u64 size

const MAP_OFFSETS_ELEM_LEN: u64 = MAP_OFFSETS_ELEM_CHUNK_UID_LEN
    + MAP_OFFSETS_ELEM_OFFSET_START_LEN
    + MAP_OFFSETS_ELEM_OFFSET_END_LEN;

const MAP_OFFSETS_START_OFFSET: u64 = UID_LEN + MAX_SIZE_LEN + ACTUAL_SIZE_LEN;

//pub type VolumeChunkOffset = [u8; 2];
#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
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

        let volume_uid = decode_uuid_to_string(buf);
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


    pub fn read_max_size_from_file(&mut self, file: &File) -> Result<u64, XEngineError> {
        let mut buf = [0u8; 8]; // u64 size
        if let Err(err) = file.read_exact_at(&mut buf, OFFSET_MAX_SIZE) {
            return Err(XEngineError::IO(err));
        }

        let config = get_bincode_config();
        let max_size = decode_number(&buf, &config)?;

        return Ok(max_size);
    }

    pub fn read_actual_size_from_file(&mut self, file: &mut File) -> Result<u64, XEngineError> {
        let mut buf = [0u8; 8]; // u64 size
        if let Err(err) = file.read_exact_at(&mut buf, OFFSET_ACTUAL_SIZE) {
            return Err(XEngineError::IO(err));
        } else {
            file.seek(SeekFrom::Start(0)).unwrap();
        }

        let config = get_bincode_config();
        let actual_size: u64 = decode_number(&buf, &config)?;

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
                let config = get_bincode_config();

                let volume_uid = Uuid::parse_str(&self.uid).unwrap();
                let volume_uid = volume_uid.to_bytes_le();

                let volume_size = 
                    MAP_OFFSETS_START_OFFSET
                    + (MAP_OFFSETS_ELEM_LEN * self.max_size)
                    + (CHUNK_SIZE as u64 * self.max_size); 
                
                file.set_len(volume_size).unwrap();

                let max_size =
                    bincode::encode_to_vec(&self.max_size, config).map_err(XEngineError::Encode)?;

                let actual_size = self.get_actual_size();
                let actual_size =
                    bincode::encode_to_vec(&actual_size, config).map_err(XEngineError::Encode)?;

                println!("max_size length: {}", max_size.len());
                println!("actual_size length: {}", actual_size.len());

                file.write_all(&volume_uid).unwrap();
                file.write_all(&max_size).unwrap();
                file.write_all(&actual_size).unwrap();

                return Ok(());
            } else {
                return Err(XEngineError::VolumeAlreadyAllocated);
            }
        }
    }

    pub fn open(&mut self, write: bool) -> Result<File, XEngineError> {
        let file = OpenOptions::new().read(true).write(write).open(&self.path);

        if let Err(err) = file {
            return Err(XEngineError::IO(err));
        }

        return Ok(file.unwrap());
    }

    pub fn write_headers(&mut self, file: &mut File) -> Result<(), XEngineError> {
        //TODO FIX BUG
        
        let config = get_bincode_config();
        let actual_size = self.offsets.len() as u64;

        let header_len = MAP_OFFSETS_START_OFFSET + (actual_size * MAP_OFFSETS_ELEM_LEN);
        let mut buf = vec![0u8; header_len as usize];
        let buf = buf.as_mut_slice();


        println!("actuoal_size: {actual_size}");
        println!("header_len {header_len}");
        println!("MaxOffsets_start_offset {MAP_OFFSETS_START_OFFSET}");

        let index = UID_LEN as usize;
        let volume_uid_slice = &mut buf[..index];
        let volume_uid_bytes = encode_uuid_from_string(self.uid.clone()).unwrap();
        volume_uid_slice.copy_from_slice(&volume_uid_bytes);

        let max_size_slice = &mut buf[index..(index + 8)];
        let max_size_bytes = encode_number(self.max_size, config).unwrap();
        max_size_slice.copy_from_slice(&max_size_bytes);
        
        let actual_size_slice = &mut buf[(index + 8)..(index + 16)];
        let actual_size_bytes = encode_number(actual_size, config).unwrap();
        actual_size_slice.copy_from_slice(&actual_size_bytes);

        let map_start = MAP_OFFSETS_START_OFFSET as usize;

        let offsets_slice = &mut buf[map_start..header_len as usize];

        for (i, (uid, offset)) in self.offsets.clone().into_iter().enumerate() {
            let start = i * 32;
            let end = start + 32;

            let offset_slice = &mut offsets_slice[start..end];

            let chunk_uid_slice = &mut offset_slice[..16];
            let chunk_uid_bytes = encode_uuid_from_string(uid).unwrap();
            chunk_uid_slice.copy_from_slice(&chunk_uid_bytes);
            
            let chunk_start_slice = &mut offset_slice[16..24];
            let chunk_start_bytes = encode_number(offset.start, config).unwrap();
            chunk_start_slice.copy_from_slice(&chunk_start_bytes);

            let chunk_end_slice = &mut offset_slice[24..32];
            let chunk_end_bytes = encode_number(offset.end, config).unwrap();
            chunk_end_slice.copy_from_slice(&chunk_end_bytes);
        }

        file.write_all_at(buf, 0).unwrap();

        return Ok(());
    }

    pub fn read_headers(&mut self, file: &mut File, cached: bool) -> Result<(), XEngineError> {
        let config = get_bincode_config();
        let actual_size = self.read_actual_size_from_file(file)?;

        // UID + MAX_SIZE + ACTUAL_SIZE + OFFSET_MAP_OFFSETS + (max_size * 32)
        let header_len = MAP_OFFSETS_START_OFFSET + (actual_size * MAP_OFFSETS_ELEM_LEN);
        let mut buf = vec![0u8; header_len as usize];

        let bytes = file.read_exact(&mut buf);

        if let Err(err) = bytes {
            return Err(XEngineError::IO(err));
        }

        let buf = buf.as_slice();
        let volume_uid_bytes = buf
            [OFFSET_VOLUME_UID as usize..(OFFSET_VOLUME_UID + UID_LEN) as usize]
            .try_into()
            .unwrap();
        let volume_uid = decode_uuid_to_string(volume_uid_bytes);
        println!("reading uuid: {volume_uid}");
        self.set_uid(volume_uid);

        let max_size_bytes =
            &buf[OFFSET_MAX_SIZE as usize..(OFFSET_MAX_SIZE + MAX_SIZE_LEN) as usize];
        let max_size = decode_number(max_size_bytes, &config).unwrap();
        self.set_max_size(max_size);
        println!("reading max_size: {max_size}");

        let actual_size_bytes =
            &buf[OFFSET_ACTUAL_SIZE as usize..(OFFSET_ACTUAL_SIZE + ACTUAL_SIZE_LEN) as usize];
        let actual_size: u64 = decode_number(actual_size_bytes, &config).unwrap();
        println!("reading actual_size: {actual_size}");

        let mut offsets = VolumeOffsets::with_capacity(actual_size as usize);
        //let mut offsets = VolumeOffsets::new();

        let mut index = MAP_OFFSETS_START_OFFSET as usize;


        for _ in 0..actual_size {
            let map_elem_bytes = &buf[index..(index + MAP_OFFSETS_ELEM_LEN as usize)];
            let result = parse_offset_map_elem(&map_elem_bytes, config)?;
            
            offsets.insert(result.uid, result.offset);
            index += MAP_OFFSETS_ELEM_LEN as usize;
        }

        self.offsets = offsets;

        return Ok(());
    }
}

impl ChunksHandler for Volume {
    fn get_max_size(&self) -> u64 {
        return self.max_size;
    }
    
    fn get_actual_size(&self) -> u64 {
        return self.offsets.len() as u64;
    }

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

            let chunk = Chunk {
                uid: uuid,
                data: buf,
                length: Some(chunk_len as usize),
            };

            return Ok(Some(chunk));
        }
    }

    fn add_chunk_v2(&mut self, file: &File, chunk: Chunk) -> Result<Option<String>, XEngineError> {
        let max_size = self.get_max_size();
        let actual_size = self.get_actual_size();

        assert!(actual_size + 1 <= max_size, "Can't add others chunks to the handler: actual_size + 1 > max_size ({} + 1 > {})", actual_size, max_size);
        
        let chunk_uid = chunk.uid.clone();

        let offsets = self.offsets.clone();
        let head_chunks = offsets
            .values()
            .map(|x| x.clone().end)
            .max()
            .unwrap_or(MAP_OFFSETS_START_OFFSET);

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
