#![feature(test)]

type Solution = u64;
type EmptyY = usize;
type EmptyX = usize;
type Galaxy = (usize, usize);
pub type ParseOutput = (Vec<Galaxy>, Vec<EmptyX>, Vec<EmptyY>);
const MAIN_INPUT: &str = include_str!("main_input");
const TEST_INPUT: &str = include_str!("test_input");

pub fn parse(file: &str) -> ParseOutput {
    let (len_y, len_x) = (file.lines().count(), file.lines().next().unwrap().len());
    let mut galaxies = Vec::new();
    let mut empty_x = vec![0; len_x];
    let mut empty_y = vec![0; len_y];
    let lines: Vec<&[u8]> = file.lines().map(|l| l.as_bytes()).collect();

    let mut empty_space_y = 0;
    for y in 0..len_y {
        let mut has_galaxy_in_row = false;
        for x in 0..len_x {
            if lines[y][x] == b'#' {
                has_galaxy_in_row = true;
                galaxies.push((y, x));
            }
        }
        if !has_galaxy_in_row {
            empty_space_y += 1;
        }
        empty_y[y] = empty_space_y;
    }

    let mut empty_space_x = 0;
    for x in 0..len_x {
        let mut has_galaxy_in_column = false;
        for y in 0..len_y {
            if lines[y][x] == b'#' {
                has_galaxy_in_column = true;
            }
        }
        if !has_galaxy_in_column {
            empty_space_x += 1;
        }
        empty_x[x] = empty_space_x;
    }

    (galaxies, empty_y, empty_x)
}

pub fn manhattan_distance(p1: (usize, usize), p2: (usize, usize)) -> Solution {
    (p1.0.abs_diff(p2.0) + p1.1.abs_diff(p2.1)) as Solution
}

fn part_1((galaxies, empty_y, empty_x): &ParseOutput, dark_energy: usize) -> Solution {
    let g_n = galaxies.len();
    let mut sol = 0;
    for i1 in 0..g_n {
        for i2 in i1 + 1..g_n {
            let g1 = galaxies[i1];
            let g2 = galaxies[i2];
            sol += manhattan_distance(
                (
                    empty_y[g1.0] * dark_energy + g1.0,
                    empty_x[g1.1] * dark_energy + g1.1,
                ),
                (
                    empty_y[g2.0] * dark_energy + g2.0,
                    empty_x[g2.1] * dark_energy + g2.1,
                ),
            );
        }
    }
    sol
}
fn part_2(out: &mut ParseOutput) -> Solution {
    part_1(out, 1000000 - 1)
}
fn main() {
    let parse_output = &mut parse(MAIN_INPUT);
    println!("Solution to part 1 is {}", part_1(parse_output, 1));
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
        assert_eq!(part_1(&parse_output, 1), 374);
    }

    #[test]
    pub fn test_part_2() {
        let parse_output = &mut parse(TEST_INPUT);
        assert_eq!(part_1(parse_output, 9), 1030);
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
            assert_eq!(part_1(black_box(&parse_output), 1), 9609130);
        });
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let parse_output = &mut parse(MAIN_INPUT);
        b.iter(|| {
            assert_eq!(part_1(black_box(parse_output), 1000000 - 1), 702152204842);
        });
    }
}
