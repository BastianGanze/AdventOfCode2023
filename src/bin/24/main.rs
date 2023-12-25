#![feature(test)]

use std::cmp::{max, min};
use std::f64;

type Solution = usize;
type Line2D = (f64, f64);
type VecFormat2D = ((f64, f64), (f64, f64));
type VecFormat3D = ((f64, f64, f64), (f64, f64, f64));
pub type ParseOutput = Vec<VecFormat3D>;
const MAIN_INPUT: &str = include_str!("main_input");
const TEST_INPUT: &str = include_str!("test_input");

fn part_1(lines: &ParseOutput, area_min: f64, area_max: f64) -> Solution {
    let range = area_min..=area_max;
    let mut points = 0;
    let lines2d: Vec<VecFormat2D> = lines
        .iter()
        .map(|((z, y, x), (v_z, v_y, v_x))| ((*y, *x), (*v_y, *v_x)))
        .collect();
    for l_i_1 in 0..lines2d.len() {
        for l_i_2 in l_i_1..lines2d.len() {
            let l1 = get_line_2d(&lines2d[l_i_1]);
            let l2 = get_line_2d(&lines2d[l_i_2]);
            if let Some(intersection) = compute_intersection_point(l1, l2) {
                if range.contains(&intersection.0)
                    && range.contains(&intersection.1)
                    && collision_happened_in_the_past(
                        intersection,
                        &lines2d[l_i_1],
                        &lines2d[l_i_2],
                    )
                {
                    points += 1;
                }
            }
        }
    }
    points
}

fn collision_happened_in_the_past(
    (i_y, _): (f64, f64),
    ((y_1, _), (v_y_1, _)): &VecFormat2D,
    ((y_2, _), (v_y_2, _)): &VecFormat2D,
) -> bool {
    let t_1 = (i_y - y_1) / v_y_1;
    let t_2 = (i_y - y_2) / v_y_2;
    return t_1 > 0.0 && t_2 > 0.0;
}

fn get_line_2d(((y, x), (v_y, v_x)): &VecFormat2D) -> Line2D {
    let m = v_y / v_x;
    (m, y - m * x)
}

fn compute_intersection_point((m1, c1): Line2D, (m2, c2): Line2D) -> Option<(f64, f64)> {
    let y = (c2 - c1) / (m1 - m2);
    let x = (m1 * y) + c1;

    if x.is_nan() || x.is_infinite() {
        None
    } else {
        Some((x, y))
    }
}

fn part_2(lines: &mut ParseOutput) -> Solution {
    let intersection_points = [(0.0, 0.0, 0.0), (0.0, 0.0, 0.0), (0.0, 0.0, 0.0)];
    let intersection_point_i = 0;
    let mut minr = (Solution::MAX, Solution::MAX, Solution::MAX);
    let mut maxr = (Solution::MIN, Solution::MIN, Solution::MIN);
    let mut avg = (0.0, 0.0, 0.0);
    for ((z, y, x), (v_z, v_y, v_x)) in lines.iter() {
        avg = (avg.0 + v_z, avg.1 + v_y, avg.2 + v_x);
        minr = (
            min(minr.0, *z as Solution),
            min(minr.1, *y as Solution),
            min(minr.2, *x as Solution),
        );
        maxr = (
            max(maxr.0, *z as Solution),
            max(maxr.1, *y as Solution),
            max(maxr.2, *x as Solution),
        );
    }
    avg = (
        avg.0 / lines.len() as f64,
        avg.1 / lines.len() as f64,
        avg.2 / lines.len() as f64,
    );
    println!("{:?} \n{:?} \navg{:?}", minr, maxr, avg);
    0
}

pub fn parse(file: &str) -> ParseOutput {
    file.lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let (poss, vels) = l.split_once(" @ ").unwrap();
            let pos = poss
                .split(", ")
                .map(|p| p.parse().unwrap())
                .collect::<Vec<f64>>();
            let vel = vels
                .split(", ")
                .map(|p| p.trim_start().parse().unwrap())
                .collect::<Vec<f64>>();
            ((pos[2], pos[1], pos[0]), (vel[2], vel[1], vel[0]))
        })
        .collect()
}
fn main() {
    let parse_output = &mut parse(MAIN_INPUT);
    println!(
        "Solution to part 1 is {}",
        part_1(parse_output, 200000000000000.0, 400000000000000.0)
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
        assert_eq!(part_1(&parse_output, 7.0, 27.0), 2);
    }

    #[test]
    pub fn test_part_2() {
        let parse_output = &mut parse(TEST_INPUT);
        assert_eq!(part_2(parse_output), 2);
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
            assert_eq!(
                part_1(
                    black_box(&parse_output),
                    200000000000000.0,
                    400000000000000.0
                ),
                16779
            );
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
