use std::fs::{self};

use uuid::Uuid;
use RootFS::schema::{device::Device, xfile::XFile, volume::Volume};

use similar::{Algorithm, TextDiff};

#[test]
pub fn serialize_and_deserilize() {
    let dev1 = Device::new("4754f539-a953-4dc4-ad37-7a8ab142218c".into());

    let user_uid = "da64d273-e31b-48ca-8184-c741a34cb92d";
    let vol_path = "./tmp/vol100.rootfs";
    let file_path = "./assets/README.md";
    let export_path = "./exports/README.md";
    
    let vfolder = "/home";

    fs::remove_file(vol_path).unwrap();
    fs::remove_file(export_path).unwrap();

    let user_uid = Uuid::parse_str(user_uid).unwrap();
    let vol1 = Volume::new(vol_path.into(), 10).unwrap();

    let file = XFile::new(user_uid, file_path.into(), vfolder.into()).unwrap();
    println!("{:?}", file);

    file.export(export_path.into()).unwrap();

    let original_file = fs::read_to_string(file_path).unwrap();
    let export_file = fs::read_to_string(export_path).unwrap();

    let diff = TextDiff::configure()
    .algorithm(Algorithm::Myers)
    .diff_lines(&original_file, &export_file);

    if !(diff.ratio() == 1.0) {
        println!("--- Difference Detected ---");
        for change in diff.iter_all_changes() {
            print!(
                "{}{}",
                match change.tag() {
                    similar::ChangeTag::Delete => "-",
                    similar::ChangeTag::Insert => "+",
                    similar::ChangeTag::Equal => " ",
                },
                change
            );
        }
        panic!("Files differ!");
    }
}