#![feature(test)]

use std::collections::VecDeque;
use std::convert::TryInto;

type Solution = usize;
pub type ParseOutput<'a> = Vec<&'a str>;
const MAIN_INPUT: &str = include_str!("main_input");
const TEST_INPUT: &str = include_str!("test_input");

fn part_1(hashes: &ParseOutput) -> Solution {
    hashes
        .iter()
        .map(|h| {
            h.as_bytes().iter().fold(0 as Solution, |acc, c| {
                ((acc + (*c) as Solution) * 17) % 256
            }) as Solution
        })
        .sum::<Solution>() as Solution
}

#[derive(Debug)]
struct Boxxu<'a> {
    lenses: VecDeque<(&'a str, u8)>,
}

impl<'a> Boxxu<'a> {
    pub fn default() -> Boxxu<'a> {
        Boxxu {
            lenses: VecDeque::new(),
        }
    }
}

#[derive(Debug)]
enum Operation {
    Equal(u8),
    Minus,
}

fn part_2(instructions: &mut ParseOutput) -> Solution {
    let mut boxes = std::iter::repeat_with(|| Boxxu::default())
        .take(256)
        .collect::<Vec<Boxxu>>();
    for instruction in instructions {
        let bytes = instruction.as_bytes();
        let mut operation_position = bytes.iter().position(|c| *c == b'=' || *c == b'-').unwrap();
        let box_label = &instruction[..operation_position];
        let mut box_i: usize = bytes[..operation_position]
            .iter()
            .fold(0, |acc, c| ((acc + (*c) as usize) * 17) % 256);
        let mut operation = match bytes[operation_position..][0] {
            b'=' => Operation::Equal(
                (bytes[operation_position..][1] as char)
                    .to_digit(10)
                    .unwrap()
                    .try_into()
                    .unwrap(),
            ),
            b'-' => Operation::Minus,
            d => panic!("{}", d as char),
        };
        let boxxu = &mut boxes[box_i];
        match operation {
            Operation::Equal(focus) => match boxxu.lenses.iter().position(|l| l.0 == box_label) {
                Some(lens_i) => {
                    boxxu.lenses[lens_i] = (box_label, focus);
                }
                None => {
                    boxxu.lenses.push_back((box_label, focus));
                }
            },
            Operation::Minus => {
                if let Some(lens_i) = boxxu.lenses.iter().position(|l| l.0 == box_label) {
                    boxxu.lenses.remove(lens_i);
                }
            }
        }
    }
    boxes
        .iter()
        .enumerate()
        .map(|(b_i, b)| {
            b.lenses
                .iter()
                .enumerate()
                .map(|(l_i, l)| (b_i + 1) * (l_i + 1) * (l.1 as usize))
                .sum::<Solution>()
        })
        .sum::<Solution>()
}

pub fn parse<'a>(file: &str) -> ParseOutput {
    file.trim_end_matches('\n').split(',').collect()
}
fn main() {
    let parse_output = &mut parse(MAIN_INPUT);
    println!("Solution to part 1 is {}", part_1(parse_output));
    println!("Solution to part 2 is {}", part_2(parse_output));
}

#[cfg(test)]
mod tests {
    extern crate test;

    use test::{black_box, Bencher};

    use super::*;

    #[test]
    pub fn test_part_1() {
        let parse_output = parse(TEST_INPUT);
        assert_eq!(part_1(&parse_output), 1320);
    }

    #[test]
    pub fn test_part_2() {
        let parse_output = &mut parse(TEST_INPUT);
        assert_eq!(part_2(parse_output), 145);
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
            assert_eq!(part_1(black_box(&parse_output)), 513172);
        });
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let parse_output = &mut parse(MAIN_INPUT);
        b.iter(|| {
            assert_eq!(part_2(black_box(parse_output)), 237806);
        });
    }
}
