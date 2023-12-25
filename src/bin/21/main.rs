#![feature(test)]

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::ops::Div;

use fnv::{FnvHashMap, FnvHashSet};
type Solution = usize;
type Grid = Vec<Vec<u8>>;
pub type ParseOutput = (Grid, (usize, usize));

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Field(usize, usize, Solution);
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct FieldI(usize, usize, i32, i32, Solution);
const MAIN_INPUT: &str = include_str!("main_input");

pub fn parse(file: &'static str) -> ParseOutput {
    let mut start = (0, 0);
    let grid: Grid = file
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.as_bytes().iter().map(|c| *c).collect())
        .collect();
    for (y, l) in grid.iter().enumerate() {
        for (x, c) in l.iter().enumerate() {
            if *c == b'S' {
                start = (y, x);
            }
        }
    }
    (grid, start)
}

fn part_1((grid, start): &ParseOutput, max_steps: usize) -> Solution {
    let mut current_steps: BinaryHeap<Field> = BinaryHeap::new();
    current_steps.push(Field(start.0, start.1, 0));
    let is_even = !(max_steps % 2 == 0) as usize;
    let mut fields_with_steps: FnvHashSet<(usize, usize)> = FnvHashSet::default();
    let mut processed_paths: FnvHashSet<(usize, usize)> = FnvHashSet::default();
    while let Some(Field(y, x, path_length)) = current_steps.pop() {
        if !processed_paths.insert((y, x)) || path_length > max_steps {
            continue;
        }
        if path_length % 2 == is_even {
            fields_with_steps.insert((y, x));
        }
        for (n_y, n_x) in get_neighbours(grid, (y, x)) {
            current_steps.push(Field(n_y, n_x, path_length + 1));
        }
    }
    fields_with_steps.len()
}

fn part_2(out: &ParseOutput, max_steps: usize) -> Solution {
    let half_grid = out.0.len() / 2;
    let full_grid = out.0.len();
    let data_point_1 = (solve_i(out, half_grid), half_grid as f64);
    let data_point_2 = (
        solve_i(out, half_grid + full_grid * 2),
        (half_grid + full_grid * 2) as f64,
    );
    let data_point_3 = (
        solve_i(out, half_grid + full_grid * 4),
        (half_grid + full_grid * 4) as f64,
    );
    lagrange_interpolate(
        &[data_point_1, data_point_2, data_point_3],
        max_steps as f64,
    ) as Solution
}

fn lagrange_interpolate(points: &[(f64, f64)], x: f64) -> f64 {
    let mut result = 0.0;

    for i in 0..points.len() {
        let mut li = 1.0;
        for j in 0..points.len() {
            if i != j {
                li *= (x - points[j].1) / (points[i].1 - points[j].1);
            }
        }
        result += points[i].0 * li;
    }

    result
}

fn solve_i((grid, start): &ParseOutput, max_steps: usize) -> f64 {
    let is_even_c = !(max_steps % 2 == 0) as usize;
    let mut current_steps: BinaryHeap<FieldI> = BinaryHeap::new();
    current_steps.push(FieldI(start.0, start.1, 0, 0, 0));
    let mut fields_with_steps: FnvHashMap<(usize, usize, i32, i32), usize> = FnvHashMap::default();
    let mut processed_paths: FnvHashSet<(usize, usize, i32, i32)> = FnvHashSet::default();
    while let Some(FieldI(y, x, i_y, i_x, path_length)) = current_steps.pop() {
        if !processed_paths.insert((y, x, i_y, i_x)) || path_length > max_steps {
            continue;
        }
        if path_length % 2 == is_even_c {
            fields_with_steps.insert((y, x, i_y, i_x), path_length);
        }
        for (n_y, n_x, n_i_y, n_i_x) in get_neighbours_i(grid, (y, x, i_y, i_x)) {
            current_steps.push(FieldI(n_y, n_x, n_i_y, n_i_x, path_length + 1));
        }
    }
    fields_with_steps.len() as f64
}

fn get_neighbours_i(
    grid: &Grid,
    (y, x, i_y, i_x): (usize, usize, i32, i32),
) -> Vec<(usize, usize, i32, i32)> {
    let mut n = Vec::new();
    match x.checked_sub(1) {
        Some(n_x) => {
            if grid[y][n_x] != b'#' {
                n.push((y, n_x, i_y, i_x))
            }
        }
        None => {
            if grid[y][grid.len() - 1] != b'#' {
                n.push((y, grid.len() - 1, i_y, i_x - 1))
            }
        }
    }
    match x + 1 < grid.len() {
        true => {
            if grid[y][x + 1] != b'#' {
                n.push((y, x + 1, i_y, i_x))
            }
        }
        false => {
            if grid[y][0] != b'#' {
                n.push((y, 0, i_y, i_x + 1))
            }
        }
    }
    match y.checked_sub(1) {
        Some(n_y) => {
            if grid[n_y][x] != b'#' {
                n.push((n_y, x, i_y, i_x))
            }
        }
        None => {
            if grid[grid.len() - 1][x] != b'#' {
                n.push((grid.len() - 1, x, i_y - 1, i_x))
            }
        }
    }
    match y + 1 < grid.len() {
        true => {
            if grid[y + 1][x] != b'#' {
                n.push((y + 1, x, i_y, i_x))
            }
        }
        false => {
            if grid[0][x] != b'#' {
                n.push((0, x, i_y + 1, i_x))
            }
        }
    }

    n
}

fn get_neighbours(grid: &Grid, (y, x): (usize, usize)) -> Vec<(usize, usize)> {
    let mut n = Vec::new();
    if let Some(n_x) = x.checked_sub(1) {
        if grid[y][n_x] != b'#' {
            n.push((y, n_x))
        }
    }
    if x + 1 < grid.len() {
        if grid[y][x + 1] != b'#' {
            n.push((y, x + 1))
        }
    }

    if let Some(n_y) = y.checked_sub(1) {
        if grid[n_y][x] != b'#' {
            n.push((n_y, x))
        }
    }
    if y + 1 < grid.len() {
        if grid[y + 1][x] != b'#' {
            n.push((y + 1, x))
        }
    }

    n
}
fn main() {
    let parse_output = &mut parse(MAIN_INPUT);
    println!("Solution to part 1 is {}", part_1(parse_output, 64));
    println!("Solution to part 2 is {}", part_2(parse_output, 26501365));
}

#[cfg(test)]
mod tests {
    extern crate test;

    use test::{black_box, Bencher};

    use super::*;

    const TEST_INPUT: &str = include_str!("test_input");
    #[test]
    pub fn test_part_1() {
        let parse_output = parse(TEST_INPUT);
        assert_eq!(part_1(&parse_output, 1), 2);
        assert_eq!(part_1(&parse_output, 2), 4);
        assert_eq!(part_1(&parse_output, 3), 6);
        assert_eq!(part_1(&parse_output, 5), 13);
        assert_eq!(part_1(&parse_output, 6), 16);
    }

    #[test]
    pub fn test_part_2() {
        let parse_output = parse(TEST_INPUT);
        assert_eq!(solve_i(&parse_output, 10), 50.0);
        assert_eq!(solve_i(&parse_output, 50), 1594.0);
        assert_eq!(part_2(&parse_output, 10), 50);
        assert_eq!(part_2(&parse_output, 50), 1594);
        assert_eq!(part_2(&parse_output, 100), 6536);
        assert_eq!(part_2(&parse_output, 500), 167004);
        assert_eq!(part_2(&parse_output, 1000), 668697);
        assert_eq!(part_2(&parse_output, 5000), 16733044);
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
            assert_eq!(part_1(black_box(&parse_output), 64), 1798691765);
        });
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let parse_output = &mut parse(MAIN_INPUT);
        b.iter(|| {
            assert_eq!(part_2(black_box(parse_output), 26501365), 1104);
        });
    }
}

impl PartialOrd for FieldI {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.4 > other.4 {
            return Some(Ordering::Less);
        }

        if self.4 == other.4 {
            return Some(Ordering::Equal);
        }

        return Some(Ordering::Greater);
    }
}

impl Ord for FieldI {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.4 > other.4 {
            return Ordering::Less;
        }

        if self.4 == other.4 {
            return Ordering::Equal;
        }

        return Ordering::Greater;
    }
}

impl PartialOrd for Field {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.2 > other.2 {
            return Some(Ordering::Less);
        }

        if self.2 == other.2 {
            return Some(Ordering::Equal);
        }

        return Some(Ordering::Greater);
    }
}

impl Ord for Field {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.2 > other.2 {
            return Ordering::Less;
        }

        if self.2 == other.2 {
            return Ordering::Equal;
        }

        return Ordering::Greater;
    }
}
