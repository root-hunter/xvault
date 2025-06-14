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

use uuid::Uuid;
use RootFS::schema::{device::Device, file::DFile, volume::Volume};

fn main() {
    let dev1 = Device::new("4754f539-a953-4dc4-ad37-7a8ab142218c".into());

    let mut vol1 = Volume::new("/home/roothunter/lab/RootFS/tmp/vol100.rootfs".into(), 10).unwrap();

    let user_uid = "da64d273-e31b-48ca-8184-c741a34cb92d";
    let user_uid = Uuid::parse_str(user_uid).unwrap();

    let file_path = "/home/roothunter/lab/RootFS/tests/README.md";
    let vpath = "/home";

    let mut file = DFile::new(user_uid, file_path.into(), vpath.into()).unwrap();

    vol1.add_chunks_from_file(&mut file);
    vol1.save().unwrap();

    println!("{:?}", file);
}
