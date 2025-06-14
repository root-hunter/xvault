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
use std::collections::HashMap;
pub use std::{
    fs::{self, File},
    io::{self, Read},
    path::Path,
};
pub use uuid::Uuid;

use crate::engine::{
    chunk::{Chunk, ChunkHandler},
    volume::Volume,
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Device {
    pub uid: String,
    pub volumes: HashMap<String, Volume>,
}

impl Device {
    pub fn new(uid: String) -> Result<Self, uuid::Error> {
        let uid = Uuid::parse_str(&uid);

        if uid.is_err() {
            return Err(uid.unwrap_err());
        } else {
            return Ok(Self {
                uid: uid.unwrap().into(),
                volumes: HashMap::new(),
            });
        }
    }

    pub fn add_volume(&mut self, volume: Volume) {
        self.volumes.insert(volume.uid.clone(), volume);
    }
}

impl ChunkHandler for Device {
    fn get_chunk(&mut self, chunk_uid: String) -> Option<&Chunk> {
        for volume in self.volumes.values_mut() {
            if let Some(chunk) = volume.get_chunk(chunk_uid.clone()) {
                return Some(chunk);
            }
        }
        return None;
    }
    
    fn add_chunk(&mut self, chunk: Chunk) {
        todo!()
    }
    
    fn add_chunks_from_file(&mut self, file: &mut super::xfile::XFile) {
        todo!()
    }
}