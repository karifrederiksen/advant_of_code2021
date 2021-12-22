#![feature(test)]

extern crate test;

type Template = Vec<u8>;
type Rule = (u8, u8, u8);

type Input = (Template, Vec<Rule>);

fn parse(s: &str) -> Input {
    let (template, rules) = s.split_once("\n\n").unwrap();
    let template: Template = template.bytes().collect();

    let rules: Vec<Rule> = rules
        .split('\n')
        .map(|l| {
            let (l, r) = l.split_once(" -> ").unwrap();
            (
                l.bytes().nth(0).unwrap(),
                l.bytes().nth(1).unwrap(),
                r.bytes().nth(0).unwrap(),
            )
        })
        .collect();

    (template, rules)
}

fn step(template: Template, rules: &[Rule]) -> Template {
    let mut output = Template::new();
    let mut prev_c = template[0];
    for c in template.into_iter().skip(1) {
        for &(c1, c2, o) in rules {
            if c1 == prev_c && c2 == c {
                output.push(o);
                continue;
            }
            output.push(prev_c);
        }
        prev_c = c;
    }
    output.push(prev_c);
    output
}

fn answer_part1(mut template: Template, rules: &[Rule]) -> usize {
    for _ in 0..2 {
        template = step(template, rules);
    }
    let mut blyat = [0; 255];
    for &c in &template {
        blyat[c as usize] += 1;
    }
    let min_count = blyat.iter().min().unwrap();
    let n: usize = blyat.iter().sum();
    n - min_count
}

fn answer_part2(mut template: Template, rules: &[Rule]) -> usize {
    todo!()
}

fn main() {
    let (template, rules) = parse(include_str!("inputs"));
    println!("Part 1 = {}", answer_part1(template.clone(), &rules));
    println!("Part 2 = {}", answer_part2(template.clone(), &rules));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &'static str = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

    #[test]
    fn _step() {
        let (template, rules) = parse(EXAMPLE_INPUT);
        let template1 = step(template.clone(), &rules);
        let template2 = step(template1.clone(), &rules);
        let template3 = step(template2.clone(), &rules);
        let template4 = step(template3.clone(), &rules);
        assert_eq!("NNCB".as_bytes(), template);
        assert_eq!("NCNBCHB".as_bytes(), template1);
        assert_eq!("NBCCNBBBCBHCB".as_bytes(), template2);
        assert_eq!("NBBBCNCCNBBNBNBBCHBHHBCHB".as_bytes(), template3);
        assert_eq!("NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB".as_bytes(), template4);
    }
    #[test]
    fn _part1() {
        let (template, rules) = parse(EXAMPLE_INPUT);
        assert_eq!(0, answer_part1(template, &rules));
    }
    #[test]
    fn _part2() {
        let (template, rules) = parse(EXAMPLE_INPUT);
        assert_eq!(0, answer_part2(template, &rules));
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
        let (template, rules) = parse(include_str!("inputs"));
        b.iter(|| {
            test::black_box(answer_part1(template.clone(), &rules));
        });
    }

    #[bench]
    fn bench_answer_part2(b: &mut test::Bencher) {
        let (template, rules) = parse(include_str!("inputs"));
        b.iter(|| {
            test::black_box(answer_part2(template.clone(), &rules));
        });
    }
}