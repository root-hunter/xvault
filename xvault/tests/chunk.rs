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

use std::path::Path;

use uuid::Uuid;
use xvault::engine::chunk::CHUNK_SIZE;
use xvault::engine::xfile::XFile;

const USER_UID: &str = "da64d273-e31b-48ca-8184-c741a34cb92d";

#[test]
fn test_uids() {
    let user_id = Uuid::parse_str(USER_UID).unwrap();
    let file_path_1 = Path::new("assets/README.md");
    let vfolder = "home".to_string();

    let file = XFile::new(user_id, file_path_1, vfolder).unwrap();

    println!("File chunks count: {}", file.chunks.len());

    println!("XFile: {:#?}", file);

    let chunks_count = (file.size as f32) / (CHUNK_SIZE as f32);
    let chunks_count = chunks_count.ceil() as usize;

    let uids = file.chunks.iter().map(|c| c.uid.clone()).collect::<Vec<_>>();
    let mut generated_uids = Vec::new();

    for i in 0..chunks_count {
        let file_uid = Uuid::parse_str(&file.uid).unwrap();
        let chunk_uid = Uuid::new_v5(&file_uid, &i.to_be_bytes());

        generated_uids.push(chunk_uid.to_string());

        println!("Chunk UID: {}", chunk_uid);
    }

    if uids.len() != generated_uids.len() {
        panic!("Mismatch in number of UIDs: expected {}, got {}", uids.len(), generated_uids.len());
    }

    for uid in uids {
        if !generated_uids.contains(&uid) {
            panic!("UID {} not found in generated UIDs", uid);
        }
    }
}