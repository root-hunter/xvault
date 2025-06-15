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

mod utils;

use std::{
    fs::{self}, path::Path
};

use rand::{rngs::StdRng, seq::{IndexedRandom, SliceRandom}, SeedableRng};
use xvault::engine::{chunk::{ChunkHandler, CHUNK_SIZE}, device::Device, volume::Volume, xfile::{XFile, XFileHandler, XFileQuery}};
use uuid::Uuid;


use utils::{compare_files_bin, FnCompareFile};

const USER_UID: &str = "da64d273-e31b-48ca-8184-c741a34cb92d";
const DEVIDE_UID: &str = "4754f539-a953-4dc4-ad37-7a8ab142218c";
const TMP_FOLDER: &str = "./tmp";
const ASSETS_FOLDER: &str = "./assets";
const EXPORTS_FOLDER: &str = "./exports/test_device";
const VOL_PATH_1: &str = "./tmp/vol_test_device_100.rootfs";
const VOL_PATH_2: &str = "./tmp/vol_test_device_200.rootfs";
const VOL_PATH_3: &str = "./tmp/vol_test_device_300.rootfs";

const RNG_SEED: u64 = 3;

fn test_file(file_path: &str, compare: FnCompareFile) {
    let vfolder = "/home";
    let assets_path = Path::new(ASSETS_FOLDER);
    let assets_file_path = assets_path.join(file_path);

    let exports_path = Path::new(EXPORTS_FOLDER);
    let export_file_path = exports_path.join(file_path);

    let vols = vec![VOL_PATH_1, VOL_PATH_2, VOL_PATH_3];
    for vol in vols {
        if let Err(_) = fs::remove_file(vol) {
            println!("Vol file {} does not exist", vol);
        }
    }

    if let Err(_) = fs::remove_file(export_file_path.clone()) {
        println!("Export file doest find");
    }

    let mut rng = StdRng::seed_from_u64(RNG_SEED);
  
    let user_id = Uuid::parse_str(USER_UID).unwrap();
    let vfolder = "home".to_string();

    let file = XFile::new(user_id, &assets_file_path, vfolder).unwrap();

    println!("File chunks count: {}", file.chunks.len());

    let mut vol1 = Volume::new(DEVIDE_UID.into(), VOL_PATH_1.into(), 10).unwrap();
    let mut vol2 = Volume::new(DEVIDE_UID.into(), VOL_PATH_2.into(), 10).unwrap();
    let mut vol3 = Volume::new(DEVIDE_UID.into(), VOL_PATH_3.into(), 10).unwrap();


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

    //println!("Vol1: {:#?}", vol1);
    //println!("Vol2: {:#?}", vol2);
    //println!("Vol3: {:#?}", vol3);

    let mut dev = Device::new(DEVIDE_UID.into()).unwrap();
    dev.add_volume(vol1);
    dev.add_volume(vol2);
    dev.add_volume(vol3);

    //println!("Device: {:#?}", dev);

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

    new_file.export_path(&export_file_path).unwrap();

    compare(&assets_file_path, &export_file_path);
}
include!(concat!(env!("OUT_DIR"), "/generated_device_tests.rs"));