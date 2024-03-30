use std::{fs::File, hint::black_box};

use divan::{AllocProfiler, Bencher};

// #[global_allocator]
// static ALLOC: AllocProfiler = AllocProfiler::system();

fn main() {
    divan::main();
}

// Register a `fibonacci` function and benchmark it over multiple cases.
#[divan::bench]
fn all_delimiters(bencher: Bencher) {
    bencher.bench(|| {
        let file = File::open("measurements.txt").unwrap();
        let result = row_challenge::utils::read_delimiters(file);
        black_box(result);
    });
}
