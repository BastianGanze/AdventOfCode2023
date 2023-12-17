#![feature(test)]
#![feature(let_chains)]

use std::collections::{BinaryHeap, HashMap};

use crate::grid::{Field, Grid};

type Solution = u32;
pub type ParseOutput = Grid;
const MAIN_INPUT: &str = include_str!("main_input");
const TEST_INPUT: &str = include_str!("test_input");

#[derive(Debug, Hash, Eq, PartialEq)]
struct Cell {
    position: (usize, usize),
    straights: i32,
    direction: (i32, i32),
}
mod grid;
fn part_1(grid: &ParseOutput) -> Solution {
    calc_best_path(grid.clone(), 0, 3)
}

fn part_2(grid: &mut ParseOutput) -> Solution {
    calc_best_path(grid.clone(), 3, 10)
}

pub fn parse(file: &str) -> ParseOutput {
    let mut grid = Grid::new((file.lines().count(), file.lines().next().unwrap().len()));

    for (y, l) in file.lines().filter(|l| !l.is_empty()).enumerate() {
        for (x, c) in l.bytes().enumerate() {
            grid.set_field_height(y, x, (c - 48) as Solution);
        }
    }

    grid
}
fn main() {
    let parse_output = &mut parse(MAIN_INPUT);
    println!("Solution to part 1 is {}", part_1(parse_output));
    println!("Solution to part 2 is {}", part_2(parse_output));
}

fn calc_best_path(mut grid: ParseOutput, min_straights: i32, max_straights: i32) -> Solution {
    let mut memoization: HashMap<Cell, (u32, u32)> = HashMap::new();
    let mut open_fields = BinaryHeap::<Field>::new();
    let start = Field::new((0, 0), 0, 0, (0, 0), 0);
    let end_coords = (grid.fields.len() - 1, grid.fields[0].len() - 1);
    open_fields.push(start);
    while let Some(mut current_field) = open_fields.pop() {
        let neighbours = grid.get_unmarked_neighbours(
            current_field.coordinate.0,
            current_field.coordinate.1,
            current_field.direction,
            current_field.straights,
            min_straights,
        );
        for (y, x, heat_loss, new_direction, new_straights) in neighbours {
            if new_straights >= max_straights {
                continue;
            }
            if (y, x) == end_coords {
                if new_straights >= min_straights {
                    return current_field.total_heat_loss + heat_loss;
                }
            }
            let field_cost = grid.get_field_cost(y, x);
            let new_total_heat_loss = current_field.total_heat_loss + heat_loss;
            let new_total_field_cost = current_field.cost + field_cost;
            let new_cell = Cell {
                straights: new_straights,
                direction: new_direction,
                position: (y, x),
            };
            if let Some(&best_for_field) = memoization.get(&new_cell) {
                if new_total_heat_loss >= best_for_field.0 {
                    continue;
                }
            }
            memoization.insert(new_cell, (new_total_heat_loss, new_total_field_cost));

            open_fields.push(Field::new(
                (y, x),
                new_total_field_cost,
                new_total_heat_loss,
                new_direction,
                new_straights,
            ));
        }
    }

    0
}

#[cfg(test)]
mod tests {
    extern crate test;

    use test::{black_box, Bencher};

    use super::*;

    #[test]
    pub fn test_part_1() {
        let parse_output = parse(TEST_INPUT);
        assert_eq!(part_1(&parse_output), 102);
    }

    #[test]
    pub fn test_part_2() {
        let parse_output = &mut parse(TEST_INPUT);
        assert_eq!(part_2(parse_output), 94);
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
            assert_eq!(part_1(black_box(&parse_output)), 1195);
        });
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let parse_output = &mut parse(MAIN_INPUT);
        b.iter(|| {
            assert_eq!(part_2(black_box(parse_output)), 1347);
        });
    }
}
