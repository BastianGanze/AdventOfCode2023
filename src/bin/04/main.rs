#![feature(test)]

use std::collections::HashSet;

type Solution = u32;
pub type ParseOutput = Vec<(HashSet<u32>, Vec<u32>)>;
const MAIN_INPUT: &str = include_str!("main_input");
const TEST_INPUT: &str = include_str!("test_input");

pub fn parse(file: &str) -> ParseOutput {
    file.lines()
        .map(|l| {
            let (_, a) = l.split_once(':').unwrap();
            let (win, norm) = a.split_once('|').unwrap();
            (
                win.split(' ')
                    .filter(|c| !c.is_empty())
                    .map(|n| n.trim().parse().unwrap())
                    .collect(),
                norm.split(' ')
                    .filter(|c| !c.is_empty())
                    .map(|n| n.trim().parse().unwrap())
                    .collect(),
            )
        })
        .collect()
}

fn part_1(cards: &ParseOutput) -> Solution {
    cards
        .iter()
        .map(|(winning_numbers, real_numbers)| {
            let count = real_numbers
                .iter()
                .filter(|r| winning_numbers.contains(r))
                .count() as u32;
            if count == 0 {
                return 0;
            }
            u32::pow(2, count - 1)
        })
        .sum()
}

fn part_2(cards: &mut ParseOutput) -> Solution {
    let copies_of_cards: Vec<usize> = cards
        .iter()
        .map(|(winning_numbers, real_numbers)| {
            real_numbers
                .iter()
                .filter(|r| winning_numbers.contains(r))
                .count()
        })
        .collect();
    let mut cards_per_card: Vec<u32> = Vec::with_capacity(copies_of_cards.len());
    for _ in 0..copies_of_cards.len() {
        cards_per_card.push(1)
    }
    for (c_i, _) in cards.iter().enumerate() {
        for _ in 0..cards_per_card[c_i] {
            for copy_i in 1..=copies_of_cards[c_i] {
                cards_per_card[c_i + copy_i] += 1;
            }
        }
    }
    cards_per_card.iter().sum()
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
        assert_eq!(part_1(&parse_output), 13);
    }

    #[test]
    pub fn test_part_2() {
        let parse_output = &mut parse(TEST_INPUT);
        assert_eq!(part_2(parse_output), 30);
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
            assert_eq!(part_1(black_box(&parse_output)), 26914);
        });
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let parse_output = &mut parse(MAIN_INPUT);
        b.iter(|| {
            assert_eq!(part_2(black_box(parse_output)), 13080971);
        });
    }
}
