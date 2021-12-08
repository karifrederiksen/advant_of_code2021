#![feature(test)]
extern crate test;

type Vec2 = (i32, i32);

fn parse(s: &str) -> Vec<(Vec2, Vec2)> {
    s.split('\n')
        .map(|l| {
            let (l, r) = l.split_once(" -> ").expect("arrow");
            let (x1, y1) = l.split_once(',').expect("left comma");
            let (x2, y2) = r.split_once(',').expect("right comma");
            let start = (
                x1.parse::<i32>().expect("x1"),
                y1.parse::<i32>().expect("y1"),
            );
            let end = (
                x2.parse::<i32>().expect("x2"),
                y2.parse::<i32>().expect("y2"),
            );
            (start, end)
        })
        .collect()
}

struct Grid {
    grid: Vec<u8>,
    width: usize,
}

impl Grid {
    pub fn new(vents: &[(Vec2, Vec2)], diagonal: bool) -> Self {
        let (width, height) = Self::dimensions(vents);
        let mut grid = vec![0; width * height];

        for (start, end) in vents {
            if !diagonal && start.0 != end.0 && start.1 != end.1 {
                continue;
            }
            let (start, end) = if start.0 <= end.0 {
                (start, end)
            } else {
                (end, start)
            };
            let v = (
                (end.0 - start.0).clamp(-1, 1),
                (end.1 - start.1).clamp(-1, 1),
            );
            let mut pos = *start;
            grid[(pos.1 as usize) * width + (pos.0 as usize)] += 1;
            while pos != *end {
                pos = (pos.0 + v.0, pos.1 + v.1);
                grid[(pos.1 as usize) * width + (pos.0 as usize)] += 1;
            }
        }
        Self { grid, width }
    }

    fn dimensions(vents: &[(Vec2, Vec2)]) -> (usize, usize) {
        let mut x_max = usize::MIN;
        let mut y_max = usize::MIN;

        for (x1, x2, y1, y2) in vents
            .iter()
            .map(|&((x1, y1), (x2, y2))| (x1 as usize, x2 as usize, y1 as usize, y2 as usize))
        {
            // x_max = usize::max(x_max, usize::max(x1, x2));
            if x1 > x_max {
                x_max = x1;
            }
            if x2 > x_max {
                x_max = x2;
            }
            if y1 > y_max {
                y_max = y1;
            }
            if y2 > y_max {
                y_max = y2;
            }
        }
        (x_max + 1, y_max + 1)
    }

    pub fn dangerous_vents(&self) -> u32 {
        let mut count = 0;

        for n in &self.grid {
            if *n >= 2 {
                count += 1;
            }
        }

        count
    }

    #[allow(dead_code)]
    pub fn to_string(&self) -> String {
        let mut s = String::new();

        for (i, &n) in self.grid.iter().enumerate() {
            if n == 0 {
                s.push('.');
            } else {
                s.push_str(&format!("{}", n));
            }
            if i % self.width == (self.width - 1) {
                s.push('\n');
            }
        }

        s
    }
}

fn answer_part1(vents: &[(Vec2, Vec2)]) -> u32 {
    let grid = Grid::new(&vents, false);
    grid.dangerous_vents()
}

fn answer_part2(vents: &[(Vec2, Vec2)]) -> u32 {
    let grid = Grid::new(&vents, true);
    grid.dangerous_vents()
}

fn main() {
    let vents = parse(include_str!("inputs"));
    println!(
        "Part 1 = {}\nPart 2 = {}",
        answer_part1(&vents),
        answer_part2(&vents)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &'static str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn test_part1() {
        let vents = parse(EXAMPLE_INPUT);
        let grid = Grid::new(&vents, false);
        assert_eq!(5, grid.dangerous_vents());
    }

    #[test]
    fn test_part2() {
        let vents = parse(EXAMPLE_INPUT);
        let grid = Grid::new(&vents, true);
        assert_eq!(12, grid.dangerous_vents());
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
