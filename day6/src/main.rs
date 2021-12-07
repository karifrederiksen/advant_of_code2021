#![feature(test)]
extern crate test;

#[derive(Debug, Clone, Copy)]
struct Model {
    adults: [u64; 7],
    young: [u64; 2],
}

impl Model {
    pub fn parse(s: &str) -> Self {
        let mut adults = [0; 7];
        let young = [0; 2];
        for i in s.split(',').map(|n| n.parse::<usize>().expect("number")) {
            adults[i] += 1;
        }
        Self { adults, young }
    }

    pub fn step(&mut self) {
        let new_adults = self.young[0];
        self.young[0] = self.young[1];
        self.young[1] = self.adults[0];
        self.adults[0] = self.adults[1];
        self.adults[1] = self.adults[2];
        self.adults[2] = self.adults[3];
        self.adults[3] = self.adults[4];
        self.adults[4] = self.adults[5];
        self.adults[5] = self.adults[6];
        self.adults[6] = self.young[1] + new_adults;
    }

    pub fn total(&self) -> u64 {
        self.adults.iter().map(|&n| n as u64).sum::<u64>()
            + self.young.iter().map(|&n| n as u64).sum::<u64>()
    }
}

fn answer_part1(mut model: Model) -> u64 {
    for _ in 0..80 {
        model.step();
    }
    model.total()
}

fn answer_part2(mut model: Model) -> u64 {
    for _ in 0..256 {
        model.step();
    }
    model.total()
}

fn main() {
    let model = Model::parse(include_str!("inputs"));
    println!(
        "Part 1 = {}\nPart 2 = {}",
        answer_part1(model),
        answer_part2(model)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &'static str = "3,4,3,1,2";

    #[test]
    fn test_part1() {
        let model = Model::parse(EXAMPLE_INPUT);
        assert_eq!(5934, answer_part1(model));
    }

    #[test]
    fn test_part2() {
        let model = Model::parse(EXAMPLE_INPUT);
        assert_eq!(26984457539, answer_part2(model));
    }

    #[bench]
    fn _parse(b: &mut test::Bencher) {
        let input = include_str!("inputs");
        b.iter(|| {
            test::black_box(Model::parse(input));
        });
    }

    #[bench]
    fn _answer_part1(b: &mut test::Bencher) {
        let model = Model::parse(include_str!("inputs"));
        b.iter(|| {
            test::black_box(answer_part1(model));
        });
    }

    #[bench]
    fn _answer_part2(b: &mut test::Bencher) {
        let model = Model::parse(include_str!("inputs"));
        b.iter(|| {
            test::black_box(answer_part2(model));
        });
    }
}
