#![feature(test)]
extern crate test;

fn parse(s: &str) -> Vec<u32> {
    s.split("\n")
        .filter_map(|s| s.parse::<u32>().ok())
        .collect()
}

fn count_increasing(window_size: usize, meas: &[u32]) -> u32 {
    let mut count = 0;
    for out_idx in 0..(meas.len() - window_size) {
        let in_idx = out_idx + window_size;
        if meas[out_idx] < meas[in_idx] {
            count += 1;
        }
    }
    count
}

fn main() {
    let measurements = parse(include_str!("inputs"));
    println!(
        "Part1 = {}\nPart2 = {}",
        count_increasing(1, &measurements),
        count_increasing(3, &measurements)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &'static str = "199
200
208
210
200
207
240
269
260
263";

    #[test]
    fn test() {
        let data = parse(EXAMPLE_DATA);
        assert_eq!(count_increasing(1, &data), 7);
        assert_eq!(count_increasing(3, &data), 5);
    }

    #[bench]
    fn _parse(b: &mut test::Bencher) {
        let input = include_str!("inputs");
        b.iter(|| {
            test::black_box(parse(input));
        });
    }

    #[bench]
    fn _count_increasing_1(b: &mut test::Bencher) {
        let meas = parse(include_str!("inputs"));
        b.iter(|| {
            test::black_box(count_increasing(1, &meas));
        });
    }

    #[bench]
    fn _count_increasing_3(b: &mut test::Bencher) {
        let meas = parse(include_str!("inputs"));
        b.iter(|| {
            test::black_box(count_increasing(3, &meas));
        });
    }
}
