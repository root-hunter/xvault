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
use std::{collections::HashMap, fs::OpenOptions, io::Write};
pub use std::{
    fs::{self, File},
    io::{self, Read},
    path::Path,
};
pub use uuid::Uuid;

use crate::engine::chunk::{Chunk, ChunkHandler};

pub type VolumeChunks = HashMap<String, Chunk>;

#[derive(Debug)]
pub enum Error {
    FileNotExists,
    IO(io::Error),
    Encode(bincode::error::EncodeError),
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
    pub max_size: usize,
}

impl Default for Volume {
    fn default() -> Self {
        Self {
            uid: Default::default(),
            path: Default::default(),
            chunks: Default::default(),
            max_size: Default::default(),
        }
    }
}

impl Volume {
    pub fn new() -> Self {
        return Self::default();
    }

    pub fn set_uid(&mut self, device_uid: String) -> &mut Self {
        assert!(!device_uid.is_empty(), "Device UID cannot be empty");

        let path_str = self.path.clone();
        assert!(!path_str.is_empty(), "Volume path cannot be empty");

        let device_uid = Uuid::parse_str(&device_uid).unwrap();
        let volume_uid = Uuid::new_v5(&device_uid, path_str.as_bytes());

        self.uid = volume_uid.to_string();
        return self;
    }

    pub fn set_path(&mut self, path: String) -> &mut Self {
        self.path = path;
        return self;
    }

    pub fn set_max_size(&mut self, max_size: usize) -> &mut Self {
        self.max_size = max_size;
        return self;
    }

    pub fn build(&mut self) -> Result<&mut Self, io::Error> {
        assert!(!self.uid.is_empty(), "Volume uid cannot be empty");
        assert!(!self.path.is_empty(), "Volume path cannot be empty");
        assert!(self.max_size > 0, "Volume max_size cannot be 0");

        let path_str = self.path.clone();
        let exists = fs::exists(path_str.clone());

        if exists.is_err() {
            return Err(exists.unwrap_err());
        } else {
            let path = Path::new(&path_str);
            if !path.is_absolute() {
                let path = fs::canonicalize(path_str.clone()).unwrap_or_else(|_| path.to_path_buf());
                let path = path.to_str().unwrap_or("").to_string();

                self.set_path(path);
            }

            if !exists.unwrap() {
                let res = fs::File::create(path);

                if let Err(err) = res {
                    return Err(err);
                }
            }

            return Ok(self);
        }
    }

    pub fn save(&mut self) -> Result<(), Error> {
        let exists = self.exists();

        if let Ok(exists) = exists {
            if exists {
                let path = Path::new(&self.path);

                let fp = OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(path);

                if let Ok(mut fp) = fp {
                    let mut buffer = [0u8; 4096 * 10];

                    let res_length = bincode::encode_into_slice(
                        VolumeWrap { data: self.clone() },
                        &mut buffer,
                        bincode::config::standard(),
                    );

                    if let Ok(length) = res_length {
                        let res = fp.write(&buffer[..length]);

                        if let Ok(_) = res {
                            return Ok(());
                        } else {
                            return Err(Error::IO(res.unwrap_err()));
                        }
                    } else {
                        return Err(Error::Encode(res_length.unwrap_err()));
                    }
                } else {
                    return Err(Error::IO(fp.unwrap_err()));
                }
            } else {
                return Err(Error::FileNotExists);
            }
        } else {
            return Err(exists.unwrap_err());
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
        return self.chunks.get(&uuid);
    }

    fn add_chunk(&mut self, chunk: Chunk) -> Option<String> {
        self.chunks.insert(chunk.uid.clone(), chunk);
        return Some(self.uid.clone());
    }

    fn is_full(self) -> bool {
        return self.chunks.len() >= self.max_size;
    }
}
