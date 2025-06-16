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

use std::fs::{self, OpenOptions};
use xvault::engine::{volume::Volume};

#[test]
fn volume_test_set_max_size_from_disk() {
    let vol_path = "./tmp/vol33002.rootfs";
    let mut volume = Volume::new();
    let max_size = 100;

    volume
        .set_path(vol_path.to_string())
        .set_uid_from_device("4754f539-a953-4dc4-ad37-7a8ab142218c".into())
        .set_max_size(max_size)
        .build()
        .unwrap();

    fs::remove_file(vol_path).unwrap_or(());
    volume.alloc_on_disk().unwrap();

    let file = OpenOptions::new().read(true).open(&vol_path);
    let file = file.unwrap();

    // Now read it back
    assert!(volume.set_max_size_from_disk(&file).is_ok());
    assert_eq!(volume.max_size, max_size);
}

#[test]
fn volume_test_set_uid_from_disk() {
    let vol_path = "./tmp/vol35002.rootfs";
    let mut volume = Volume::new();

    let volume_uid = "4754f539-a953-4dc4-ad37-7a8ab142218c".to_string();

    volume
        .set_path(vol_path.to_string())
        .set_uid(volume_uid.clone())
        .set_max_size(100)
        .build()
        .unwrap();

    fs::remove_file(vol_path).unwrap_or(());
    volume.alloc_on_disk().unwrap();

    let file = OpenOptions::new().read(true).open(&vol_path);
    let file = file.unwrap();
    
    // Now read it back
    assert!(volume.set_uid_from_file(&file).is_ok());
    assert!(!volume.uid.is_empty());
    assert_eq!(volume.uid, volume_uid);
}