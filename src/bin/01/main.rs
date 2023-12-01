#![feature(test)]

use std::io::Lines;

type Solution = u32;
pub type ParseOutput = Vec<String>;
const MAIN_INPUT: &str = include_str!("main_input");
const TEST_INPUT: &str = include_str!("test_input");
const TEST_INPUT_2: &str = include_str!("test_input_2");

pub fn parse(file: &str) -> ParseOutput {
    file.lines().map(|l| l.to_string()).collect()
}

fn part_1(parse_output: &ParseOutput) -> Solution {
    let mut solution = 0;
    for l in parse_output {
        let mut first = 0;
        let mut last = 0;
        for c in l.chars() {
            if let Some(d) = c.to_digit(10) {
                if first == 0 {
                    first = d * 10;
                }
                last = d;
            }
        }
        solution = solution + first + last;
    }
    solution
}

fn part_2(parse_output: &ParseOutput) -> Solution {
    let mut solution = 0;
    let mut current_word: String = "".into();
        for l in parse_output {
        let mut first = 0;
        let mut last = 0;
        for c in l.chars() {
            let mut num = c.to_digit(10);
            if num.is_none() {
                current_word.push(c);
                let n = if current_word.contains("one") {
                    Some(1)
                } else if current_word.contains("two") {
                    Some(2)
                } else if current_word.contains("three") {
                    Some(3)
                } else if current_word.contains("four") {
                    Some(4)
                } else if current_word.contains("five") {
                    Some(5)
                } else if current_word.contains("six") {
                    Some(6)
                } else if current_word.contains("seven") {
                    Some(7)
                } else if current_word.contains("eight") {
                    Some(8)
                } else if current_word.contains("nine") {
                    Some(9)
                } else {
                    None
                };
                if n.is_some() {
                    current_word.clear();
                    current_word.push(c);
                }
                num = n;
            } else {
                current_word.clear();
            }

            if let Some(d) = num {
                current_word.clear();
                if c.to_digit(10).is_none() {
                    current_word.push(c);
                }
                if first == 0 {
                    first = d * 10;
                }
                last = d;
            }
        }
        solution = solution + first + last;
    }
    solution
}

fn main() {
    let parse_output = parse(MAIN_INPUT);
    println!("Solution to part 1 is {}", part_1(&parse_output));
    println!("Solution to part 2 is {}", part_2(&parse_output));
}

#[cfg(test)]
mod tests {
    extern crate test;
    use super::*;
    use test::{black_box, Bencher};

    #[test]
    pub fn test_part_1() {
        let parse_output = parse(TEST_INPUT);
        assert_eq!(part_1(&parse_output), 142);
    }

    #[test]
    pub fn test_part_2() {
        let parse_output = parse(TEST_INPUT_2);
        assert_eq!(part_2(&parse_output), 281);
    }

    #[bench]
    fn bench_parse(b: &mut Bencher) {
        b.iter(|| {
            let _ = parse(MAIN_INPUT);
        });
    }

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        let parse_output = parse(MAIN_INPUT);
        b.iter(move || {
            assert_eq!(part_1(black_box(&parse_output)), 54877);
        });
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let parse_output = parse(MAIN_INPUT);
        b.iter(|| {
            assert_eq!(part_2(black_box(&parse_output)), 54100);
        });
    }
}
