use std::{
    fs::{self},
    path::Path,
};

use RootFS::schema::{device::Device, volume::Volume, xfile::XFile};
use uuid::Uuid;

use similar::{Algorithm, TextDiff};

const USER_UID: &str = "da64d273-e31b-48ca-8184-c741a34cb92d";
const DEVIDE_UID: &str = "4754f539-a953-4dc4-ad37-7a8ab142218c";
const TMP_FOLDER: &str = "./tmp";
const ASSETS_FOLDER: &str = "./assets";
const EXPORTS_FOLDER: &str = "./exports";
const VOL_PATH: &str = "./tmp/vol100.rootfs";

macro_rules! generate_file_tests {
    ($(($name:ident, $file1:expr)),* $(,)?) => {
        $(
            #[test]
            fn $name() {
                let vfolder = "/home";
                let assets_path = Path::new(ASSETS_FOLDER);
                let assets_file_path = assets_path.join($file1);
                let file_name = assets_file_path.file_name().unwrap().to_str().unwrap();

                let exports_path = Path::new(EXPORTS_FOLDER);
                let export_file_path = exports_path.join(file_name);

                let dev1 = Device::new(DEVIDE_UID.into());


                if let Err(err) = fs::remove_file(VOL_PATH) {
                    println!("Vol file doest find");
                }

                if let Err(err) = fs::remove_file(export_file_path.clone()) {
                    println!("Export file doest find");
                }

                let user_uid = Uuid::parse_str(USER_UID).unwrap();
                let vol1 = Volume::new(VOL_PATH.into(), 10).unwrap();

                let file = XFile::new(user_uid, &assets_file_path, vfolder.into()).unwrap();

                file.export_path(&export_file_path).unwrap();

                let original_file = fs::read_to_string(assets_file_path).unwrap();
                let export_file = fs::read_to_string(export_file_path).unwrap();

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
        )*
    };
}

generate_file_tests! {
    (test_file_1, "README.md"),
    (test_file_2, "Satoshi_Nakamoto.html"),
}