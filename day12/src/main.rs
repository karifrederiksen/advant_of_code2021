#![feature(test)]
extern crate test;

use multimap::MultiMap;

type Input<'a> = Vec<(&'a str, &'a str)>;
type Path = (bool, Vec<String>);

fn parse(s: &str) -> Input {
    s
    .split('\n').map(|l| l.split_once('-').unwrap())
    .collect()
}

struct PathFinder {
    graph: MultiMap<String, String>,
    queue: Vec::<Path>,
}

impl PathFinder {
    fn new(inputs: &Input) -> Self {
        let mut graph = MultiMap::new();
        for &(a, b) in inputs.iter()
        {
            let a = a.to_string();
            let b = b.to_string();
            if a == "start" {
                graph.insert(a, b);
            } else if b == "start" {
                graph.insert(b, a);
            } else if a == "end" {
                graph.insert(b, a);
            } else if b == "end" {
                graph.insert(a, b);
            } else  {
                graph.insert(a.clone(), b.clone());
                graph.insert(b, a);
            }
        }
        Self {
            graph,
            queue: vec![(false, vec!["start".to_string()])],
        }
    }

    fn next_part1(&mut self) -> Option<Path> {
        let mut result: Option<Path> = None;
        loop {
            if result.is_some() {
                break;
            }
            let path = match self.queue.pop() {
                None => break,
                Some(p)=> p,
            };
            for next in self.graph
                .get_vec(path.1.last().unwrap())
                .unwrap()
                .iter()
                .filter(|&a| Self::is_upper(a) || !path.1.contains(a))
            {
                let mut next_path: Path = path.clone();
                next_path.1.push(next.to_string());
                if next == "end" && result.is_none() {
                    result = Some(next_path);
                } else {
                    self.queue.push(next_path);
                }
            }
        }
        result
    }

    fn next_part2(&mut self) -> Option<Path> {
        let mut result: Option<Path> = None;
        loop {
            if result.is_some() {
                break;
            }
            let path = match self.queue.pop() {
                None => break,
                Some(p)=> p,
            };
            for next in self.graph
                .get_vec(path.1.last().unwrap())
                .unwrap()
                .iter()
            {
                let mut next_path =
                    if !Self::is_upper(next) && !path.0 && path.1.contains(next) {
                        let mut p = path.clone();
                        p.0 = true;
                        p
                    } else if Self::is_upper(next) || !path.1.contains(next) {
                        path.clone()
                    } else {
                        continue;
                    };
                next_path.1.push(next.to_string());
                if next == "end" && result.is_none() {
                    result = Some(next_path);
                } else {
                    self.queue.push(next_path);
                }
            }
        }
        result
    }

    fn is_upper(s: &str) -> bool {
        s.chars().nth(0).unwrap().is_uppercase()
    }
}

fn answer_part1(inputs: &Input) -> usize {
    let mut pf = PathFinder::new(inputs);
    let mut count = 0;
    while let Some(_path) = pf.next_part1() {
        count += 1;
    }
    count
}

fn answer_part2(inputs: &Input) -> usize {
    let mut pf = PathFinder::new(inputs);
    let mut count = 0;
    while let Some(_path) = pf.next_part2() {
        count += 1;
    }
    count
}

fn main() {
    let inputs = parse(include_str!("inputs"));
    println!("Part 1 = {}", answer_part1(&inputs));
    println!("Part 2 = {}", answer_part2(&inputs));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &'static str = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";

    const EXAMPLE_INPUT_2: &'static str = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";

    #[test]
    fn _part1_a() {
        let inputs = parse(EXAMPLE_INPUT);
        assert_eq!(10, answer_part1(&inputs));
    }
    #[test]
    fn _part1_b() {
        let inputs = parse(EXAMPLE_INPUT_2);
        assert_eq!(226, answer_part1(&inputs));
    }
    #[test]
    fn _part2_a() {
        let inputs = parse(EXAMPLE_INPUT);
        assert_eq!(36, answer_part2(&inputs));
    }
    #[test]
    fn _part2_b() {
        let inputs = parse(EXAMPLE_INPUT_2);
        assert_eq!(3509, answer_part2(&inputs));
    }

    #[bench]
    fn bench_parse(b: &mut test::Bencher) {
        let input = include_str!("inputs");
        b.iter(|| {
            test::black_box(parse(input));
        });
    }

    #[bench]
    fn bench_answer_part1(b: &mut test::Bencher) {
        let inputs = parse(include_str!("inputs"));
        b.iter(|| {
            test::black_box(answer_part1(&inputs));
        });
    }

    #[bench]
    fn bench_answer_part2(b: &mut test::Bencher) {
        let inputs = parse(include_str!("inputs"));
        b.iter(|| {
            test::black_box(answer_part2(&inputs));
        });
    }
}