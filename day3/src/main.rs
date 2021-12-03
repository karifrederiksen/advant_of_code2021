#![feature(test)]
extern crate test;

type BitVec = Vec<bool>;

fn parse(s: &str) -> Vec<BitVec> {
    s.split('\n')
        .map(|l| l.chars().map(|x| x == '1').collect::<BitVec>())
        .collect()
}

fn gamma_rate(lines: &[BitVec]) -> BitVec {
    assert!(lines.len() > 0, "requires 1 or more samples");
    (0..lines[0].len())
        .map(|i| lines.iter().map(|l| if l[i] { 1 } else { -1 }).sum::<i32>() >= 0)
        .collect::<BitVec>()
}

fn epsilon_rate(gamma: &BitVec) -> BitVec {
    gamma.into_iter().map(|x| !x).collect()
}

fn to_decimal(xs: BitVec) -> u64 {
    xs.into_iter()
        .fold(0u64, |sum, next| (sum << 1) + next as u64)
}

fn oxygen_generator_rating_base<P>(lines: &[BitVec], mut p: P) -> BitVec
where
    P: FnMut(usize, usize) -> bool,
{
    let mut lines: Vec<_> = lines.iter().collect();

    assert!(lines.len() > 0, "requires 1 or more samples");
    for i in 0..lines[0].len() {
        let ones_count = lines.iter().filter(|x| x[i]).count();
        if p(ones_count * 2, lines.len()) {
            lines.retain(|l| l[i]);
        } else {
            lines.retain(|l| !l[i]);
        }
        if lines.len() == 1 {
            return lines[0].clone();
        }
    }
    panic!("oxygen generator rating not found")
}

fn oxygen_generator_rating(lines: &[BitVec]) -> BitVec {
    oxygen_generator_rating_base(lines, |l, r| l >= r)
}

fn co2_scrubber_rating(lines: &[BitVec]) -> BitVec {
    oxygen_generator_rating_base(lines, |l, r| l < r)
}

fn answer_part1(lines: &[BitVec]) -> u64 {
    let gamma = gamma_rate(&lines);
    to_decimal(epsilon_rate(&gamma)) * to_decimal(gamma)
}

fn answer_part2(lines: &[BitVec]) -> u64 {
    let oxygen = oxygen_generator_rating(&lines);
    let co2 = co2_scrubber_rating(&lines);
    to_decimal(oxygen) * to_decimal(co2)
}

fn main() {
    let lines = parse(include_str!("inputs"));
    println!(
        "Part 1 = {}\nPart 2 = {}",
        answer_part1(&lines),
        answer_part2(&lines)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &'static str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    #[test]
    fn tests_part1() {
        let lines = parse(TEST_INPUT);
        assert_eq!(22, to_decimal(gamma_rate(&lines)));
        assert_eq!(9, to_decimal(epsilon_rate(&gamma_rate(&lines))));
        assert_eq!(198, answer_part1(&lines));
    }

    #[test]
    fn tests_part2() {
        let lines = parse(TEST_INPUT);
        assert_eq!(23, to_decimal(oxygen_generator_rating(&lines)));
        assert_eq!(10, to_decimal(co2_scrubber_rating(&lines)));
        assert_eq!(230, answer_part2(&lines));
    }

    #[bench]
    fn _parse(b: &mut test::Bencher) {
        let input = include_str!("inputs");
        b.iter(|| {
            test::black_box(parse(input));
        });
    }

    #[bench]
    fn _answer_part1(b: &mut test::Bencher) {
        let lines = parse(include_str!("inputs"));
        b.iter(|| {
            test::black_box(answer_part1(&lines));
        });
    }

    #[bench]
    fn _answer_part2(b: &mut test::Bencher) {
        let lines = parse(include_str!("inputs"));
        b.iter(|| {
            test::black_box(answer_part2(&lines));
        });
    }
}
