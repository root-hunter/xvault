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
    fs::{File}, io::{BufReader, Read}, path::PathBuf
};

pub fn compare_files(original_path: &PathBuf, exported_path: &PathBuf) {
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