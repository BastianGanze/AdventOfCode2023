#![feature(test)]

use std::collections::{BTreeSet, HashSet};

use fnv::{FnvHashMap, FnvHashSet};

type Solution = usize;

type BrickID = usize;
pub type BrickGrid = FnvHashMap<(usize, usize), HashSet<BrickID>>;
pub type Brick = ((usize, usize, usize), (usize, usize, usize));
pub type ParseOutput = Vec<Brick>;
const MAIN_INPUT: &str = include_str!("main_input");

pub fn parse(file: &str) -> ParseOutput {
    let mut bricks = Vec::new();
    for l in file.lines().filter(|l| !l.is_empty()) {
        let (start, end) = l.split_once("~").unwrap();
        if let [s_x, s_y, s_z] = start.split(",").collect::<Vec<&str>>()[..] {
            if let [e_x, e_y, e_z] = end.split(",").collect::<Vec<&str>>()[..] {
                bricks.push((
                    (
                        s_z.parse().unwrap(),
                        s_y.parse().unwrap(),
                        s_x.parse().unwrap(),
                    ),
                    (
                        e_z.parse().unwrap(),
                        e_y.parse().unwrap(),
                        e_x.parse().unwrap(),
                    ),
                ));
            }
        }
    }
    bricks.sort();
    bricks
}

fn part_1(bricks: &ParseOutput) -> Solution {
    let (deleted_bricks, _, _) = solve(bricks);
    deleted_bricks.len()
}

fn part_2(bricks: &mut ParseOutput) -> Solution {
    let (deleted_bricks, support, supported_by) = solve(bricks);
    let remaining_bricks = (0..bricks.len())
        .filter(|d| !deleted_bricks.contains(d))
        .collect::<FnvHashSet<usize>>();
    remaining_bricks
        .iter()
        .map(|b| {
            let mut falling = FnvHashSet::default();
            let mut bricks: BTreeSet<&usize> = BTreeSet::new();
            bricks.insert(b);
            while let Some(brick) = bricks.pop_first() {
                for brick_this_one_supports in support.get(brick).unwrap() {
                    let supported_by_no_others = supported_by
                        .get(brick_this_one_supports)
                        .unwrap()
                        .iter()
                        .filter(|os| **os != *brick && !falling.contains(*os))
                        .count();
                    if supported_by_no_others == 0 {
                        bricks.insert(brick_this_one_supports);
                        falling.insert(*brick_this_one_supports);
                    }
                }
            }
            falling.len()
        })
        .sum()
}
fn solve(
    bricks: &ParseOutput,
) -> (
    FnvHashSet<usize>,
    FnvHashMap<usize, FnvHashSet<usize>>,
    FnvHashMap<usize, FnvHashSet<usize>>,
) {
    let mut brick_grid: BrickGrid = FnvHashMap::default();
    let mut br: Vec<Brick> = bricks.clone();
    for (i, ((_, s_y, s_x), (_, e_y, e_x))) in bricks.iter().enumerate() {
        for y in *s_y..=*e_y {
            for x in *s_x..=*e_x {
                brick_grid.entry((y, x)).or_default().insert(i);
            }
        }
    }
    let mut supports_map: FnvHashMap<usize, FnvHashSet<usize>> = FnvHashMap::default();
    let mut supported_by_map: FnvHashMap<usize, FnvHashSet<usize>> = FnvHashMap::default();
    for i in 0..br.len() {
        supports_map.insert(i, FnvHashSet::default());
        supported_by_map.insert(i, FnvHashSet::default());
        let ((s_z, s_y, s_x), (_, e_y, e_x)) = br.get(i).unwrap();
        let mut new_brick_min = *s_z;
        'outer: while new_brick_min > 1 {
            for y in *s_y..=*e_y {
                for x in *s_x..=*e_x {
                    let bricks_in_z = brick_grid.get(&(y, x)).unwrap();
                    for brick_id in bricks_in_z {
                        if *brick_id != i {
                            let ((_, _, _), (other_brick_max_z, _, _)) =
                                br.get(*brick_id).unwrap() as &Brick;
                            if new_brick_min - 1 == *other_brick_max_z {
                                break 'outer;
                            }
                        }
                    }
                }
            }
            new_brick_min -= 1;
        }

        let brick = br.get_mut(i).unwrap();
        let d = brick.1 .0 - brick.0 .0;
        brick.0 .0 = new_brick_min;
        brick.1 .0 = new_brick_min + d;
    }

    for i in 0..br.len() {
        let ((brick_min_z, s_y, s_x), (_, e_y, e_x)) = br.get(i).unwrap();
        for y in *s_y..=*e_y {
            for x in *s_x..=*e_x {
                let bricks_in_z = brick_grid.get(&(y, x)).unwrap();
                for brick_id in bricks_in_z {
                    if *brick_id != i {
                        let ((_, _, _), (other_brick_max_z, _, _)) =
                            br.get(*brick_id).unwrap() as &Brick;
                        if brick_min_z - 1 == *other_brick_max_z {
                            supported_by_map.get_mut(&i).unwrap().insert(*brick_id);
                            supports_map.get_mut(brick_id).unwrap().insert(i);
                        }
                    }
                }
            }
        }
    }

    let mut deleted_brick_ids: FnvHashSet<usize> = FnvHashSet::default();
    for i in 0..br.len() {
        let bricks_this_one_is_supporting = supports_map.get(&i).unwrap();
        if bricks_this_one_is_supporting.is_empty() {
            deleted_brick_ids.insert(i);
        } else {
            let mut is_redundant = bricks_this_one_is_supporting.iter().all(|s| {
                let bricks_this_one_is_supporting_are_supported_by =
                    supported_by_map.get(s).unwrap();
                let supports_n = bricks_this_one_is_supporting_are_supported_by
                    .iter()
                    .filter(|os| **os != i)
                    .count();
                supports_n > 0
            });

            if is_redundant {
                deleted_brick_ids.insert(i);
            }
        }
    }
    (deleted_brick_ids, supports_map, supported_by_map)
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
    const TEST_INPUT: &str = include_str!("test_input");
    const TEST_INPUT_2: &str = include_str!("test_input_2");
    #[test]
    pub fn test_part_1() {
        let parse_output = parse(TEST_INPUT);
        assert_eq!(part_1(&parse_output), 5);
    }

    #[test]
    pub fn test_part_lypheo() {
        let parse_output = &mut parse(TEST_INPUT_2);
        assert_eq!(part_1(parse_output), 501);
    }

    #[test]
    pub fn test_part_lypheo_2() {
        let parse_output = &mut parse(TEST_INPUT_2);
        assert_eq!(part_2(parse_output), 80948);
    }

    #[test]
    pub fn test_part_2() {
        let parse_output = &mut parse(TEST_INPUT);
        assert_eq!(part_2(parse_output), 7);
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
            assert_eq!(part_1(black_box(&parse_output)), 517);
        });
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let parse_output = &mut parse(MAIN_INPUT);
        b.iter(|| {
            assert_eq!(part_2(black_box(parse_output)), 61276);
        });
    }
}
