// https://github.com/rust-lang/portable-simd/blob/master/beginners-guide.md
// https://rust-lang.github.io/portable-simd/core_simd/simd/index.html
use std::{
    collections::HashMap,
    fmt::Display,
    ops::BitAnd,
    simd::{
        cmp::{SimdPartialEq, SimdPartialOrd},
        Simd,
    },
};
use fxhash::FxHashMap;

#[derive(Default)]
struct Stats {
    min: f64,
    sum: f64,
    max: f64,
    count: u64,
}

impl Stats {
    fn update(&mut self, value: f64) {
        self.min = self.min.min(value);
        self.max = self.max.max(value);
        self.sum += value;
        self.count += 1;
    }
}

impl Display for Stats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let avg = self.sum / self.count as f64;
        write!(f, "min: {}, avg: {}, max: {}", self.min, avg, self.max)
    }
}

#[allow(clippy::cast_possible_truncation)]
pub fn read_delimiters(data: &[u8]) {
    let mut map: HashMap<u32, Stats, _> = FxHashMap::default();

    let semicolon: u8 = b';';
    let newline: u8 = b'\n';

    // todo lane count
    let chunk_size = 64;

    // // todo:
    let mut positions: Vec<i64> = Vec::new();

    positions.push(-1);

    let not_ascii_mask: Simd<u8, 64> = Simd::splat(0x80);

    let mut chunks_iter = data.chunks_exact(chunk_size).enumerate();

    // let slow_process = false;
    let mut position_idx = 0;

    let zero = Simd::splat(0);

    'main_loop: while let Some((chunk_idx, byte)) = chunks_iter.next() {
        // println!("loop");
        // println!("loop");
        let start_idx = chunk_idx as u64 * chunk_size as u64;

        let simd_chunk: Simd<u8, 64> = Simd::from_slice(byte);

        let not_ascii = simd_chunk.bitand(&not_ascii_mask);
        // println!("not ascii {:?}", not_ascii);
        // Mask::
        // println!("mask {:?}", mask);

        let non_ascii_count = not_ascii.simd_gt(zero).to_bitmask().count_ones();
        if non_ascii_count != 0 {
            // println!("raw hex bytes {:X?}", byte);
            // println!("non ascii count {}", non_ascii_count);
            // todo: can crash
            let s = unsafe { core::str::from_utf8_unchecked(&data[start_idx as usize..]) };

            let mut subtract = 0;

            // println!("not ascii ==");
            for (i, c) in s.char_indices() {
                // println!("{i} ascii {:?}", c);
                // println!("{i} -> {c:?}");
                let mut end_idx = start_idx + i as u64;
                // print!("{c}");
                if c == ';' || c == '\n' {
                    positions.push(start_idx as i64 + i as i64);
                }
                if !c.is_ascii() {
                    // println!("not ascii: {}", c);
                    // println!("not ascii: {}", c);
                    end_idx += c.len_utf8() as u64 - 1;
                }

                // todo: > or >=
                if end_idx - start_idx - subtract >= chunk_size as u64 {
                    // println!("next chunk lolol");
                    // ignore todo: jank
                    chunks_iter.next();
                    subtract += chunk_size as u64;
                    // start_idx += chunk_size as u64;
                }

                if end_idx as usize % chunk_size == chunk_size - 1 {
                    // println!("break");
                    // println!();
                    continue 'main_loop;
                }
            }

            // assert!(not_ascii_count == 0);

            continue 'main_loop;

            // let simd_chunk: Simd<u8, 64> = Simd::from_slice(byte);
        }

        let semicolon_mask = simd_chunk.simd_eq(Simd::splat(semicolon));
        let newline_mask = simd_chunk.simd_eq(Simd::splat(newline));

        let mut input = semicolon_mask.to_bitmask() | newline_mask.to_bitmask();

        // let ones = input.count_ones();

        while input > 0 {
            positions.push(start_idx as i64 + input.trailing_zeros() as i64);

            // input &= unsafe { input.unchecked_sub(1) };
            input &= input - 1;
        }

        while position_idx < positions.len() - 2 {
            let start = (positions[position_idx] + 1) as usize;
            let semi = positions[position_idx + 1] as usize;
            let end = positions[position_idx + 2] as usize;

            let key = &data[start..semi];
            let key = unsafe { std::str::from_utf8_unchecked(key) };

            let value = &data[semi + 1..end];
            let value = unsafe { std::str::from_utf8_unchecked(value) };
            let value = value.parse::<f64>().unwrap();

            let key = fxhash::hash32(key);
            let stats = map.entry(key).or_default();
            stats.update(value);

            position_idx += 2;
        }

        if chunk_idx % 1_000_000 == 0 {
            let pos_len = positions.len();
            // println!("line {:.2}M", pos_len as f64 / 2.0 / 1000.0 / 1000.0);
        }
    }

    positions.push(data.len() as i64);

    // todo: remainder
    //
    // let remainder_offset = data.len() % chunk_size;
    // let start = data.len() - remainder_offset;
    // //
    // for (i, &byte) in data[start..].iter().enumerate() {
    //     if byte == semicolon || byte == newline {
    //         positions.push(remainder_offset as i64 + i as i64);
    //     }
    // }
    //

    //

    // for (k, v) in map {
    //     println!("{k:?} -> {v:.2}");
    // }
}
// static INPUT: &[u8] = b"\
// Hiroshima;18.5
// Kathmandu;19.4
// Ankara;6.6
// Nassau;23.8
// Milan;14.0
// Los Angeles;14.1
// Los Angeles;14.1
// Los Angeles;14.1
// Los Angeles;14.1
// ";
//
// #[cfg(test)]
// mod tests {
//     #[test]
//     fn test_find_delimiters() {
//         // let result = super::read_delimeters(INPUT);
//         // println!("{:?}", result);
//     }
// }
