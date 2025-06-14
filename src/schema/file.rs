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
    fs::{self},
    io::{self, Read},
    path::Path,
};
use uuid::Uuid;

use crate::schema::chunk::{Chunk, CHUNK_SIZE};


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DFile {
    pub uid: String,
    pub vpath: String,
    pub chunks: Vec<Chunk>,
}

impl DFile {
    pub fn new(user_uid: Uuid, file_path: String, vpath: String) -> Result<Self, io::Error> {
        let file_path = Path::new(&file_path);

        let file = fs::File::open(file_path);

        if let Ok(mut file) = file {
            let mut chunks = Vec::new();

            let filename = file_path.file_name().unwrap();
            let filename = filename.to_str().unwrap();
            let vabs = format!("{}/{}", vpath, filename);

            let file_uid = Uuid::new_v5(&user_uid, vabs.as_bytes());

            let mut buf = [0u8; CHUNK_SIZE];

            let mut i: usize = 0;
            while file.read(&mut buf).unwrap() > 0 {
                let chunk_uid = Uuid::new_v5(&file_uid, &i.to_be_bytes());
                let chunk = Chunk {
                    uid: chunk_uid.into(),
                    data: buf.to_vec(),
                };

                buf = [0u8; CHUNK_SIZE];
                chunks.push(chunk);
                i += 1;
            }

            return Ok(DFile {
                uid: file_uid.into(),
                vpath: vabs,
                chunks: chunks,
            });
        } else {
            return Err(file.unwrap_err());
        }
    }
}


