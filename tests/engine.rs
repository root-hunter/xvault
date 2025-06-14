use std::fs;

use uuid::Uuid;
use RootFS::schema::{device::Device, xfile::XFile, volume::Volume};

#[test]
pub fn serialize_and_deserilize() {
    let dev1 = Device::new("4754f539-a953-4dc4-ad37-7a8ab142218c".into());

    let user_uid = "da64d273-e31b-48ca-8184-c741a34cb92d";
    let vol_path = "/home/roothunter/lab/RootFS/tmp/vol100.rootfs";
    let file_path = "/home/roothunter/lab/RootFS/assets/README.md";
    let vfolder = "/home";
    fs::remove_file(vol_path).unwrap();

    let user_uid = Uuid::parse_str(user_uid).unwrap();
    let mut vol1 = Volume::new(vol_path.into(), 10).unwrap();

    let mut file = XFile::new(user_uid, file_path.into(), vfolder.into()).unwrap();

    vol1.add_chunks_from_file(&mut file);
    vol1.save().unwrap();

    println!("{:?}", file);
}