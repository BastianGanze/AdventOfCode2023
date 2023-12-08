#![feature(test)]

mod card_type;

use crate::card_type::{char_to_card_type, CardType};
use std::cmp::Ordering;
use std::collections::HashMap;

type Solution = u32;

type Hand = [CardType; 5];

type Game = ([CardType; 5], Solution);
pub type ParseOutput = Vec<Game>;
const MAIN_INPUT: &str = include_str!("main_input");
const FIVE_OF_A_KIND: Solution = 7;
const FOUR_OF_A_KIND: Solution = 6;
const FULL_HOUSE: Solution = 5;
const THREE_OF_A_KIND: Solution = 4;
const TWO_PAIR: Solution = 3;
const ONE_PAIR: Solution = 2;
const HIGH_CARD: Solution = 1;
const TEST_INPUT: &str = include_str!("test_input");

pub fn parse(file: &str) -> ParseOutput {
    file.lines()
        .map(|l| {
            let (hand_chars, bid_chars) = l.split_once(' ').unwrap();
            let mut hand = [CardType::Q; 5];
            for (c_i, c) in hand_chars.chars().enumerate() {
                hand[c_i] = char_to_card_type(c).unwrap();
            }
            (hand, bid_chars.parse().unwrap())
        })
        .collect()
}

fn part_1(games: &ParseOutput) -> Solution {
    let mut ranked_games = games
        .iter()
        .map(|(a, b)| (hand_rank(a), a.clone(), *b))
        .collect::<Vec<(Solution, Hand, Solution)>>();
    ranked_games.sort_by(sort_games);
    ranked_games
        .iter()
        .enumerate()
        .map(|(i, (_, _, bid))| (i as Solution + 1) * bid)
        .sum()
}

fn sort_games(
    (rank_a, hand_a, _): &(Solution, Hand, Solution),
    (rank_b, hand_b, _): &(Solution, Hand, Solution),
) -> Ordering {
    match rank_a.cmp(rank_b) {
        Ordering::Equal => hand_a.cmp(hand_b),
        order => order,
    }
}

fn part_2(games: &mut ParseOutput) -> Solution {
    let mut ranked_games = games
        .iter()
        .map(|(a, b)| (a.map(|c| remap_to_make_joker_weak(c)), *b))
        .map(|(a, b)| (hand_rank_joker(&a), a, b))
        .collect::<Vec<(Solution, Hand, Solution)>>();

    ranked_games.sort_by(sort_games);

    ranked_games
        .iter()
        .enumerate()
        .map(|(i, (_, _, bid))| (i as Solution + 1) * bid)
        .sum()
}

fn hand_rank_joker(hand: &Hand) -> Solution {
    let mut counts: HashMap<CardType, Solution> = HashMap::new();

    for card in hand.iter() {
        *counts.entry(*card).or_insert(0) += 1;
    }

    let mut amounts_without_jokers = counts
        .iter()
        .filter(|(c, _)| &CardType::Two != *c)
        .map(|(a, b)| *b)
        .collect::<Vec<Solution>>();
    amounts_without_jokers.sort();
    let rank = match &amounts_without_jokers[..] {
        [_] => FIVE_OF_A_KIND, // No jokers
        [] => FIVE_OF_A_KIND,  // Only Jokers
        [a, b] => {
            let joker_amount = 5 - a - b;
            match joker_amount {
                0 => {
                    if *b == 4 {
                        FOUR_OF_A_KIND
                    } else {
                        FULL_HOUSE
                    }
                }
                1 => {
                    if *b == 3 && *a == 1 {
                        FOUR_OF_A_KIND
                    } else {
                        FULL_HOUSE
                    }
                }
                2 | 3 => FOUR_OF_A_KIND,
                _ => panic!(),
            }
        }
        [a, b, c] => {
            let joker_amount = 5 - a - b - c;
            match joker_amount {
                0 => {
                    if *c == 3 {
                        THREE_OF_A_KIND
                    } else {
                        TWO_PAIR
                    }
                }
                1 | 2 => THREE_OF_A_KIND,
                _ => panic!(),
            }
        }
        [a, b, c, d] => {
            let joker_amount = 5 - a - b - c - d;
            match joker_amount {
                0 => ONE_PAIR,
                1 => {
                    if *d == 2 {
                        THREE_OF_A_KIND
                    } else {
                        ONE_PAIR
                    }
                }
                _ => panic!(),
            }
        }
        [_, _, _, _, _] => HIGH_CARD,
        _ => panic!("No!!!"),
    };
    rank
}

fn hand_rank(hand: &Hand) -> Solution {
    let mut counts: HashMap<CardType, Solution> = HashMap::new();

    for card in hand.iter() {
        *counts.entry(*card).or_insert(0) += 1;
    }

    let mut amounts = counts.iter().map(|(a, b)| *b).collect::<Vec<Solution>>();
    amounts.sort();
    match &amounts[..] {
        [_] => FIVE_OF_A_KIND,
        [_, b] => {
            if *b == 4 {
                FOUR_OF_A_KIND
            } else {
                FULL_HOUSE
            }
        }
        [_, a, b] => {
            if a != b {
                THREE_OF_A_KIND
            } else {
                TWO_PAIR
            }
        }
        [_, _, _, _] => ONE_PAIR,
        [_, _, _, _, _] => HIGH_CARD,
        _ => panic!("No!!!"),
    }
}

fn remap_to_make_joker_weak(c: CardType) -> CardType {
    match c {
        CardType::A => CardType::A,
        CardType::K => CardType::K,
        CardType::Q => CardType::Q,
        CardType::T => CardType::J,
        CardType::Nine => CardType::T,
        CardType::Eight => CardType::Nine,
        CardType::Seven => CardType::Eight,
        CardType::Six => CardType::Seven,
        CardType::Five => CardType::Six,
        CardType::Four => CardType::Five,
        CardType::Three => CardType::Four,
        CardType::Two => CardType::Three,
        CardType::J => CardType::Two,
    }
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
        assert_eq!(part_1(&parse_output), 6440);
    }

    #[test]
    pub fn test_part_2() {
        let parse_output = &mut parse(TEST_INPUT);
        assert_eq!(part_2(parse_output), 5905);
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
            assert_eq!(part_1(black_box(&parse_output)), 251121738);
        });
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let parse_output = &mut parse(MAIN_INPUT);
        b.iter(|| {
            assert_eq!(part_2(black_box(parse_output)), 251421071);
        });
    }

    #[test]
    pub fn no_jokers() {
        use card_type::CardType::*;
        assert_eq!(
            FIVE_OF_A_KIND,
            hand_rank_joker(&[A, A, A, A, A]),
            "FIVE_OF_A_KIND"
        );
        assert_eq!(
            FOUR_OF_A_KIND,
            hand_rank_joker(&[A, A, A, A, T]),
            "FOUR_OF_A_KIND"
        );
        assert_eq!(FULL_HOUSE, hand_rank_joker(&[A, A, A, T, T]), "FULL_HOUSE");
        assert_eq!(
            THREE_OF_A_KIND,
            hand_rank_joker(&[A, A, A, T, Q]),
            "THREE_OF_A_KIND"
        );
        assert_eq!(TWO_PAIR, hand_rank_joker(&[A, A, Q, T, T]), "TWO_PAIR");
        assert_eq!(ONE_PAIR, hand_rank_joker(&[A, A, Q, K, T]), "ONE_PAIR");
        assert_eq!(HIGH_CARD, hand_rank_joker(&[A, Q, K, T, Two]), "HIGH_CARD");
    }
}
