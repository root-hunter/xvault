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

use std::{fs, io::{self, Read}, path::Path};
use uuid::Uuid;

const CHUNK_SIZE: usize = 512;

#[derive(Debug)]
struct Chunk {
    uid: Uuid,
    data: Vec<u8>,
}

#[derive(Debug)]
struct DFile {
    uid: Uuid,
    vpath: String,
    chunks: Vec<Chunk>,
}

impl DFile {
    fn new(user_uid: Uuid, file_path: String, vpath: String) -> Result<Self, io::Error> {

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
                let chunk = Chunk{
                    uid: chunk_uid,
                    data: buf.to_vec()
                };

                buf = [0u8; CHUNK_SIZE];
                chunks.push(chunk);
                i += 1;
            }

            return Ok(DFile { 
                uid: file_uid,
                vpath,
                chunks: chunks
            });
        } else {
            return Err(file.unwrap_err());
        }

    }
}

#[derive(Debug)]
struct Volume {
    path: String,
    chunks: Vec<Chunk>,
}

impl Volume {
    fn new(path: String, size: usize) -> Self {
        let mut chunks = Vec::<Chunk>::new();

        for i in 0..size {
            chunks.push(Chunk {
                uid: Uuid::new_v4(),
                data: Vec::from([0; CHUNK_SIZE]),
            });
        }

        return Volume {
            path: path,
            chunks: chunks,
        };
    }

    fn init(&self) -> Result<fs::File, io::Error> {
        let exists = fs::exists(self.path.clone());

        if exists.is_err() {
            return Err(exists.unwrap_err());
        } else {
            let path = self.get_path();

            if !exists.unwrap() {
                let file = fs::File::create(path);
                return file
            } else {
                let file = fs::File::open(path);
                return file
            }
        }
    }

    fn get_path(&self) -> &Path {
        return &Path::new(&self.path);
    }
}

#[derive(Debug)]
struct Device {
    uid: Uuid,
    volumes: Vec<Volume>,
}

impl Device {
    fn new(uid: String) -> Result<Self, uuid::Error> {
        let uid = Uuid::parse_str(&uid);

        if uid.is_err() {
            return Err(uid.unwrap_err());
        } else {
            return Ok(Self {
                uid: uid.unwrap(),
                volumes: Vec::new(),
            });
        }
    }
}

fn main() {
    let dev1 = Device::new("4754f539-a953-4dc4-ad37-7a8ab142218c".into());

    let vol1 = Volume::new("/home/roothunter/lab/RootFS/tmp/vol100.rootfs".into(), 10);

    vol1.init().unwrap();

    let user_uid = "da64d273-e31b-48ca-8184-c741a34cb92d";
    let user_uid = Uuid::parse_str(user_uid).unwrap();

    let file_path = "/home/roothunter/lab/RootFS/tests/README.md";
    let vpath = "/home";

    let file = DFile::new(user_uid, file_path.into(), vpath.into()).unwrap();

    println!("{:?}", file);
}
