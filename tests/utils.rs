use std::{
    fs::{self, File}, io::{BufReader, Read}, path::PathBuf
};

use similar::{Algorithm, TextDiff};

pub type FnCompareFile = fn(&PathBuf, &PathBuf);

pub fn compare_files_text(original_path: &PathBuf, exported_path: &PathBuf) {
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

pub fn compare_files_bin(original_path: &PathBuf, exported_path: &PathBuf) {
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