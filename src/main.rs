use std::path::Path;

use rand::rngs::StdRng;
use uuid::Uuid;
use xvault::engine::chunk::{ChunkHandler, CHUNK_SIZE};
use xvault::engine::device::Device;
use xvault::engine::xfile::{XFileHandler, XFileQuery};
use xvault::engine::{volume::Volume, xfile::XFile};

const USER_UID: &str = "da64d273-e31b-48ca-8184-c741a34cb92d";
const DEVIDE_UID: &str = "4754f539-a953-4dc4-ad37-7a8ab142218c";

use rand::SeedableRng;
use rand::seq::{IndexedRandom, SliceRandom};

fn main() {
    const vol_path_1: &str = "tmp/vol100.rootfs";
    const vol_path_2: &str = "tmp/vol200.rootfs";
    const vol_path_3: &str = "tmp/vol300.rootfs";

    let user_id = Uuid::parse_str(USER_UID).unwrap();
    let file_path_1 = Path::new("assets/text/README.md");
    let vfolder = "home".to_string();

    let file = XFile::new(user_id, file_path_1, vfolder).unwrap();

    println!("File chunks count: {}", file.chunks.len());

    let mut vol1 = Volume::new(DEVIDE_UID.into(), vol_path_1.into(), 10).unwrap();
    let mut vol2 = Volume::new(DEVIDE_UID.into(), vol_path_2.into(), 10).unwrap();
    let mut vol3 = Volume::new(DEVIDE_UID.into(), vol_path_3.into(), 10).unwrap();

    let mut rng = StdRng::seed_from_u64(3);

    let mut file_chunks = file.chunks.to_vec();

    file_chunks.shuffle(&mut rng);

    println!("XFile: {:#?}", file);

    println!("Shuffle list: {:#?}", file_chunks);

    let vols = vec![1, 2, 3];

    for chunk in file_chunks {
        let vol_index = vols.choose(&mut rng).unwrap();        
        let vol = match vol_index {
            1 => &mut vol1,
            2 => &mut vol2,
            3 => &mut vol3,
            _ => panic!("Invalid volume index"),
        };

        vol.add_chunk(chunk);
    }

    let chunks_count = (file.size as f32) / (CHUNK_SIZE as f32);
    let chunks_count = chunks_count.ceil() as usize;

    for i in 0..chunks_count {
        let file_uid = Uuid::parse_str(&file.uid).unwrap();
        let chunk_uid = Uuid::new_v5(&file_uid, &i.to_be_bytes());

        println!("Chunk UID: {}", chunk_uid);
    }

    println!("Vol1: {:#?}", vol1);
    println!("Vol2: {:#?}", vol2);
    println!("Vol3: {:#?}", vol3);

    vol1.save().unwrap();
    vol2.save().unwrap();
    vol3.save().unwrap();

    let mut dev = Device::new(DEVIDE_UID.into()).unwrap();
    dev.add_volume(vol1);
    dev.add_volume(vol2);
    dev.add_volume(vol3);

    println!("Device: {:#?}", dev);

    let query = XFileQuery {
        uid: file.uid.clone(),
        chunk_count: chunks_count,
    };

    let find_chunks = dev.find_file_chunks(query);
    println!("Find chunks: {:#?}", find_chunks);

    let new_file = XFile {
        uid: file.uid.clone(),
        vpath: file.vpath,
        size: file.size,
        chunks: find_chunks.unwrap_or_default(),
    };

    new_file.export("exports/test_device/text/README.md".into()).unwrap();
}