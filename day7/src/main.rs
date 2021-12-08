#![feature(int_abs_diff)]
#![feature(test)]
extern crate test;

fn parse(s: &str) -> Vec<u32> {
    s.split(',')
        .map(|n| n.parse::<u32>().expect("number"))
        .collect()
}

fn answer_part1(poss: &Vec<u32>) -> u32 {
    let max = poss.iter().copied().max().unwrap();
    (0..=max)
        .map(|p| poss.iter().map(|&n| n.abs_diff(p)).sum())
        .min()
        .unwrap()
}

fn answer_part2(poss: &Vec<u32>) -> u32 {
    let max = poss.iter().copied().max().unwrap();
    (0..=max)
        .map(|p| poss.iter().map(|&n| tri(n.abs_diff(p))).sum())
        .min()
        .unwrap()
}

fn tri(n: u32) -> u32 {
    (n * (n + 1)) / 2
}

fn main() {
    let poss = parse(include_str!("inputs"));
    println!(
        "Part 1 = {}\nPart 2 = {}",
        answer_part1(&poss),
        answer_part2(&poss)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &'static str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn test_tri() {
        assert_eq!(0, tri(0));
        assert_eq!(1, tri(1));
        assert_eq!(3, tri(2));
        assert_eq!(6, tri(3));
        assert_eq!(10, tri(4));
        assert_eq!(15, tri(5));
        assert_eq!(21, tri(6));
    }

    #[test]
    fn test_part1() {
        let positions = parse(EXAMPLE_INPUT);
        assert_eq!(37, answer_part1(&positions));
    }

    #[test]
    fn test_part2() {
        let positions = parse(EXAMPLE_INPUT);
        assert_eq!(168, answer_part2(&positions));
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
        let vents = parse(include_str!("inputs"));
        b.iter(|| {
            test::black_box(answer_part1(&vents));
        });
    }

    #[bench]
    fn _answer_part2(b: &mut test::Bencher) {
        let vents = parse(include_str!("inputs"));
        b.iter(|| {
            test::black_box(answer_part2(&vents));
        });
    }
}
