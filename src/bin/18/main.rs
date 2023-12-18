#![feature(test)]

use crate::Dir::*;
use std::cmp::{max, min};
use std::collections::{BTreeMap, HashMap};

type Solution = i64;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct GridCell {
    position: (Solution, Solution),
    color: usize,
}
type Grid = HashMap<(Solution, Solution), GridCell>;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}
type Instruction = (Dir, Solution, (Dir, Solution));
pub type ParseOutput = Vec<Instruction>;
const MAIN_INPUT: &str = include_str!("main_input");
const TEST_INPUT: &str = include_str!("test_input");
type IsEdge = bool;
fn part_1(instructions: &ParseOutput) -> Solution {
    let mut grid: BTreeMap<Solution, Vec<((Solution, Solution), IsEdge)>> = BTreeMap::new();
    let (mut current_y, mut current_x) = (0, 0);
    let mut wrapped_instructions = Vec::with_capacity(instructions.len() + 2);
    wrapped_instructions.push(instructions.last().unwrap());
    wrapped_instructions.extend(instructions.iter());
    wrapped_instructions.push(instructions.first().unwrap());
    let mut min_x = Solution::MAX;
    let mut max_x = Solution::MIN;
    for m in wrapped_instructions.windows(3) {
        match m {
            [(last_dir, _, _), (dir, n, _), (next_dir, _, _)] => {
                match dir {
                    Up => {
                        let new_y = current_y - n;
                        for y in new_y + 1..=current_y - 1 {
                            grid.entry(y)
                                .or_default()
                                .push(((current_x, current_x), false));
                        }
                        current_y = new_y
                    }
                    Down => {
                        let new_y = current_y + n;
                        for y in current_y + 1..=new_y - 1 {
                            grid.entry(y)
                                .or_default()
                                .push(((current_x, current_x), false));
                        }
                        current_y = new_y
                    }
                    Left => {
                        let new_x = current_x - n;
                        min_x = min(min_x, new_x);
                        max_x = max(max_x, new_x);
                        grid.entry(current_y)
                            .or_default()
                            .push(((new_x, current_x), last_dir != next_dir));
                        current_x = new_x;
                    }
                    Right => {
                        let new_x = current_x + n;
                        min_x = min(min_x, new_x);
                        max_x = max(max_x, new_x);
                        grid.entry(current_y)
                            .or_default()
                            .push(((current_x, new_x), last_dir != next_dir));
                        current_x = new_x;
                    }
                };
            }
            _ => panic!(),
        }
    }
    println!("{} {}", min_x, max_x);
    let mut inside_fields = 0;
    let mut outside_fields = 0;
    for (_, line) in grid.iter_mut() {
        line.sort();
        let ((x_22, x_33), is_e) = line.first().unwrap();
        let mut inside = !is_e || (x_22 == x_33);
        outside_fields += line.iter().map(|(s, _)| s.1 - s.0 + 1).sum::<Solution>();
        if line.is_empty() {
            continue;
        }
        for w in line.windows(2) {
            match w {
                [((_, x_1), _), ((x_2, x_3), is_edge_2)] => {
                    if inside {
                        inside_fields += x_2 - x_1 - 1;
                    }
                    if x_2 == x_3 || !is_edge_2 {
                        inside = !inside;
                    }
                }
                _ => panic!(),
            }
        }
    }
    outside_fields + inside_fields
}

fn part_2(instructions: &ParseOutput) -> Solution {
    let real_instructions: ParseOutput = instructions
        .iter()
        .map(|(_, _, (d, n))| (d.clone(), *n, (d.clone(), *n)))
        .collect();
    part_1(&real_instructions)
}

pub fn parse(file: &str) -> ParseOutput {
    file.lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let (d, i, c) = if l.len() == 13 {
                (&l[0..1], &l[2..3], &l[6..12])
            } else {
                (&l[0..1], &l[2..4], &l[7..13])
            };
            let dir = match d.chars().next().unwrap() {
                'R' => Right,
                'U' => Up,
                'L' => Left,
                'D' => Down,
                _ => panic!(),
            };
            let num = i.parse().unwrap();
            let actual_num = Solution::from_str_radix(&c[0..5], 16).unwrap();
            let actual_dir = match &c[5..6].parse().unwrap() {
                0 => Right,
                1 => Down,
                2 => Left,
                3 => Up,
                _ => panic!(),
            };
            (dir, num, (actual_dir, actual_num))
        })
        .collect::<ParseOutput>()
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
        assert_eq!(part_1(&parse_output), 62);
    }

    #[test]
    pub fn test_part_2() {
        let parse_output = &mut parse(TEST_INPUT);
        assert_eq!(part_2(parse_output), 952408144115);
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
            assert_eq!(part_1(black_box(&parse_output)), 39039);
        });
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let parse_output = &mut parse(MAIN_INPUT);
        b.iter(|| {
            assert_eq!(part_2(black_box(parse_output)), 44644464596918);
        });
    }
}
