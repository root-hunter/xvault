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

use std::{
    fs::{self, File}, io::{BufReader, Read}, path::{Path, PathBuf}
};

use RootFS::engine::{volume::Volume, xfile::XFile};
use uuid::Uuid;

use similar::{Algorithm, TextDiff};

const USER_UID: &str = "da64d273-e31b-48ca-8184-c741a34cb92d";
const DEVIDE_UID: &str = "4754f539-a953-4dc4-ad37-7a8ab142218c";
const TMP_FOLDER: &str = "./tmp";
const ASSETS_FOLDER: &str = "./assets";
const EXPORTS_FOLDER: &str = "./exports";
const VOL_PATH: &str = "./tmp/vol100.rootfs";

type TestFileFun = fn(&PathBuf, &PathBuf);

fn compare_files_text(original_path: &PathBuf, exported_path: &PathBuf) {
    let original_file = fs::read_to_string(original_path).unwrap();
    let export_file = fs::read_to_string(exported_path).unwrap();

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

fn compare_files_bin(original_path: &PathBuf, exported_path: &PathBuf) {
    const BUFFER_SIZE: usize = 8192;

    let file1 = File::open(original_path).unwrap();
    let file2 = File::open(exported_path).unwrap();

    let mut reader1 = BufReader::new(file1);
    let mut reader2 = BufReader::new(file2);

    let mut buffer1 = [0u8; BUFFER_SIZE];
    let mut buffer2 = [0u8; BUFFER_SIZE];

    loop {
        let read1 = reader1.read(&mut buffer1).unwrap();
        let read2 = reader2.read(&mut buffer2).unwrap();

        if read1 != read2 {
            panic!("Files are different !");
        }

        if read1 == 0 {
            break; // EOF su entrambi
        }

        if buffer1[..read1] != buffer2[..read2] {
            panic!("Files are different !");
        }
    }
}

fn test_files(file_path: &str, compare: TestFileFun) {
    let vfolder = "/home";
    let assets_path = Path::new(ASSETS_FOLDER);
    let assets_file_path = assets_path.join(file_path);

    let exports_path = Path::new(EXPORTS_FOLDER);
    let export_file_path = exports_path.join(file_path);

    //let dev1 = Device::new(DEVIDE_UID.into());

    if let Err(_) = fs::remove_file(VOL_PATH) {
        println!("Vol file doest find");
    }

    if let Err(_) = fs::remove_file(export_file_path.clone()) {
        println!("Export file doest find");
    }

    let user_uid = Uuid::parse_str(USER_UID).unwrap();
    let file = XFile::new(user_uid, &assets_file_path, vfolder.into()).unwrap();

    file.export_path(&export_file_path).unwrap();

    compare(&assets_file_path, &export_file_path);
}

macro_rules! generate_file_text_tests {
    ($(($name:ident, $file1:expr)),* $(,)?) => {
        $(
            #[test]
            fn $name() {
                test_files($file1, compare_files_text);
            }
        )*
    };
}

generate_file_text_tests! {
    (file_text_1, "text/README.md"),
    (file_text_2, "text/Satoshi_Nakamoto.html"),
}

macro_rules! generate_file_bin_tests {
    ($(($name:ident, $file1:expr)),* $(,)?) => {
        $(
            #[test]
            fn $name() {
                test_files($file1, compare_files_bin);
            }
        )*
    };
}

generate_file_bin_tests! {
    (file_bin_1, "bin/Pac_2005.exe"),
}