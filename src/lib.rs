#![feature(portable_simd)]
#![feature(unchecked_math)]

use std::{fs::File, io::Read, os::unix::fs::MetadataExt};

pub mod utils;

pub fn obtain_data() -> Vec<u8> {
    let root = project_root::get_project_root().unwrap();

    let path = root.join("measurements.txt");
    let mut file = File::open(path).unwrap();
    let meta = file.metadata().unwrap();
    let size = meta.size();

    // create vec of size / 8
    let mut data = vec![0; size as usize];
    file.read_exact(&mut data).unwrap();

    data
}

pub fn obtain_partial_data() -> Vec<u8> {
    let root = project_root::get_project_root().unwrap();

    let path = root.join("measurements.txt");
    let mut file = File::open(path).unwrap();
    let meta = file.metadata().unwrap();
    let size = meta.size();
    let size = size / 32;

    // create vec of size / 8
    let mut data = vec![0; size as usize];
    file.read_exact(&mut data).unwrap();

    let last_idx = preceding_line_start_idx(&data, data.len() - 1);

    data.truncate(last_idx + 1);
    data
}
// return the closest index to the left that is a newline
pub fn preceding_line_start_idx(data: &[u8], idx: usize) -> usize {
    let mut idx = idx;

    while data[idx] != b'\n' {
        idx -= 1;
    }

    idx
}
