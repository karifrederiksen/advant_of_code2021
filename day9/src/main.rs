#![feature(test)]
extern crate test;
use std::collections::{HashSet};

type Grid = Vec<Vec<u8>>;

fn parse(s: &str) -> Grid {
    s.split('\n')
        .map(|l| l.bytes().map(|b| b - b'0').collect::<Vec<u8>>())
        .collect()
}

fn answer_part1(g: &Grid) -> u64 {
    let mut result = 0;
    for y in 0..g.len() {
        for x in 0..g[0].len() {
            let pts = neighbours(g, x, y);
            let n = g[y][x];
            if pts
                .iter()
                .all(|&(x, y)| n < g[y][x])
            {
                result += n as u64 + 1;
            }
        }
    }
    result
}

fn neighbours(g: &Grid, x: usize, y: usize) -> Vec<(usize, usize)> {
    let up = y
        .checked_sub(1)
        .and_then(|y| if y < g.len() { Some((x, y)) } else { None });
    let down = if y + 1 < g.len() {
        Some((x, y + 1))
    } else {
        None
    };
    let left = x.checked_sub(1).map(|x| (x, y));
    let right = if x + 1 < g[y].len() {
        Some((x + 1, y))
    } else {
        None
    };
    [up, right, down, left].iter().filter_map(|p| *p).collect()
}

fn answer_part2(g: &Grid) -> usize {
    let mut products: Vec<_> = 
        (0..g.len())
        .flat_map(|y| (0..g[y].len()).map(move |x| (x, y)))
        .filter_map(|(x, y)| {
            if neighbours(g, x, y)
                .iter()
                .all(|&(x_, y_)| g[y][x] < g[y_][x_])
            {
                Some((x, y))
            } else {
                None
            }
        })
        .into_iter()
        .map(|(x, y)| {
            let mut stack = vec![(x, y)];
            let mut visited = HashSet::<(usize, usize)>::new();
            
            while let Some((x, y)) = stack.pop() {
                if !visited.insert((x, y)) {
                    continue;
                }
                for (x_, y_) in neighbours(g, x, y).into_iter().filter(|&(x_, y_)| g[y_][x_] != 9 && g[y_][x_] > g[y][x]) {
                    stack.push((x_, y_));
                }
            }
            visited.len()
        })
        .collect();
    products.sort();
    products.into_iter().rev().take(3).product::<usize>()
}

fn main() {
    let g = parse(include_str!("inputs"));
    println!(
        "Part 1 = {}\nPart 2 = {}",
        answer_part1(&g),
        answer_part2(&g)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &'static str = "2199943210
3987894921
9856789892
8767896789
9899965678";

    #[test]
    fn test_part1() {
        let g = parse(EXAMPLE_INPUT);
        assert_eq!(15, answer_part1(&g));
    }
    #[test]
    fn test_part2() {
        let g = parse(EXAMPLE_INPUT);
        assert_eq!(1134, answer_part2(&g));
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
