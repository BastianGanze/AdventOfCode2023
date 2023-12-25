#![feature(test)]

use std::f64;

use nalgebra::Vector3;

type Solution = f64;
type Line2D = (Solution, Solution);
type VecFormat2D = ((Solution, Solution), (Solution, Solution));
type VecFormat3D = (Vector3<Solution>, Vector3<Solution>);
pub type ParseOutput = Vec<VecFormat3D>;
const MAIN_INPUT: &str = include_str!("main_input");
const TEST_INPUT: &str = include_str!("test_input");

fn part_1(lines: &ParseOutput, area_min: Solution, area_max: Solution) -> Solution {
    let range = area_min..=area_max;
    let mut points = 0;
    let lines2d: Vec<VecFormat2D> = lines
        .iter()
        .map(|(p, v)| ((p.y, p.x), (v.y, v.x)))
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
    points as Solution
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

fn part_2(projectiles: &mut ParseOutput) -> Solution {
    let three_projectiles: ParseOutput = projectiles[0..3].into();
    let search_vel = 300;
    let mut intersection_point = (0.0, 0.0, 0.0);
    'outer: for vz in -search_vel..search_vel {
        for vy in -search_vel..search_vel {
            for vx in -search_vel..search_vel {
                let mut try_projectiles = three_projectiles.clone();
                for (_, v) in try_projectiles.iter_mut() {
                    v.z += vz as f64;
                    v.y += vy as f64;
                    v.x += vx as f64;
                }
                if let (Some(i), Some(i2), Some(i3)) = (
                    intersect_3d(&try_projectiles[0], &try_projectiles[1]),
                    intersect_3d(&try_projectiles[1], &try_projectiles[2]),
                    intersect_3d(&try_projectiles[0], &try_projectiles[2]),
                ) {
                    if i == i2 && i == i3 {
                        println!("{:?}", i);
                        intersection_point = i;
                        break 'outer;
                    }
                }
            }
        }
    }
    intersection_point.0 + intersection_point.1 + intersection_point.2
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
            (
                Vector3::new(pos[0], pos[1], pos[2]),
                Vector3::new(vel[0], vel[1], vel[2]),
            )
        })
        .collect()
}

fn intersect_3d(
    (p1, v1): &VecFormat3D,
    (p2, v2): &VecFormat3D,
) -> Option<(Solution, Solution, Solution)> {
    // cred https://math.stackexchange.com/questions/270767/find-intersection-of-two-3d-lines
    let g = p2 - p1;
    let h = v2.cross(&g).magnitude();
    let k = v2.cross(&v1).magnitude();
    if h == 0.0 || k == 0.0 {
        return None;
    }
    Some((
        (p1.z + (v1.z * h / k).round()) as Solution,
        (p1.y + (v1.y * h / k).round()) as Solution,
        (p1.x + (v1.x * h / k).round()) as Solution,
    ))
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

    use test::{black_box, Bencher};

    use super::*;

    #[test]
    pub fn test_part_1() {
        let parse_output = parse(TEST_INPUT);
        assert_eq!(part_1(&parse_output, 7.0, 27.0), 2.0);
    }

    #[test]
    pub fn test_part_2() {
        let parse_output = &mut parse(TEST_INPUT);
        assert_eq!(part_2(parse_output), 47.0);
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
                16779.0
            );
        });
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let parse_output = &mut parse(MAIN_INPUT);
        b.iter(|| {
            assert_eq!(part_2(black_box(parse_output)), 1104.0);
        });
    }
}
