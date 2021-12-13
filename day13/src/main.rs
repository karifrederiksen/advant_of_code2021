#![feature(test)]

extern crate test;

#[derive(Debug, Clone, Copy)]
enum Fold {
    Vertical(usize),
    Horizontal(usize),
}

type Grid = Vec<Vec<bool>>;
type Input = (Grid, Vec<Fold>);

fn parse(s: &str) -> Input {
    let (dots, instrs) = s.split_once("\n\n").unwrap();
    let dots: Vec<(usize, usize)> = dots
        .split('\n')
        .map(|d| {
            let (x, y) = d.split_once(',').unwrap();
            (
                x.parse::<usize>().expect("int"),
                y.parse::<usize>().expect("int"),
            )
        })
        .collect();

    let (width, height) = dots
        .iter()
        .fold((0, 0), |(xm, ym), &(x, y)| (xm.max(x), ym.max(y)));
    let mut grid = vec![vec![false; width + 1]; height + 1];

    for (x, y) in dots.into_iter() {
        grid[y][x] = true;
    }

    let folds: Vec<Fold> = instrs.split('\n').map(|l| if l.contains("fold along y=") {
        Fold::Vertical(l.chars().skip(13).collect::<String>().parse::<usize>().expect("int"))
    } else if l.contains("fold along x=") {
        Fold::Horizontal(l.chars().skip(13).collect::<String>().parse::<usize>().expect("int"))
    } else {
        panic!("unexpected fold {}", l)
    })
    .collect();

    (grid, folds)
}

fn step(grid: &mut Grid, fold: Fold) {
    match fold {
        Fold::Horizontal(split_x) => {
            for y in 0..grid.len() {
                for x in 0..split_x {
                    let from_x = grid[y].len() - 1 - x;
                    grid[y][x] |= grid[y][from_x];
                }
                grid[y].truncate(split_x);
            }
        }
        Fold::Vertical(split_y) => {
            for y in 0..split_y {
                for x in 0..grid[y].len() {
                    let from_y = grid.len() - 1 - y;
                    grid[y][x] |= grid[from_y][x];
                }
            }
            grid.truncate(split_y);
        }
    }
}

fn answer_part1(mut grid: Grid, folds: &[Fold]) -> usize {
    step(&mut grid, folds[0]);
    grid.iter().map(|l| l.iter().filter(|b| **b).count()).sum()
}

fn answer_part2(mut grid: Grid, folds: &[Fold]) -> String {
    for fold in folds.iter() {
        step(&mut grid, *fold);
    }
    let mut s = String::new();
    for row in grid {
        for cell in row {
            if cell {
                s.push('#');
            } else {
                s.push('.');
            }
        }
        s.push('\n');
    }
    s
}

fn main() {
    let (grid, folds) = parse(include_str!("inputs"));
    println!("Part 1 = {}", answer_part1(grid.clone(), &folds));
    println!("Part 2 = \n{}", answer_part2(grid.clone(), &folds));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &'static str = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

    #[test]
    fn _part1() {
        let (grid, folds) = parse(EXAMPLE_INPUT);
        assert_eq!(17, answer_part1(grid, &folds));
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
        let (grid, folds) = parse(include_str!("inputs"));
        b.iter(|| {
            test::black_box(answer_part1(grid.clone(), &folds));
        });
    }

    #[bench]
    fn bench_answer_part2(b: &mut test::Bencher) {
        let (grid, folds) = parse(include_str!("inputs"));
        b.iter(|| {
            test::black_box(answer_part2(grid.clone(), &folds));
        });
    }
}