#![feature(test)]
extern crate test;
use std::collections::HashSet;

#[derive(Debug)]
struct Data {
    samples: [String; 10],
    outputs: [String; 4],
}

fn parse_line(s: &str) -> Data {
    let (part1, part2) = s.split_once('|').expect("delimited");
    let mut patterns: [String; 10] = Default::default();
    let mut i = 0;
    for s in part1.split(' ').filter(|&s| s != "") {
        patterns[i] = s.to_string();
        i += 1;
    }
    let mut outputs: [String; 4] = Default::default();
    let mut i = 0;
    for s in part2.split(' ').filter(|&s| s != "") {
        outputs[i] = s.to_string();
        i += 1;
    }

    Data {
        samples: patterns,
        outputs,
    }
}

fn parse(s: &str) -> Vec<Data> {
    s.split('\n').map(parse_line).collect()
}

fn answer_part1(data: &[Data]) -> usize {
    data.iter()
        .map(|x| {
            x.outputs
                .iter()
                .map(|x| x.len())
                .filter(|&x| x == 2 || x == 3 || x == 4 || x == 7)
                .count()
        })
        .sum()
}

fn decode(data: &Data) -> u64 {
    let mut map: [HashSet<char>; 10] = Default::default();
    for pat in data.samples.iter().chain(&data.outputs) {
        match pat.len() {
            2 => map[1] = pat.chars().collect(),
            3 => map[7] = pat.chars().collect(),
            4 => map[4] = pat.chars().collect(),
            7 => map[8] = pat.chars().collect(),
            _ => {}
        }
    }
    let mut fourdiff = map[4].clone();
    fourdiff.retain(|c| !map[1].contains(c));

    for pat in data.samples.iter().chain(&data.outputs) {
        let pat: HashSet<char> = pat.chars().collect();
        match pat.len() {
            5 => {
                if pat.is_superset(&map[1]) {
                    map[3] = pat;
                } else if pat.is_superset(&fourdiff) {
                    map[5] = pat;
                } else {
                    map[2] = pat;
                }
            }
            6 => {
                if pat.is_superset(&map[4]) {
                    map[9] = pat;
                } else if pat.is_superset(&map[1]) {
                    map[0] = pat;
                } else {
                    map[6] = pat;
                }
            }
            _ => {}
        }
    }

    data.outputs
        .iter()
        .enumerate()
        .map(|(i, o)| {
            let o: HashSet<char> = o.chars().collect();
            let n = map
                .iter()
                .enumerate()
                .filter(|x| x.1 == &o)
                .map(|x| x.0)
                .nth(0)
                .unwrap() as u64;
            n * 10u32.pow(3 - i as u32) as u64
        })
        .sum()
}

fn answer_part2(data: &[Data]) -> u64 {
    data.iter().map(decode).sum()
}

fn main() {
    let data = parse(include_str!("inputs"));
    println!(
        "Part 1 = {}\nPart 2 = {}",
        answer_part1(&data),
        answer_part2(&data)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &'static str =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn test_part1() {
        let data = parse(EXAMPLE_INPUT);
        assert_eq!(26, answer_part1(&data));
    }
    #[test]
    fn test_part2() {
        let data = parse(EXAMPLE_INPUT);
        assert_eq!(61229, answer_part2(&data));
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
