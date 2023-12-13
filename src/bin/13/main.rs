#![feature(test)]
type Solution = usize;
type GridLines = Vec<Vec<bool>>;
type GridColumns = GridLines;
pub type ParseOutput = Vec<(GridLines, GridColumns)>;
const MAIN_INPUT: &str = include_str!("main_input");
const TEST_INPUT: &str = include_str!("test_input");

fn solve(grids: &ParseOutput, use_fixed: bool) -> Solution {
    grids.iter().fold(0, |acc, (rows, columns)| {
        acc + get_mirror_positions(columns)
            .iter()
            .filter_map(|(s, fixed)| if *fixed == use_fixed { Some(*s) } else { None })
            .sum::<usize>()
            + get_mirror_positions(rows)
                .iter()
                .filter_map(|(s, fixed)| {
                    if *fixed == use_fixed {
                        Some(s * 100)
                    } else {
                        None
                    }
                })
                .sum::<usize>()
    })
}
fn get_mirror_positions(the_data: &Vec<Vec<bool>>) -> Vec<(usize, bool)> {
    let mut palindromes = Vec::new();
    'outer: for seed in 1..the_data.len() {
        let i_range = (0..seed).rev();
        let i_range_rev = seed..the_data.len();
        let mut fixed = false;
        for (i, i_rev) in i_range.zip(i_range_rev) {
            match is_equal_with_possible_fix(&the_data[i], &the_data[i_rev]) {
                (false, Some(_)) => {
                    if fixed {
                        // Can fix one but can't fix two
                        continue 'outer;
                    }
                    fixed = true;
                }
                (false, None) => continue 'outer,
                _ => {}
            };
        }
        palindromes.push((seed, fixed));
    }

    palindromes
}

fn is_equal_with_possible_fix(a: &Vec<bool>, b: &Vec<bool>) -> (bool, Option<usize>) {
    let mut fix = None;
    let mut is_equal = true;
    for i in 0..a.len() {
        if a[i] != b[i] {
            if fix.is_none() {
                is_equal = false;
                fix = Some(i);
            } else {
                return (false, None);
            }
        }
    }

    (is_equal, fix)
}

pub fn parse(file: &str) -> ParseOutput {
    let mut grids = Vec::new();
    let grids_str = file.split("\n\n");
    for grid_str in grids_str {
        let lines: Vec<Vec<bool>> = grid_str
            .lines()
            .map(|l| l.chars().map(|c| c == '#').collect())
            .collect();
        let (len_y, len_x) = (
            grid_str.lines().count(),
            grid_str.lines().next().unwrap().len(),
        );

        let mut columns: Vec<Vec<bool>> = Vec::new();
        for x in 0..len_x {
            let mut c = Vec::new();
            for y in 0..len_y {
                c.push(lines[y][x])
            }
            columns.push(c);
        }
        grids.push((lines, columns));
    }
    grids
}
fn main() {
    let parse_output = &mut parse(MAIN_INPUT);
    println!("Solution to part 1 is {}", solve(parse_output, false));
    println!("Solution to part 2 is {}", solve(parse_output, true));
}

#[cfg(test)]
mod tests {
    extern crate test;
    use super::*;
    use test::{black_box, Bencher};

    #[test]
    pub fn test_part_1() {
        let parse_output = parse(TEST_INPUT);
        assert_eq!(solve(&parse_output, false), 405);
    }

    #[test]
    pub fn test_part_2() {
        let parse_output = &mut parse(TEST_INPUT);
        assert_eq!(solve(parse_output, true), 400);
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
            assert_eq!(solve(black_box(&parse_output), false), 41859);
        });
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let parse_output = &mut parse(MAIN_INPUT);
        b.iter(|| {
            assert_eq!(solve(black_box(parse_output), true), 30842);
        });
    }
}
