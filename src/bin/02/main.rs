#![feature(test)]

use std::cmp::max;

type Solution = u32;
pub type ParseOutput = Vec<(u32, Vec<(u32, u32, u32)>)>;
const MAIN_INPUT: &str = include_str!("main_input");
const TEST_INPUT: &str = include_str!("test_input");

pub fn parse(file: &str) -> ParseOutput {
    file.lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let (game_id_str, games_str) = l.split_once(':').unwrap();
            let game_id: u32 = game_id_str.replace("Game ", "").parse().unwrap();
            let mut games = Vec::new();
            let game_split = games_str.split(';');

            for color_split in game_split {
                let mut game = (0, 0, 0);
                for color in color_split.replace(' ', "").split(',') {
                    if color.contains("red") {
                        game.0 = color.replace("red", "").parse().unwrap();
                    }
                    if color.contains("green") {
                        game.1 = color.replace("green", "").parse().unwrap();
                    }
                    if color.contains("blue") {
                        game.2 = color.replace("blue", "").parse().unwrap();
                    }
                }
                games.push(game);
            }
            (game_id, games)
        })
        .collect()
}

fn part_1(games: &ParseOutput) -> Solution {
    let max_r = 12;
    let max_g = 13;
    let max_b = 14;
    let mut solution = 0;
    for (game_id, balls) in games {
        if balls
            .iter()
            .all(|game| game.0 <= max_r && game.1 <= max_g && game.2 <= max_b)
        {
            solution += game_id;
        }
    }
    solution
}

fn part_2(games: &mut ParseOutput) -> Solution {
    let mut solution = 0;
    for (_, balls) in games {
        let min_balls = balls.iter().fold((0, 0, 0), |acc, (r, g, b)| {
            (max(acc.0, *r), max(acc.1, *g), max(acc.2, *b))
        });
        solution += min_balls.0 * min_balls.1 * min_balls.2
    }
    solution
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
        assert_eq!(part_1(&parse_output), 8);
    }

    #[test]
    pub fn test_part_2() {
        let parse_output = &mut parse(TEST_INPUT);
        assert_eq!(part_2(parse_output), 2286);
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
            assert_eq!(part_1(black_box(&parse_output)), 2268);
        });
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let parse_output = &mut parse(MAIN_INPUT);
        b.iter(|| {
            assert_eq!(part_2(black_box(parse_output)), 63542);
        });
    }
}
