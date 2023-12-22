#![feature(test)]

use std::cmp::{max, min, Ordering};
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
    let mut current_steps: BinaryHeap<FieldI> = BinaryHeap::new();
    let is_even_c = !(max_steps % 2 == 0) as usize;
    let max_plot_count_even = part_1(out, out.0.len() + is_even_c + 3);
    let max_plot_count_uneven = part_1(out, out.0.len() + is_even_c + 2);
    let remaining_steps = part_1(out, max_steps % out.0.len());
    let (grid, start) = out;
    let fields = (max_steps / grid.len()) * 2;
    let fields_occupied_in_square = (fields * fields) / 2;
    let guess = (fields_occupied_in_square / 2 * max_plot_count_uneven)
        + (fields_occupied_in_square / 2 * max_plot_count_even);
    println!(
        "grid_length {} fields {} guess {} remainder {}",
        grid.len(),
        fields,
        guess,
        remaining_steps
    );
    current_steps.push(FieldI(start.0, start.1, 0, 0, 0));
    let mut fields_with_steps: FnvHashMap<(usize, usize, i32, i32), usize> = FnvHashMap::default();
    let mut processed_paths: FnvHashSet<(usize, usize, i32, i32)> = FnvHashSet::default();
    let r = -4..=4;
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
    let mut right_even_tmpl = FnvHashMap::default();
    let mut right_even_tmpl2 = FnvHashMap::default();
    let mut right_odd_tmpl = FnvHashMap::default();
    for ((y, x, i_y, i_x), s) in &fields_with_steps {
        if *i_y == 0 && *i_x == 1 {
            right_odd_tmpl.insert((*y, *x, *i_y, *i_x), *s);
        }
        if *i_y == 0 && *i_x == 2 {
            right_even_tmpl.insert((*y, *x, *i_y, *i_x), *s);
        }
        if *i_y == 0 && *i_x == 4 {
            right_even_tmpl2.insert((*y, *x, *i_y, *i_x), *s);
        }
    }

    println!("{}", fields_with_steps.len());
    let f = (fields as i32 / 2);
    for i_y in -f..=f {
        for _ in 0..i_y.abs() {
            print!("   ")
        }
        for i_x in -f + i_y.abs() - 1..=f - i_y.abs() + 1 {
            print!(
                "{:0>2} ",
                fields_with_steps
                    .keys()
                    .filter(|(_, _, e, r)| *e == i_y && *r == i_x)
                    .count()
            );
        }
        println!();
    }

    for ((y, x, i_y, i_x), s) in &right_even_tmpl {
        println!("{}", right_even_tmpl2.get(&(*y, *x, 0, 4)).unwrap() - *s);
    }
    println!(
        "{} {}",
        right_even_tmpl.len(),
        right_even_tmpl
            .iter()
            .filter(|((y, x, i_y, i_x), s)| *s + (grid.len() * 2 * 5) < max_steps)
            .count()
    );
    println!(
        "{} {}",
        right_odd_tmpl.len(),
        right_odd_tmpl
            .iter()
            .filter(|((y, x, i_y, i_x), s)| *s + (grid.len() * 2 * 5) < max_steps)
            .count()
    );
    panic!();
    let f = (fields as i32 / 2) + 1;
    let mut even_diff_state: FnvHashMap<(usize, usize, i32, i32), usize> = FnvHashMap::default();
    let mut even_universe = None;
    let mut uneven_diff_state: FnvHashMap<(usize, usize, i32, i32), usize> = FnvHashMap::default();
    let mut uneven_universe = None;
    for i_y in -f..=f {
        for i_x in -f + i_y.abs() - 1..=f - i_y.abs() + 1 {
            let mut universe_min_s = 0;
            //   if ((-f..=-f + 2).contains(&i_x) || (f - 2..=f).contains(&i_x)) && i_y == 0 {
            println!("##");
            println!(
                "{:?}, {:?} - {} {}",
                i_y,
                i_x,
                fields_with_steps
                    .keys()
                    .filter(|(_, _, e, r)| *e == i_y && *r == i_x)
                    .count(),
                (i_y + i_x) % 2 == 0
            );
            println!("##");
            for y in 0..grid.len() {
                for x in 0..grid.len() {
                    if let (Some(s)) = fields_with_steps.get(&(y, x, i_y, i_x)) {
                        print!("{}", s);
                    } else {
                        print!(".")
                    }
                }
                println!();
            }
        }
    }
    'outer: for i_x in 0..=f + 1 {
        let mut universe_min_s = usize::MAX;
        for y in 0..grid.len() {
            for x in 0..grid.len() {
                if let Some(s) = fields_with_steps.get(&(y, x, 0, i_x)) {
                    universe_min_s = min(universe_min_s, *s);

                    if universe_min_s >= grid.len() * 2 {
                        if (i_x) % 2 == 0 {
                            even_universe = Some((0, i_x));
                        } else {
                            uneven_universe = Some((0, i_x));
                        }
                    }
                    if even_universe.is_some() && uneven_universe.is_some() {
                        break 'outer;
                    }
                }
            }
        }
    }
    let mut another_guess = 0;
    for i_y in -f..=f {
        for i_x in -f + i_y.abs()..=f - i_y.abs() {
            if (i_y + i_x) % 2 == 0 {
                another_guess += even_diff_state.len();
            } else {
                another_guess += uneven_diff_state.len();
            }
            println!(
                "{} {} {} {}",
                i_y,
                i_x,
                (i_y + i_x) % 2 == 0,
                (i_y.abs() + i_x.abs()) * (grid.len() as i32)
            );
        }
    }
    println!("yo {:?} {:?}", max_plot_count_even, max_plot_count_uneven);
    println!(
        "actual {} diff {} - {:.2}%",
        fields_with_steps.len(),
        fields_with_steps.len() as i64 - guess as i64,
        (fields_with_steps.len() as f64 - guess as f64).div(fields_with_steps.len() as f64) * 100.0
    );
    println!(
        "actual {} diff {} - {:.2}%",
        fields_with_steps.len(),
        fields_with_steps.len() as i64 - another_guess as i64,
        (fields_with_steps.len() as f64 - another_guess as f64).div(fields_with_steps.len() as f64)
            * 100.0
    );
    fields_with_steps.len()
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
        //assert_eq!(part_2(&parse_output, 1), 2);
        //assert_eq!(part_2(&parse_output, 2), 4);
        //assert_eq!(part_2(&parse_output, 3), 6);
        //assert_eq!(part_2(&parse_output, 5), 13);
        //assert_eq!(part_2(&parse_output, 6), 16);
        //assert_eq!(part_2(&parse_output, 10), 50);
        //assert_eq!(part_2(&parse_output, 50), 1594);
        assert_eq!(part_2(&parse_output, 110), 6536);
        //assert_eq!(part_2(&parse_output, 500), 167004);
        //assert_eq!(part_2(&parse_output, 1000), 668697);
        //assert_eq!(part_2(&parse_output, 1500), 1505076);
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
