use std::fs::{self, OpenOptions};

use uuid::Uuid;
use xvault::engine::{chunk::ChunkHandler, volume::Volume, xfile::XFile};

const USER_UID: &str = "da64d273-e31b-48ca-8184-c741a34cb92d";
const DEVIDE_UID: &str = "4754f539-a953-4dc4-ad37-7a8ab142218c";
const VOL_PATH: &str = "./tmp/vol10002.rootfs";

fn main() {
    let vfolder = "vfolder1";
    let file_path = "assets/canterbury/plrabn12.txt";
    let file_path = std::path::Path::new(file_path);

    let user_uid = Uuid::parse_str(USER_UID).unwrap();

    let file = XFile::new(user_uid, file_path, vfolder.into());
    let vol_path = VOL_PATH.to_string();

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
