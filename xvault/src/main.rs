use std::fs::{self};

use uuid::Uuid;
use xvault::engine::{chunk::ChunksHandler, device::Device, volume::Volume, xfile::XFile};

const USER_UID: &str = "da64d273-e31b-48ca-8184-c741a34cb92d";
const DEVIDE_UID: &str = "4754f539-a953-4dc4-ad37-7a8ab142218c";
const VOL_PATH: &str = "./tmp/vol_main.rootfs";

fn main() {
    let vfolder = "vfolder1";
    let file_path = "./assets/canterbury/plrabn12.txt";
    let file_path = std::path::Path::new(file_path);

    let user_uid = Uuid::parse_str(USER_UID).unwrap();

    let file = XFile::new(user_uid, file_path, vfolder.into());
    let vol_path = VOL_PATH.to_string();

    fs::remove_file(vol_path.clone()).unwrap_or(());

    if let Ok(file) = file {
        let dev= Device::new(DEVIDE_UID.into()).unwrap();

        let mut vol1 = Volume::new();
        vol1.set_path(vol_path)
            .set_uid_from_device(dev.uid)
            .set_max_size(200)
            .build()
            .unwrap();
        vol1.alloc_on_disk().unwrap();

        println!("File chunks: {:#?}", file.chunks);

        let mut fp = vol1.open(true).unwrap();
        vol1.add_chunks_v2(&fp, &file.chunks).unwrap();

        //vol1.set_offsets_from_file(&fp).unwrap();

        let old_chunks = vol1.offsets.clone();

        vol1.write_headers(&mut fp).unwrap();
        vol1.offsets.clear();
        vol1.chunks.clear();

        vol1.read_headers(&mut fp, false).unwrap();


        let chunk = vol1
            .get_chunk_v2(&fp, "2151b616-8f2c-574b-9e46-1abc0abb11da".into())
            .unwrap();

        if let Some(chunk) = chunk {
            println!(
                "Retrieved Chunk UID: {}, Length: {:?}",
                chunk.uid, chunk.length
            );
            println!("Chunk Data: {:?}", String::from_utf8(chunk.data));
        } else {
            println!("Chunk not found");
        }

        let new_chunks = vol1.offsets.clone();

        //vol1.read_headers(&mut fp, false).unwrap();

        println!("Volume UID: {}", vol1.uid);
        println!("Volume Path: {}", vol1.path);
        println!("Volume Max Size: {}", vol1.max_size);
        
        assert_eq!(old_chunks.len(), new_chunks.len());

        let new_chunks_uids: Vec<String> = new_chunks.keys().map(|e| e.clone()).collect();
        for (i, old) in old_chunks.clone().into_iter().enumerate() {
            let old_uid = old.0;
            let old_offset = old.1;

            assert!(new_chunks_uids.contains(&old_uid), "{} not included in the new uids", old_uid);

            let new_offset = old_chunks[&old_uid];
            assert_eq!(new_offset.start, old_offset.start, "Different start index value for chunk: {}", old_uid);
            assert_eq!(new_offset.end, old_offset.end, "Different end index value for chunk: {}", old_uid);
        }
    
    } else {
        println!("Failed to create XFile ({:?}): {}", file_path, file.unwrap_err());
    }
}
