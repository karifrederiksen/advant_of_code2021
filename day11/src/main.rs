#![feature(test)]

extern crate test;

type Grid = Vec<Vec<u32>>;

fn parse(s: &str) -> Grid {
    s.split('\n')
        .map(|l| l.bytes().map(|b| (b - b'0') as u32).collect())
        .collect()
}

fn step(g: &mut Grid) -> usize {
    let height = g.len();
    let width = g[0].len();
    let mut to_visit: Vec::<(usize, usize)> = (0..g.len())
        .flat_map(|y| (0..g[y].len()).map(move |x| (x, y)))
        .collect();
    while let Some((x, y)) = to_visit.pop() {
        let n = g[y][x];
        if n == 9 {
            for y_ in y.saturating_sub(1)..=(y + 1).min(height - 1) {
                for x_ in x.saturating_sub(1)..=(x + 1).min(width - 1) {
                    if x_ != x || y_ != y {
                        to_visit.push((x_, y_));
                    }
                }
            }
        }
        g[y][x] = n + 1;
    }
    for row in g.iter_mut() {
        for cell in row.iter_mut() {
            if *cell > 9 {
                *cell = 0;
            }
        }
    }
    g.iter().map(|r| r.iter().filter(|&&c| c == 0).count()).sum()
}

fn answer_part1(mut g: Grid) -> usize {
    (0..100).map(|_| step(&mut g)).sum()
}

fn answer_part2(mut g: Grid) -> usize {
    let cell_count: usize = g.iter().map(|r| r.len()).sum();
    (0..)
        .map(|i| (i + 1, step(&mut g)))
        .filter(|(_, flashes)| *flashes == cell_count)
        .nth(0)
        .unwrap()
        .0
}

fn main() {
    let g = parse(include_str!("inputs"));
    println!(
        "Part 1 = {}\nPart 2 = {}",
        answer_part1(g.clone()),
        answer_part2(g)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &'static str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

#[test]
fn test_part1() {
    let g = parse(EXAMPLE_INPUT);
    assert_eq!(1656, answer_part1(g));
}
#[test]
fn test_part2() {
    let g = parse(EXAMPLE_INPUT);
    assert_eq!(195, answer_part2(g));
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
    let g = parse(include_str!("inputs"));
    b.iter(|| {
        test::black_box(answer_part1(g.clone()));
    });
}

#[bench]
fn _answer_part2(b: &mut test::Bencher) {
    let g = parse(include_str!("inputs"));
    b.iter(|| {
        test::black_box(answer_part2(g.clone()));
    });
}
}