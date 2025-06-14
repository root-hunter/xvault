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
use std::fmt::Debug;
pub use std::{
    fs::{self, File},
    io::{self, Read},
    path::Path,
};
pub use uuid::Uuid;

use crate::engine::xfile::XFile;

pub const CHUNK_SIZE: usize = 512;

#[derive(Serialize, Deserialize, Clone)]
pub struct Chunk {
    pub uid: String,
    pub data: Vec<u8>,
    pub length: Option<usize>
}

impl Debug for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Chunk {{ uid: {}, length: {:?} }}", self.uid, self.length)
    }
}

pub trait ChunkHandler {
    fn is_full(self) -> bool;
    fn get_chunk(&mut self, uuid: String) -> Option<&Chunk>;
    fn add_chunk(&mut self, chunk: Chunk) -> Option<String>;
    fn add_chunks_from_file(&mut self, file: &mut XFile) {
        for chunk in file.chunks.clone() {
            self.add_chunk(chunk);
        }
    }
}