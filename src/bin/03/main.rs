#![feature(test)]

use std::cmp::{max, min};
use std::ops::Range;
type Solution = i32;

pub type YPosition = i32;
pub type XPositionRange = Range<i32>;
pub type Symbol = (XPositionRange, YPosition, char);
pub type Number = (XPositionRange, YPosition, i32);
pub type ParseOutput = Vec<(Vec<Number>, Vec<Symbol>)>;
const MAIN_INPUT: &str = include_str!("main_input");
const TEST_INPUT: &str = include_str!("test_input");

pub fn parse(file: &str) -> ParseOutput {
    file.lines()
        .enumerate()
        .filter(|(_i, l)| !l.is_empty())
        .map(|(l_i, l)| {
            let mut numbers = Vec::new();
            let mut symbols = Vec::new();
            let mut current_symbol_vec: Vec<char> = Vec::new();
            let mut current_num_start: Option<i32> = None;
            for (c_i, c) in l.chars().enumerate() {
                if c.is_ascii_digit() {
                    if current_num_start.is_none() {
                        current_num_start = Some(c_i as i32);
                    }
                    current_symbol_vec.push(c);
                }

                if (!c.is_ascii_digit() || (c_i == l.len() - 1 && !current_symbol_vec.is_empty()))
                    && current_num_start.is_some()
                {
                    numbers.push((
                        current_num_start.unwrap()..c_i as i32,
                        l_i.try_into().unwrap(),
                        current_symbol_vec
                            .iter()
                            .collect::<String>()
                            .parse()
                            .unwrap(),
                    ));
                    current_num_start = None;
                    current_symbol_vec.clear();
                }

                if !c.is_ascii_digit() && c != '.' {
                    symbols.push((c_i as i32..(c_i + 1) as i32, l_i.try_into().unwrap(), c));
                }
            }
            (numbers, symbols)
        })
        .collect()
}

fn part_1(lines: &ParseOutput) -> Solution {
    lines
        .iter()
        .flat_map(|(numbers, _)| {
            numbers
                .iter()
                .filter_map(|number| has_surrounding_symbol(lines, number))
                .collect::<Vec<i32>>()
        })
        .sum()
}

fn part_2(lines: &mut ParseOutput) -> Solution {
    lines
        .iter()
        .flat_map(|(_, symbols)| {
            symbols
                .iter()
                .map(|symbol| surrounding_numbers(lines, symbol))
                .collect::<Vec<Vec<i32>>>()
        })
        .filter(|n| n.len() == 2)
        .map(|n| n[0] * n[1])
        .sum()
}

fn has_surrounding_symbol(
    lines: &ParseOutput,
    (number_x_range, start_y, number): &Number,
) -> Option<i32> {
    for y in (start_y - 1)..=(start_y + 1) {
        if y < 0 {
            continue;
        }
        if let Some((_, symbols)) = lines.get(y as usize) {
            if symbols.iter().any(|(symbol_x_range, _, _)| {
                max(number_x_range.start, symbol_x_range.start.saturating_sub(1))
                    < min(number_x_range.end, symbol_x_range.start.saturating_add(2))
            }) {
                return Some(*number);
            }
        }
    }

    None
}

fn surrounding_numbers(
    lines: &ParseOutput,
    (symbol_x_range, start_y, symbol): &Symbol,
) -> Vec<i32> {
    let mut numbers_vec = Vec::new();
    if *symbol != '*' {
        return numbers_vec;
    }
    for y in (start_y - 1)..=(start_y + 1) {
        if y < 0 {
            continue;
        }
        if let Some((numbers, _)) = lines.get(y as usize) {
            numbers_vec.extend(numbers.iter().filter_map(|(number_x_range, _, number)| {
                let overlap = max(number_x_range.start, symbol_x_range.start.saturating_sub(1))
                    < min(number_x_range.end, symbol_x_range.start.saturating_add(2));
                if overlap {
                    Some(number)
                } else {
                    None
                }
            }));
        }
    }

    numbers_vec
}

fn main() {
    let parse_output = &mut parse(MAIN_INPUT);
    println!("Solution to part 1 is {}", part_1(parse_output));
    println!("Solution to part 2 is {}", part_2(parse_output));
}

#[cfg(test)]
mod tests {
    extern crate test;
    use super::*;
    use test::{black_box, Bencher};

    #[test]
    pub fn test_part_1() {
        let parse_output = parse(TEST_INPUT);
        assert_eq!(part_1(&parse_output), 4361);
    }

    #[test]
    pub fn test_part_2() {
        let parse_output = &mut parse(TEST_INPUT);
        assert_eq!(part_2(parse_output), 467835);
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
            assert_eq!(part_1(black_box(&parse_output)), 536202);
        });
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let parse_output = &mut parse(MAIN_INPUT);
        b.iter(|| {
            assert_eq!(part_2(black_box(parse_output)), 78272573);
        });
    }
}
