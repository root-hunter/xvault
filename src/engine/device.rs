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
    chunk::{Chunk, ChunksHandler}, error::XEngineError, volume::Volume, xfile::{XFile, XFileChunks, XFileHandler, XFileQuery}
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

impl ChunksHandler for Device {
    fn get_max_size(&self) -> u64 {
        return self.volumes.values().map(|v| v.get_max_size() as u64).sum();
    }
    
    fn get_actual_size(&self) -> u64 {
        return self.volumes.values().map(|v| v.get_actual_size() as u64).sum();
    }

    fn get_chunk(&mut self, chunk_uid: String) -> Option<&Chunk> {
        for volume in self.volumes.values_mut() {
            if let Some(chunk) = volume.get_chunk(chunk_uid.clone()) {
                return Some(chunk);
            }
        }
        return None;
    }
    
    fn add_chunk(&mut self, chunk: Chunk) -> Option<String> {
        let mut volumes = self.volumes.values_mut().collect::<Vec<&mut Volume>>();
        volumes.sort_by(|a, b| a.chunks.len().cmp(&b.chunks.len()));
    
        for volume in volumes {
            if !volume.clone().is_full() {
                return volume.add_chunk(chunk.clone());
            }
        }

        return None;
    }
    
    fn is_full(self) -> bool {
        return self.volumes.values().all(|v| v.clone().is_full());
    }
    
    fn get_chunk_v2(&mut self, file: &File, uuid: String) -> Result<Option<Chunk>, XEngineError> {
        todo!()
    }
    
    fn add_chunk_v2(&mut self, file: &File, chunk: Chunk) -> Result<Option<String>, XEngineError> {
        todo!()
    }
    
    
}

impl XFileHandler for Device {
    fn find_file_chunks(&mut self, query: XFileQuery) -> Option<XFileChunks> {
        let file_uid = query.uid;
        let count = query.chunk_count;
        let mut chunks: Vec<Chunk> = Vec::new();

        for index in  0..count {
            let chunk_uid = XFile::build_chunk_uid(file_uid.clone(), index);
            if let Some(chunk) = self.get_chunk(chunk_uid) {
                chunks.push(chunk.clone());
            }
        }

        if chunks.is_empty() {
            return None;
        } else {
            return Some(chunks);
        }
    }
}