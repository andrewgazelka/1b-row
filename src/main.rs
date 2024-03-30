use std::{
    fs::File,
    io,
    io::{BufReader, Cursor, Read},
    os::unix::fs::MetadataExt,
};

use memmap2::MmapOptions;

fn main() {
    let mut file = File::open("measurements.txt").unwrap();
    let meta = file.metadata().unwrap();
    let size = meta.size();

    // Create a memory-mapped file
    // let mmap = unsafe { MmapOptions::new().map(&file).unwrap() };

    // create vec of size / 8
    let mut data = vec![0; size as usize];
    file.read_exact(&mut data).unwrap();

    let res = row_challenge::utils::read_delimiters(&data);
    println!("{res:?}");
}
