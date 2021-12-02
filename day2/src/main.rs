type Val = i32;

#[derive(Copy, Clone, Debug)]
enum Instr {
    Forward(Val),
    Down(Val),
    Up(Val),
}
fn answer((pos_x, pos_y): (Val, Val)) -> i64 {
    (pos_x as i64) * (pos_y as i64)
}

fn parse(s: &str) -> Vec<Instr> {
    s.split("\n")
        .flat_map(|l| {
            l.split_once(" ").map(|(tag, val)| {
                let val = val.parse::<Val>().expect("input not well formed");
                match tag {
                    "forward" => Instr::Forward(val),
                    "down" => Instr::Down(val),
                    "up" => Instr::Up(val),
                    _ => panic!("input not well formed"),
                }
            })
        })
        .collect()
}

fn eval_v1(moves: &[Instr]) -> (Val, Val) {
    let mut pos = (0, 0);
    for &inst in moves.iter() {
        match inst {
            Instr::Forward(val) => {
                pos.0 += val;
            }
            Instr::Down(val) => {
                pos.1 += val;
            }
            Instr::Up(val) => {
                pos.1 -= val;
            }
        }
    }
    pos
}

fn eval_v2(moves: &[Instr]) -> (Val, Val) {
    let mut pos = (0, 0);
    let mut aim = 0;
    for &inst in moves.iter() {
        match inst {
            Instr::Forward(val) => {
                pos.0 += val;
                pos.1 += val * aim;
            }
            Instr::Down(val) => {
                aim += val;
            }
            Instr::Up(val) => {
                aim -= val;
            }
        }
    }
    pos
}

fn main() {
    let moves = parse(&include_str!("inputs"));
    println!("Part1 = {:?}", answer(eval_v1(&moves)));
    println!("Part2 = {:?}", answer(eval_v2(&moves)));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &'static str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

    #[test]
    fn test() {
        let moves = parse(TEST_INPUT);
        assert_eq!(answer(eval_v1(&moves)), 150);
        assert_eq!(answer(eval_v2(&moves)), 900);
    }
}
