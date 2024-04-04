use std::time::Instant;

use row_challenge::{obtain_data, preceding_line_start_idx};

fn main() {
    println!("loading file into memory");
    let data = obtain_data();

    println!("starting");
    let core_count = num_cpus::get();

    let size = data.len() as u64;
    let per_core_len = (size as usize + core_count - 1) / core_count;

    let mut start_idx = 0;

    let start = Instant::now();

    std::thread::scope(|s| {
        for i in 1..=core_count {
            let data = if i == core_count {
                &data[start_idx..]
            } else {
                let tentative_end_idx = start_idx + per_core_len;
                let end_idx = preceding_line_start_idx(&data, tentative_end_idx);
                let data = &data[start_idx..=end_idx];

                start_idx = end_idx + 1;
                data
            };

            if i == core_count {
                row_challenge::utils::read_delimiters(data);
            } else {
                s.spawn(|| {
                    row_challenge::utils::read_delimiters(data);
                });
            }
        }
    });

    let seconds = start.elapsed().as_millis() as f64 / 1000.0;
    println!("elapsed: {seconds:.3}s");
}
