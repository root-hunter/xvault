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

use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::{self, Read, Write},
    os::unix::fs::MetadataExt,
    path::Path,
};
use uuid::Uuid;

use crate::engine::{chunk::{Chunk, CHUNK_SIZE}, error::XVaultError};

pub type XFileChunks = Vec<Chunk>;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct XFileQuery {
    pub uid: String,
    pub chunk_count: usize,
}

/*
    File has chunks ordered internally
*/
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct XFile {
    pub uid: String,
    pub vpath: String,
    pub size: usize,
    pub chunks: XFileChunks,
}

impl XFile {
    pub fn build_chunk_uid(file_uid: String, index: usize) -> String {
        let index_bytes = index.to_be_bytes();
        let file_uid = Uuid::parse_str(&file_uid).unwrap();

        return Uuid::new_v5(&file_uid, &index_bytes).to_string();
    }

    pub fn get_chunk_uid(&self, index: usize) -> String {
        return XFile::build_chunk_uid(self.uid.clone(), index);
    }

    pub fn new(user_uid: Uuid, file_path: &Path, vfolder: String) -> Result<Self, io::Error> {
        let file = fs::File::open(file_path);

        if let Ok(mut file) = file {
            let mut chunks = Vec::new();

            let filename = file_path.file_name().unwrap();
            let filename = filename.to_str().unwrap();

            let vabs = format!("{}/{}", vfolder, filename);
            let file_uid = Uuid::new_v5(&user_uid, vabs.as_bytes());

            let metadata = file.metadata().unwrap();
            let file_length = metadata.size() as usize;

            let mut buf = [0u8; CHUNK_SIZE];

            let mut i: usize = 0;

            loop {
                let read_bytes = file.read(&mut buf).unwrap();
                let data = buf.to_vec();

                let chunk_uid = Uuid::new_v5(&file_uid, &i.to_be_bytes());
                let mut length = None;

                if read_bytes < CHUNK_SIZE {
                    length = Some(file_length - (CHUNK_SIZE * i));
                }

                let chunk = Chunk {
                    uid: chunk_uid.into(),
                    data: data,
                    length: length,
                };

                buf = [0u8; CHUNK_SIZE];
                chunks.push(chunk);

                if read_bytes < CHUNK_SIZE {
                    break;
                }
                i += 1;
            }

            return Ok(XFile {
                uid: file_uid.into(),
                vpath: vabs,
                chunks: chunks,
                size: file_length,
            });
        } else {
            return Err(file.unwrap_err());
        }
    }

    pub fn export(self, path: String) -> Result<(), XVaultError> {
        let path = Path::new(&path);

        return self.export_path(path);
    }

    pub fn export_path(self, path: &Path) -> Result<(), XVaultError> {
        if let Some(parent) = path.parent() {
            if let Err(err) = fs::create_dir_all(parent) {
                return Err(XVaultError::IO(err));
            }
        }

        let file = File::create(path);

        if let Ok(mut file) = file {
            for chunk in self.chunks {
                let data = if chunk.length.is_some() {
                    let length = chunk.length.unwrap();
                    &chunk.data.as_slice()[..length]
                } else {
                    &chunk.data.as_slice()
                };

                if let Err(err) = file.write(data) {
                    return Err(XVaultError::IO(err));
                }
            }

            return Ok(());
        } else {
            return Err(XVaultError::IO(file.unwrap_err()));
        }
    }
}

pub trait XFileHandler {
    fn find_file_chunks(&mut self, query: XFileQuery) -> Option<XFileChunks>;
}
