#![feature(test)]

use std::cmp::max;
use std::ops::Range;

type Solution = i32;
type Grid = Vec<Vec<(u8, bool, bool)>>;
type Vector2D<T> = (T, T);
type Beam = (Vector2D<usize>, Vector2D<Solution>);
pub type ParseOutput = Grid;
const MAIN_INPUT: &str = include_str!("main_input");
const TEST_INPUT: &str = include_str!("test_input");

fn part_1(
    grid: &ParseOutput,
    start_field: Vector2D<usize>,
    start_direction: Vector2D<Solution>,
) -> Solution {
    let g = &mut grid.clone();
    let max = (0..g.len() as Solution, 0..g[0].len() as Solution);
    let mut running_beams: Vec<Beam> = vec![(start_field, start_direction)];
    while let Some(((y, x), direction)) = running_beams.pop() {
        g[y][x].1 = true;
        let (next_dir, split) = get_next_directions(direction, &g[y][x].0);
        if split.is_some() {
            // Continue if we already split on this field
            if g[y][x].2 == true {
                continue;
            }
            g[y][x].2 = true;
        }
        split.and_then(|split_dir| {
            get_next_field(&(y, x), &split_dir, &max)
                .map(|next_field| running_beams.push((next_field, split_dir)))
        });
        get_next_field(&(y, x), &next_dir, &max)
            .map(|next_field| running_beams.push((next_field, next_dir)));
    }
    g.iter()
        .map(|l| l.iter().filter(|c| c.1).count() as Solution)
        .sum()
}

fn part_2(grid: &ParseOutput) -> Solution {
    let (y_max, x_max) = (grid.len() - 1, grid[0].len() - 1);
    let mut max_tiles = Solution::MIN;
    let right = (0, 1);
    let left = (0, -1);
    let top = (-1, 0);
    let bottom = (1, 0);
    for y in 0..=y_max {
        max_tiles = max(max_tiles, part_1(grid, (y, 0), right));
        max_tiles = max(max_tiles, part_1(grid, (y, x_max), left));
    }
    for x in 0..=x_max {
        max_tiles = max(max_tiles, part_1(grid, (0, x), bottom));
        max_tiles = max(max_tiles, part_1(grid, (y_max, x), top));
    }
    max_tiles
}

fn get_next_directions(
    old_dir: Vector2D<Solution>,
    c: &u8,
) -> (Vector2D<Solution>, Option<Vector2D<Solution>>) {
    match c {
        b'|' => match old_dir {
            (0, 1) | (0, -1) => ((-1, 0), Some((1, 0))),
            (1, 0) | (-1, 0) => (old_dir, None),
            _ => panic!(),
        },
        b'-' => match old_dir {
            (1, 0) | (-1, 0) => ((0, -1), Some((0, 1))),
            (0, 1) | (0, -1) => (old_dir, None),
            _ => panic!(),
        },
        b'/' => match old_dir {
            (1, 0) => ((0, -1), None),
            (-1, 0) => ((0, 1), None),
            (0, 1) => ((-1, 0), None),
            (0, -1) => ((1, 0), None),
            _ => panic!(),
        },
        b'\\' => match old_dir {
            (1, 0) => ((0, 1), None),
            (-1, 0) => ((0, -1), None),
            (0, 1) => ((1, 0), None),
            (0, -1) => ((-1, 0), None),
            _ => panic!(),
        },
        b'.' => (old_dir, None),
        _ => panic!(),
    }
}

fn get_next_field(
    (c_y, c_x): &Vector2D<usize>,
    (d_y, d_x): &Vector2D<Solution>,
    (y_bounds, x_bounds): &Vector2D<Range<Solution>>,
) -> Option<Vector2D<usize>> {
    let (new_y, new_x) = (*c_y as Solution + *d_y, *c_x as Solution + *d_x);
    if y_bounds.contains(&new_y) && x_bounds.contains(&new_x) {
        return Some((new_y as usize, new_x as usize));
    }

    None
}

pub fn parse(file: &str) -> ParseOutput {
    file.lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.as_bytes().iter().map(|c| (*c, false, false)).collect())
        .collect()
}
fn main() {
    let parse_output = &mut parse(MAIN_INPUT);
    println!(
        "Solution to part 1 is {}",
        part_1(parse_output, (0, 0), (0, 1))
    );
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
        assert_eq!(part_1(&parse_output, (0, 0), (0, 1)), 46);
    }

    #[test]
    pub fn test_part_2() {
        let parse_output = &mut parse(TEST_INPUT);
        assert_eq!(part_2(parse_output), 51);
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
            assert_eq!(part_1(black_box(&parse_output), (0, 0), (0, 1)), 7884);
        });
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let parse_output = &mut parse(MAIN_INPUT);
        b.iter(|| {
            assert_eq!(part_2(black_box(parse_output)), 8185);
        });
    }
}
