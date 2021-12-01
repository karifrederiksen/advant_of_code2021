fn parse(s: &str) -> Vec<u32> {
    s.split("\n")
        .filter_map(|s| s.parse::<u32>().ok())
        .collect()
}

// O(n) for any WIN_SIZE. Neat!
fn count_increasing<const WIN_SIZE: usize>(meas: &[u32]) -> u32 {
    let mut window_idx: usize = 0;
    let mut window: [u32; WIN_SIZE] = [0; WIN_SIZE];
    window.copy_from_slice(&meas[0..WIN_SIZE]);

    let mut window_value: u32 = window.iter().sum();
    let mut prev_window_value: u32 = window_value;
    let mut count = 0;
    for &v in &meas[WIN_SIZE..] {
        window_value -= window[window_idx];
        window[window_idx] = v;
        window_value += v;
        window_idx = (window_idx + 1) % WIN_SIZE;
        if window_value > prev_window_value {
            count += 1;
        }
        prev_window_value = window_value;
    }
    count
}

fn main() {
    let measurements = parse(include_str!("inputs"));
    println!("Part1 = {}", count_increasing::<1>(&measurements));
    println!("Part2 = {}", count_increasing::<3>(&measurements));
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
        assert_eq!(count_increasing::<1>(&data), 7);
        assert_eq!(count_increasing::<3>(&data), 5);
    }
}
