use std::path::Path;

use rand::rngs::StdRng;
use uuid::Uuid;
use RootFS::engine::chunk::CHUNK_SIZE;
use RootFS::engine::{volume::Volume, xfile::XFile};

const USER_UID: &str = "da64d273-e31b-48ca-8184-c741a34cb92d";
const DEVIDE_UID: &str = "4754f539-a953-4dc4-ad37-7a8ab142218c";

use rand::SeedableRng;
use rand::seq::SliceRandom;

fn main() {
    const vol_path_1: &str = "./tmp/vol100.rootfs";
    const vol_path_2: &str = "./tmp/vol200.rootfs";
    const vol_path_3: &str = "./tmp/vol300.rootfs";

    let user_id = Uuid::parse_str(USER_UID).unwrap();
    let file_path_1 = Path::new("assets/text/README.md");
    let vfolder = "home".to_string();

    let file = XFile::new(user_id, file_path_1, vfolder).unwrap();

    println!("File chunks count: {}", file.chunks.len());

    let vol1 = Volume::new(vol_path_1.into(), 10).unwrap();
    let vol2 = Volume::new(vol_path_2.into(), 10).unwrap();
    let vol3 = Volume::new(vol_path_3.into(), 10).unwrap();

    let mut rng = StdRng::seed_from_u64(3);

    let mut file_chunks = file.chunks.to_vec();

    file_chunks.shuffle(&mut rng);

    println!("XFile: {:#?}", file);

    println!("Shuffle list: {:#?}", file_chunks);

    let chunks_count = (file.size as f32) / (CHUNK_SIZE as f32);
    let chunks_count = chunks_count.ceil() as usize;


    for i in 0..chunks_count {
        let file_uid = Uuid::parse_str(&file.uid).unwrap();
        let chunk_uid = Uuid::new_v5(&file_uid, &i.to_be_bytes());

        println!("Chunk UID: {}", chunk_uid);
    }

}