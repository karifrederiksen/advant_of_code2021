#![feature(exclusive_range_pattern)]
#![feature(test)]
extern crate test;

fn parse(s: &str) -> Vec<String> {
    s.split('\n')
        .map(|l| l.to_string())
        .collect()
}

fn parse_line(s: &str) -> (usize, Vec<char>) {
    let mut stack: Vec<char> = vec![];
    for c in s.chars() {
        match c {
            ')' => if stack.last() == Some(&'(') {
                stack.pop();
            } else {
                return (3, stack);
            }
            ']' => if stack.last() == Some(&'[') {
                stack.pop();
            } else {
                return (57, stack);
            }
            '}' => if stack.last() == Some(&'{') {
                stack.pop();
            } else {
                return (1197, stack);
            }
            '>' => if stack.last() == Some(&'<') {
                stack.pop();
            } else {
                return (25137, stack);
            }
            c => {
                stack.push(c);
            }
        }
    }
    (0, stack)
}

fn answer_part1(g: &Vec<String>) -> usize {
    g.iter().map(|s| parse_line(s.as_str()).0).sum()
}

fn answer_part2(g: &Vec<String>) -> usize {
    let mut scores: Vec<_> = g
        .iter()
        .filter_map(|s| match parse_line(s.as_str()) {
            (0, stack) => Some(stack),
            _ => None,
        })
        .map(|stack| {
            stack.iter().rev().fold(0, |sum, c| sum * 5 + match c {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                _ => panic!("unexpected char: {}", c)
            })
        })
        .collect();
    scores.sort();
    scores[scores.len() / 2]
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

    const EXAMPLE_INPUT: &'static str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn test_part1() {
        let g = parse(EXAMPLE_INPUT);
        assert_eq!(26397, answer_part1(&g));
    }
    #[test]
    fn test_part2() {
        let g = parse(EXAMPLE_INPUT);
        assert_eq!(288957, answer_part2(&g));
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
