#![feature(test)]
extern crate test;

use nom::{sequence::{self, preceded}, multi, character::complete::{char, u8, space1, space0}, bytes::complete::tag, IResult, combinator::map};

type Row = [u8; 5];
type Board = [Row; 5];

fn parse_row(s: &str) -> IResult<&str, Row> {
    map(sequence::tuple((
        preceded(space0, u8),
        preceded(space1, u8),
        preceded(space1, u8),
        preceded(space1, u8),
        preceded(space1, u8),
    )), |(a, b, c, d, e)| [a, b, c, d, e])(s)
}

fn parse_board(s: &str) -> IResult<&str, Board> {
    map(sequence::tuple((
        parse_row,
        preceded(char('\n'), parse_row),
        preceded(char('\n'), parse_row),
        preceded(char('\n'), parse_row),
        preceded(char('\n'), parse_row),
    )), |(a, b, c, d, e)| [a, b, c, d, e])(s)
}

fn parse(s: &str) -> IResult<&str, (Vec<u8>, Vec<Board>)> {
    let inputs = multi::separated_list1(char(','),u8);
    let boards = multi::separated_list1(tag("\n\n"), parse_board);
    
    sequence::tuple((inputs, preceded(tag("\n\n"), boards)))(s)
}

struct BingoSimulation<'a> {
    inputs: &'a [u8],
    boards: Vec<Board>
}
impl<'a> BingoSimulation<'a> {
    fn next_winner(&mut self) -> Option<(u8, Board)> {
        if self.boards.len() == 0 {
            return None;
        }
        let i = self.inputs[0];
        for idx in 0..self.boards.len() {
            let board = &mut self.boards[idx];
            for y in 0..5 {
                for x in 0..5 {
                    if board[y][x] == i {
                        board[y][x] = 255;
                    }                    
                }
            }
            for row in board.iter() {
                if row.iter().all(|&n| n == 255) {
                    return Some((i, self.boards.remove(idx)));
                }
            }
            for bidx in 0..5 {
                if board.iter().all(|row| row[bidx] == 255) {
                    return Some((i, self.boards.remove(idx)));
                }
            }
        }
        self.inputs = &self.inputs[1..];
        self.next_winner()
    }
}

fn answer_value(n: u8, board: Board) -> u32 {
    let unmarked_sum: u32 = board
        .into_iter()
        .flat_map(|row| row)
        .filter_map(|x| if x == 255 { None } else { Some(x as u32) })
        .sum();
    unmarked_sum * n as u32
}

fn answer_part1(inputs: &[u8], boards: Vec<Board>) -> u32 {
    let mut bingo = BingoSimulation { inputs, boards };
    let (n, board) = bingo.next_winner().expect("someone should win");
    answer_value(n, board)
}

fn answer_part2(inputs: &[u8], boards: Vec<Board>) -> u32 {
    let mut bingo = BingoSimulation { inputs, boards };
    let mut res = bingo.next_winner().expect("someone should win");
    while let Some(result2) = bingo.next_winner() {
        res = result2;
    }
    answer_value(res.0, res.1)
}

fn main() {
    let (inputs, boards) = parse(include_str!("inputs")).unwrap().1;
    println!("Part 1 = {:?}", answer_part1(&inputs, boards.clone()));
    println!("Part 2 = {:?}", answer_part2(&inputs, boards));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUTS: &'static str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    #[test]
    fn test_part1() {
        let (inputs, boards) = parse(EXAMPLE_INPUTS).unwrap().1;
        assert_eq!(4_512, answer_part1(&inputs, boards));
    }
    #[test]
    fn test_part2() {
        let (inputs, boards) = parse(EXAMPLE_INPUTS).unwrap().1;
        assert_eq!(1_924, answer_part2(&inputs, boards));
    }

    #[bench]
    fn _parse(b: &mut test::Bencher) {
        let input = include_str!("inputs");
        b.iter(|| {
            test::black_box(parse(input).unwrap());
        });
    }

    #[bench]
    fn _answer_part1(b: &mut test::Bencher) {
        let (inputs, boards) = parse(include_str!("inputs")).unwrap().1;
        b.iter(|| {
            test::black_box(answer_part1(&inputs, boards.clone()));
        });
    }

    #[bench]
    fn _answer_part2(b: &mut test::Bencher) {
        let (inputs, boards) = parse(include_str!("inputs")).unwrap().1;
        b.iter(|| {
            test::black_box(answer_part2(&inputs, boards.clone()));
        });
    }
}