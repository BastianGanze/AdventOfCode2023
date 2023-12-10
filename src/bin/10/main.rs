#![feature(test)]

use crate::fastest_path::{count_unmarked_fields, get_loop, turn_direction, Turn};
use crate::grid::Dirs::{East, North, South, West};
use crate::grid::FieldType::Pipe;
use crate::grid::{Dirs, Field, FieldType, Grid};
use std::cmp::min;
use std::fmt::Debug;
use std::usize;

pub mod fastest_path;
pub mod grid;
type Solution = i32;
type ParseOutput = (Grid, (usize, usize));
const MAIN_INPUT: &str = include_str!("main_input");
const TEST_INPUT: &str = include_str!("test_input");
const TEST_INPUT_2: &str = include_str!("test_input_2");

pub fn parse(file: &str) -> ParseOutput {
    use Dirs::*;
    use FieldType::*;
    let mut grid = Grid::new((file.lines().count(), file.lines().next().unwrap().len()));
    let mut start = (0, 0);

    for (y, l) in file.lines().filter(|l| !l.is_empty()).enumerate() {
        for (x, character) in l.chars().enumerate() {
            grid.set_field_type(
                y,
                x,
                match character {
                    'S' => {
                        start = (y, x);
                        Start
                    }
                    '|' => Pipe(North, South),
                    '-' => Pipe(East, West),
                    'L' => Pipe(North, East),
                    'J' => Pipe(North, West),
                    '7' => Pipe(South, West),
                    'F' => Pipe(South, East),
                    '.' => None,
                    c => panic!("Could not find '{}'", c),
                },
            );
        }
    }

    (grid, start)
}

fn part_1(out: &ParseOutput) -> Solution {
    let o = &mut out.clone();
    get_loop(o).len() as Solution / 2
}

fn part_2(out: &ParseOutput) -> Solution {
    use Dirs::*;
    let o = &mut out.clone();
    let l = get_loop(o);
    let grid = &mut o.0;
    let mut unmarked_fields = 0;
    let (mut current_normal_dir, start_i) = get_normal(&l);
    let mut from = (
        l[(start_i + 1) % l.len()].coordinate.0 as Solution - l[start_i].coordinate.0 as Solution,
        l[(start_i + 1) % l.len()].coordinate.1 as Solution - l[start_i].coordinate.1 as Solution,
    );
    for i in start_i + 1..l.len() + start_i + 1 {
        let current = l[i % l.len()];
        let next = l[(i + 1) % l.len()];
        let to = (
            next.coordinate.0 as Solution - current.coordinate.0 as Solution,
            next.coordinate.1 as Solution - current.coordinate.1 as Solution,
        );
        if let Some(seed) = get_field_seed(current_normal_dir, current) {
            unmarked_fields += count_unmarked_fields(grid, seed);
        }
        current_normal_dir = match turn_direction(from, to) {
            Turn::Left => match current_normal_dir {
                North => West,
                West => South,
                South => East,
                East => North,
            },
            Turn::Right => match current_normal_dir {
                North => East,
                East => South,
                South => West,
                West => North,
            },
            Turn::Straight => current_normal_dir,
        };
        if let Some(seed) = get_field_seed(current_normal_dir, current) {
            unmarked_fields += count_unmarked_fields(grid, seed);
        }
        from = to;
    }
    unmarked_fields
}

fn get_field_seed(mut current_normal_dir: Dirs, current: Field) -> Option<(usize, usize)> {
    match current_normal_dir {
        North => {
            if current.coordinate.0 == 0 {
                None
            } else {
                Some((current.coordinate.0 - 1, current.coordinate.1))
            }
        }
        East => Some((current.coordinate.0, current.coordinate.1 + 1)),
        South => Some((current.coordinate.0 + 1, current.coordinate.1)),
        West => {
            if current.coordinate.1 == 0 {
                None
            } else {
                Some((current.coordinate.0, current.coordinate.1 - 1))
            }
        }
    }
}

fn get_normal(l: &Vec<Field>) -> (Dirs, usize) {
    let min_field_pos = l
        .iter()
        .fold((usize::MAX, usize::MAX), |acc, l| min(acc, l.coordinate));
    let start_pos = l
        .iter()
        .position(|f| f.coordinate == min_field_pos)
        .unwrap();

    let cur_field = l[start_pos];
    let next_field = l[(start_pos + 1) % l.len()];
    match (
        next_field.coordinate.0 as Solution - cur_field.coordinate.0 as Solution,
        next_field.coordinate.1 as Solution - cur_field.coordinate.1 as Solution,
    ) {
        (0, 1) => (South, start_pos),
        (1, 0) => (East, start_pos),
        d => panic!("{:?}", d),
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

    use test::{black_box, Bencher};

    use super::*;

    #[test]
    pub fn test_part_1() {
        let parse_output = parse(TEST_INPUT);
        assert_eq!(part_1(&parse_output), 8);
    }

    #[test]
    pub fn test_part_2() {
        let parse_output = parse(TEST_INPUT_2);
        assert_eq!(part_2(&parse_output), 10);
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
            assert_eq!(part_1(black_box(&parse_output)), 6882);
        });
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let parse_output = parse(MAIN_INPUT);
        b.iter(|| {
            assert_eq!(part_2(black_box(&parse_output)), 491);
        });
    }
}
