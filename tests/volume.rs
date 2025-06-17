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

use std::fs::{self, OpenOptions};
use uuid::Uuid;
use xvault::engine::{chunk::ChunkHandler, volume::Volume, xfile::XFile};
const USER_UID: &str = "da64d273-e31b-48ca-8184-c741a34cb92d";
const DEVIDE_UID: &str = "4754f539-a953-4dc4-ad37-7a8ab142218c";

#[test]
fn volume_test_set_max_size_from_disk() {
    let vol_path = "./tmp/vol33002.rootfs";
    let mut volume = Volume::new();
    let max_size = 100;

    volume
        .set_path(vol_path.to_string())
        .set_uid_from_device("4754f539-a953-4dc4-ad37-7a8ab142218c".into())
        .set_max_size(max_size)
        .build()
        .unwrap();

    fs::remove_file(vol_path).unwrap_or(());
    volume.alloc_on_disk().unwrap();

    let file = OpenOptions::new().read(true).open(&vol_path);
    let file = file.unwrap();

    // Now read it back
    assert!(volume.set_max_size_from_disk(&file).is_ok());
    assert_eq!(volume.max_size, max_size);
}

#[test]
fn volume_test_set_uid_from_disk() {
    let vol_path = "./tmp/vol35002.rootfs";
    let mut volume = Volume::new();

    let volume_uid = "4754f539-a953-4dc4-ad37-7a8ab142218c".to_string();

    volume
        .set_path(vol_path.to_string())
        .set_uid(volume_uid.clone())
        .set_max_size(100)
        .build()
        .unwrap();

    fs::remove_file(vol_path).unwrap_or(());
    volume.alloc_on_disk().unwrap();

    let file = OpenOptions::new().read(true).open(&vol_path);
    let file = file.unwrap();
    
    // Now read it back
    assert!(volume.set_uid_from_file(&file).is_ok());
    assert!(!volume.uid.is_empty());
    assert_eq!(volume.uid, volume_uid);
}

#[test]
fn volume_test_read_and_write_offsets() {
    let vol_path = "./tmp/vol35003.rootfs".to_string();
    let vfolder = "vfolder1";
    let file_path = "assets/canterbury/plrabn12.txt";
    let file_path = std::path::Path::new(file_path);

    let user_uid = Uuid::parse_str(USER_UID).unwrap();

    let file = XFile::new(user_uid, file_path, vfolder.into());

    fs::remove_file(vol_path.clone()).unwrap_or(());

    if let Ok(mut file) = file {
        let mut vol1 = Volume::new();
        vol1.set_path(vol_path)
            .set_uid_from_device(DEVIDE_UID.into())
            .set_max_size(100)
            .build()
            .unwrap();

        vol1.alloc_on_disk().unwrap();

        vol1.add_chunks_from_file(&mut file); 

        let fp = OpenOptions::new()
            .read(true)
            .write(true)
            .open(vol1.path.clone())
            .expect("Failed to open volume file");

        for (uid, chunk) in vol1.chunks.clone() {
            println!("Chunk UID: {}, Length: {:?}", chunk.uid, chunk.length);

            vol1.add_chunk_v2(&fp, chunk).unwrap();
        }

        let old_chunk_uids: Vec<String> = vol1.offsets.keys().cloned().collect();

        vol1.write_offsets_to_file(&fp).unwrap();

        let chunk = vol1.get_chunk_v2(&fp, "1263e31a-cb1c-5833-b22b-0e0c0b96165a".into()).unwrap();

        if let Some(chunk) = chunk {
            println!("Retrieved Chunk UID: {}, Length: {:?}", chunk.uid, chunk.length);
            println!("Chunk Data: {:?}", String::from_utf8(chunk.data));
        } else {
            println!("Chunk not found");
        }

        vol1.set_offsets_from_file(&fp).unwrap();
        let new_chunk_uids: Vec<String> = vol1.offsets.keys().cloned().collect();

        println!("Volume UID: {}", vol1.uid);
        println!("Volume Path: {}", vol1.path);
        println!("Volume Max Size: {}", vol1.max_size);
        println!("Offsets: {:?}", vol1.offsets);
        assert!(new_chunk_uids.len() == old_chunk_uids.len(), "Chunk UIDs have changed after writing offsets to file.");

        for old in old_chunk_uids {
            assert!(new_chunk_uids.contains(&old), "Old chunk UID {} not found in new chunk UIDs.", old);
        }

    } else {
        println!("Failed to create XFile");
    }
}