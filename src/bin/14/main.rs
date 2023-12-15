#![feature(test)]

use std::collections::HashMap;
use std::ops::Range;

type Solution = usize;
pub type ParseOutput = Vec<Vec<u8>>;
const MAIN_INPUT: &str = include_str!("main_input");
const TEST_INPUT: &str = include_str!("test_input");

fn part_1(grid: &ParseOutput) -> Solution {
    let (len_y, len_x) = (grid.len(), grid[0].len());
    let mut sol = 0;
    for x in 0..len_x {
        let mut current_empty_spaces = 0;
        for y in 0..len_y {
            let mut current_height = len_y - y;
            match grid[y][x] {
                b'.' => current_empty_spaces += 1,
                b'O' => sol += current_height + current_empty_spaces,
                b'#' => current_empty_spaces = 0,
                _ => panic!(),
            }
        }
    }
    sol
}

fn part_2(grid: &mut ParseOutput) -> Solution {
    let mut g = grid.clone();
    let len = grid.len();
    let mut grids: HashMap<Vec<Vec<u8>>, usize> = HashMap::new();
    let cycles = 1000000000;
    println!("start {} <-", part_1(&g));
    print_grid(&g);
    for i in 1..=cycles {
        println!("Cycle {}", i);
        roll_rocks_north(&mut g, 0..len, 0..len);
        roll_rocks_west(&mut g, 0..len, 0..len);
        roll_rocks_south(&mut g, 0..len, 0..len);
        roll_rocks_east(&mut g, 0..len, 0..len);
        if let Some(cycle_start_i) = grids.get(&g) {
            let cycle_length = (i - cycle_start_i);
            println!("amount of cycles till the end {}", cycles - i);
            println!("cycle length {}", cycle_length);
            println!(
                "amount of inner cycles till the end {}",
                (cycles - i) / cycle_length
            );
            println!(
                "amount of inner cycles till the end {}",
                (cycles - i) % cycle_length
            );
            let index = cycle_start_i + ((cycles - i) % cycle_length);
            println!("{} {}", cycle_start_i, cycle_length);
            print_grid(&g);
            g = grids
                .into_iter()
                .find_map(|(a, b)| if b == index { Some(a) } else { None })
                .unwrap();
            print_grid(&g);
            println!("{}", calc_north_support(&g, len));

            break;
        } else {
            grids.insert(g.clone(), i);
        }
    }

    calc_north_support(&g, len)
}

pub fn calc_north_support(grid: &ParseOutput, len: usize) -> Solution {
    let mut sol = 0;
    for x in 0..len {
        for y in 0..len {
            if grid[y][x] == b'O' {
                sol += len - y;
            }
        }
    }
    sol
}

pub fn roll_rocks_north(
    grid: &mut ParseOutput,
    range_1: Range<Solution>,
    range_2: Range<Solution>,
) {
    for r1 in range_1 {
        let mut current_empty_spaces = 0;
        for r2 in range_2.start..range_2.end {
            match grid[r2][r1] {
                b'.' => current_empty_spaces += 1,
                b'O' => {
                    grid[r2][r1] = b'.';
                    grid[r2 - current_empty_spaces][r1] = b'O';
                }
                b'#' => current_empty_spaces = 0,
                _ => panic!(),
            }
        }
    }
}

pub fn roll_rocks_west(grid: &mut ParseOutput, range_1: Range<Solution>, range_2: Range<Solution>) {
    for r1 in range_1 {
        let mut current_empty_spaces = 0;
        for r2 in range_2.start..range_2.end {
            match grid[r1][r2] {
                b'.' => current_empty_spaces += 1,
                b'O' => {
                    grid[r1][r2] = b'.';
                    grid[r1][r2 - current_empty_spaces] = b'O';
                }
                b'#' => current_empty_spaces = 0,
                _ => panic!(),
            }
        }
    }
}

pub fn roll_rocks_south(
    grid: &mut ParseOutput,
    range_1: Range<Solution>,
    range_2: Range<Solution>,
) {
    for r1 in range_1 {
        let mut current_empty_spaces = 0;
        for r2 in (range_2.start..range_2.end).rev() {
            match grid[r2][r1] {
                b'.' => current_empty_spaces += 1,
                b'O' => {
                    grid[r2][r1] = b'.';
                    grid[r2 + current_empty_spaces][r1] = b'O';
                }
                b'#' => current_empty_spaces = 0,
                _ => panic!(),
            }
        }
    }
}

pub fn roll_rocks_east(grid: &mut ParseOutput, range_1: Range<Solution>, range_2: Range<Solution>) {
    for r1 in range_1 {
        let mut current_empty_spaces = 0;
        for r2 in (range_2.start..range_2.end).rev() {
            match grid[r1][r2] {
                b'.' => current_empty_spaces += 1,
                b'O' => {
                    grid[r1][r2] = b'.';
                    grid[r1][r2 + current_empty_spaces] = b'O';
                }
                b'#' => current_empty_spaces = 0,
                _ => panic!(),
            }
        }
    }
}

pub fn print_grid(grid: &ParseOutput) {
    println!(
        "{}",
        grid.iter()
            .map(|l| format!("{}\n", l.iter().map(|c| *c as char).collect::<String>()))
            .collect::<String>()
    );
}

pub fn parse(file: &str) -> ParseOutput {
    let mut grid = Vec::new();

    for l in file.lines().filter(|l| !l.is_empty()) {
        let mut line = Vec::new();
        for character in l.as_bytes().iter() {
            line.push(*character);
        }
        grid.push(line);
    }

    grid
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
        assert_eq!(part_1(&parse_output), 136);
    }

    #[test]
    pub fn test_part_2() {
        let parse_output = &mut parse(TEST_INPUT);
        assert_eq!(part_2(parse_output), 64);
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
            assert_eq!(part_1(black_box(&parse_output)), 105784);
        });
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let parse_output = &mut parse(MAIN_INPUT);
        b.iter(|| {
            assert_eq!(part_2(black_box(parse_output)), 1104);
        });
    }
}
