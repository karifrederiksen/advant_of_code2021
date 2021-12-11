#![feature(test)]
extern crate test;
use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq)]
struct CharSet {
    buf: [char; 7],
    len: u8,
}

impl CharSet {
    fn from_str(s: &str) -> Self {
        let mut buf = ['~'; 7];
        let mut i = 0;
        for c in s.chars() {
            buf[i] = c;
            i += 1;
        }
        buf.sort();
        Self { buf, len: i as u8 }
    }

    fn len(&self) -> usize {
        self.len as usize
    }

    fn is_superset(&self, other: &Self) -> bool {
        (0..other.len()).all(|i| self.buf[..self.len()].contains(&other.buf[i]))
    }

    fn except(&mut self, other: &Self) {
        let mut remove_count = 0;
        for &c in &other.buf[..other.len()] {
            for i in 0..self.len() {
                if self.buf[i] == c {
                    self.buf[i] = '~';
                    remove_count += 1;
                }
            }
        }
        self.buf.sort();
        self.len -= remove_count;
    }
}

impl Default for CharSet {
    fn default() -> Self {
        Self {
            buf: ['~'; 7],
            len: 0,
        }
    }
}

impl fmt::Debug for CharSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"")?;
        for &c in &self.buf[..self.len()] {
            write!(f, "{}", c)?;
        }
        write!(f, "\"")
    }
}

#[derive(Debug)]
struct Data {
    samples: [CharSet; 10],
    outputs: [CharSet; 4],
}

fn parse_line(s: &str) -> Data {
    let (part1, part2) = s.split_once('|').expect("delimited");
    let mut samples: [CharSet; 10] = Default::default();
    let mut i = 0;
    for s in part1.split(' ').filter(|&s| s != "") {
        samples[i] = CharSet::from_str(s);
        i += 1;
    }
    let mut outputs: [CharSet; 4] = Default::default();
    let mut i = 0;
    for s in part2.split(' ').filter(|&s| s != "") {
        outputs[i] = CharSet::from_str(s);
        i += 1;
    }

    Data { samples, outputs }
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
    let mut map: [CharSet; 10] = Default::default();
    for pat in data.samples.iter() {
        match pat.len() {
            2 => map[1] = *pat,
            3 => map[7] = *pat,
            4 => map[4] = *pat,
            7 => map[8] = *pat,
            _ => {}
        }
    }
    let mut fourdiff = map[4].clone();
    fourdiff.except(&map[1]);

    for pat in data.samples.iter() {
        match pat.len() {
            5 => {
                if pat.is_superset(&map[1]) {
                    map[3] = *pat;
                } else if pat.is_superset(&fourdiff) {
                    map[5] = *pat;
                } else {
                    map[2] = *pat;
                }
            }
            6 => {
                if pat.is_superset(&map[4]) {
                    map[9] = *pat;
                } else if pat.is_superset(&map[1]) {
                    map[0] = *pat;
                } else {
                    map[6] = *pat;
                }
            }
            _ => {}
        }
    }
    data.outputs
        .iter()
        .enumerate()
        .map(|(i, o)| {
            let n = map
                .iter()
                .enumerate()
                .filter(|x| x.1 == o)
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
