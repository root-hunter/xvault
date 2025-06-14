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
use std::{fs::OpenOptions, io::Write};
pub use std::{
    fs::{self, File},
    io::{self, Read},
    path::Path,
};
pub use uuid::Uuid;

use crate::schema::{chunk::Chunk, xfile::XFile};

#[derive(Debug)]
pub enum Error {
    FileNotExists,
    IO(io::Error),
}

#[derive(Decode, Encode)]
struct VolumeWrap {
    #[bincode(with_serde)]
    pub data: Volume,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Volume {
    pub path: String,
    pub chunks: Vec<Chunk>,
}

impl Volume {
    pub fn new(path_str: String, size: usize) -> Result<Self, io::Error> {
        let exists = fs::exists(path_str.clone());

        if exists.is_err() {
            return Err(exists.unwrap_err());
        } else {
            let path = Path::new(&path_str);

            if !exists.unwrap() {
                let res = fs::File::create(path);

                if let Err(err) = res {
                    return Err(err);
                }
            }

            return Ok(Volume {
                path: path_str,
                chunks: Vec::<Chunk>::new(),
            });
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

                    let length = bincode::encode_into_slice(
                        VolumeWrap { data: self.clone() },
                        &mut buffer,
                        bincode::config::standard(),
                    )
                    .unwrap();

                    let res = fp.write(&buffer[..length]);

                    if let Ok(_) = res {
                        return Ok(());
                    } else {
                        return Err(Error::IO(res.unwrap_err()));
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

    pub fn add_chunk(&mut self, chunk: Chunk) {
        self.chunks.push(chunk);
    }

    pub fn add_chunks_from_file(&mut self, file: &mut XFile) {
        for chunk in file.chunks.clone() {
            self.add_chunk(chunk);
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
