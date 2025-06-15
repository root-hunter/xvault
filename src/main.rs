use std::path::Path;

use rand::rngs::StdRng;
use uuid::Uuid;
use xvault::engine::chunk::{ChunkHandler, CHUNK_SIZE};
use xvault::engine::device::Device;
use xvault::engine::xfile::{XFileHandler, XFileQuery};
use xvault::engine::{volume::Volume, xfile::XFile};

const USER_UID: &str = "da64d273-e31b-48ca-8184-c741a34cb92d";
const DEVIDE_UID: &str = "4754f539-a953-4dc4-ad37-7a8ab142218c";

use rand::SeedableRng;
use rand::seq::{IndexedRandom, SliceRandom};

fn main() {

}