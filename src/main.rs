use std::path::Path;

use uuid::Uuid;
use RootFS::engine::{device::Device, volume::Volume, xfile::XFile};

const USER_UID: &str = "da64d273-e31b-48ca-8184-c741a34cb92d";
const DEVIDE_UID: &str = "4754f539-a953-4dc4-ad37-7a8ab142218c";
const TMP_FOLDER: &str = "./tmp";
const ASSETS_FOLDER: &str = "./assets";
const EXPORTS_FOLDER: &str = "./exports";
const VOL_PATH: &str = "./tmp/vol100.rootfs";

fn main() {
    let dev1 = Device::new(DEVIDE_UID.into());

    let mut vol1 = Volume::new(VOL_PATH.into(), 10).unwrap();

    let vpath = "/home";
    let user_uid = USER_UID;
    let user_uid = Uuid::parse_str(user_uid).unwrap();

    let file_path = "./assets/text/README.md";
    let file_path = Path::new(file_path);

    let mut file = XFile::new(user_uid, file_path, vpath.into()).unwrap();

    vol1.add_chunks_from_file(&mut file);

    println!("{:?}", file);  
}