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
use std::io::Write;
pub use std::{
    fs::{self, File},
    io::{self, Read},
    path::Path,
};
pub use uuid::Uuid;

use crate::schema::{chunk::Chunk, file::DFile};

pub enum Error {
    FileNone,
    IO(io::Error),
}

#[derive(Decode, Encode)]
struct VolumeWrap {
    #[bincode(with_serde)]
    pub data: Volume,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Volume {
    #[serde(skip)]
    pub fp: Option<File>,

    pub path: String,
    pub chunks: Vec<Chunk>,
}

impl Clone for Volume {
    fn clone(&self) -> Self {
        Self {
            fp: None,
            path: self.path.clone(),
            chunks: self.chunks.clone(),
        }
    }
}

impl Volume {
    pub fn new(path_str: String, size: usize) -> Result<Self, io::Error> {
        let exists = fs::exists(path_str.clone());

        if exists.is_err() {
            return Err(exists.unwrap_err());
        } else {
            let path = Path::new(&path_str);

            let file;

            if !exists.unwrap() {
                file = fs::File::create(path);
            } else {
                file = fs::File::open(path);
            }

            if let Ok(file) = file {
                let mut chunks = Vec::<Chunk>::new();

                return Ok(Volume {
                    path: path_str,
                    chunks: chunks,
                    fp: Some(file),
                });
            } else {
                return Err(file.unwrap_err());
            }
        }
    }

    pub fn save(&mut self) -> Result<(), Error> {
        let exists = fs::exists(self.path.clone());

        if let Ok(exists) = exists {
            let path = Path::new(&self.path);

            let mut buffer = [0u8; 4096 * 10];

            let length = bincode::encode_into_slice(
                VolumeWrap { data: self.clone() },
                &mut buffer,
                bincode::config::standard(),
            )
            .unwrap();

            if let Some(file) = &mut self.fp {
                let res = file.write(&buffer[..length]);

                if let Ok(_) = res {
                    return Ok(());
                } else {
                    return Err(Error::IO(res.unwrap_err()));
                }
            } else {
                return Err(Error::FileNone);
            }
        } else {
            return Err(Error::IO(exists.unwrap_err()));
        }
    }

    pub fn add_chunk(&mut self, chunk: Chunk) {
        self.chunks.push(chunk);
    }

    pub fn add_chunks_from_file(&mut self, file: &mut DFile) {
        for chunk in file.chunks.clone() {
            self.chunks.push(chunk);
        }
    }
}
