#![feature(test)]

mod check_range_overlap;

use crate::check_range_overlap::{check_range_overlap, RangeRelation};
use std::cmp::min;
use std::ops::Range;

// They write maps in the opposite order than I am so I glanced over that in the text and just assumed its my way
// Me running Day 04, copying the number and wondering why the solution is wrong
type Solution = i64;
type Seeds = Vec<Solution>;
type SeedRanges = Vec<Range<Solution>>;
type ConversionNumber = Solution;
type MapRange = Vec<(Range<Solution>, Range<Solution>, ConversionNumber)>;
pub type ParseOutput = (Seeds, SeedRanges, Vec<MapRange>);
const MAIN_INPUT: &str = include_str!("main_input");
const TEST_INPUT: &str = include_str!("test_input");

pub fn parse(file: &str) -> ParseOutput {
    let mut instructions = file.split("\n\n");
    let seeds: Vec<Solution> = instructions
        .next()
        .unwrap()
        .trim_start_matches("seeds: ")
        .split(" ")
        .into_iter()
        .map(|n| n.parse::<Solution>().unwrap())
        .collect();
    let seed_ranges = seeds
        .chunks(2)
        .filter_map(|slice| match slice {
            [a, b] => Some(*a..(*a + *b)),
            _ => None,
        })
        .collect();
    let map_ranges = instructions
        .map(|instruction| {
            instruction
                .split('\n')
                .skip(1)
                .filter(|i| !i.trim().is_empty())
                .map(|i| {
                    let nums = i
                        .split(" ")
                        .map(|n| n.parse::<Solution>().unwrap())
                        .collect::<Vec<Solution>>();
                    assert_eq!(nums.len(), 3);
                    (
                        nums[1]..nums[1] + nums[2],
                        nums[0]..nums[0] + nums[2],
                        nums[0] - nums[1],
                    )
                })
                .collect()
        })
        .collect();
    (seeds, seed_ranges, map_ranges)
}

fn part_1((seeds, _, transformations): &ParseOutput) -> Solution {
    let solution = seeds.iter().map(|seed| {
        let mut new_value = *seed;
        for ranges in transformations {
            for (origin_range, _, conversion_number) in ranges {
                if origin_range.contains(&new_value) {
                    new_value = new_value + conversion_number;
                    break;
                }
            }
        }
        new_value
    });
    solution.min().unwrap()
}

fn part_2((_, seed_ranges, transformations): &ParseOutput) -> Solution {
    let mut solution = Solution::MAX;
    for original_seed_range in seed_ranges {
        let mut unmapped_ranges: Vec<Range<Solution>> = vec![original_seed_range.clone()];
        let mut new_unmapped_ranges = Vec::new();
        let mut mapped_ranges = Vec::new();

        for transformation_ranges in transformations {
            for (origin_range, _, conversion_number) in transformation_ranges {
                for unmapped_range in unmapped_ranges.drain(..) {
                    match check_range_overlap(&unmapped_range, origin_range) {
                        RangeRelation::NoIntersect => {
                            new_unmapped_ranges.push(unmapped_range);
                        }
                        RangeRelation::Includes => {
                            mapped_ranges.push(
                                unmapped_range.start + conversion_number
                                    ..unmapped_range.end + conversion_number,
                            );
                        }
                        RangeRelation::Intersect(outside, inside) => {
                            new_unmapped_ranges.push(outside);
                            mapped_ranges.push(
                                inside.start + conversion_number..inside.end + conversion_number,
                            );
                        }
                        RangeRelation::IntersectTwice(outside_left, inside, outside_right) => {
                            new_unmapped_ranges.push(outside_left);
                            new_unmapped_ranges.push(outside_right);
                            mapped_ranges.push(
                                inside.start + conversion_number..inside.end + conversion_number,
                            );
                        }
                    }
                }
                unmapped_ranges.extend(new_unmapped_ranges.drain(..));
            }
            unmapped_ranges.extend(mapped_ranges.drain(..));
        }
        solution = min(
            solution,
            unmapped_ranges.iter().map(|r| r.start).min().unwrap(),
        );
    }
    solution
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
        assert_eq!(part_1(&parse_output), 35);
    }

    #[test]
    pub fn test_part_2() {
        let parse_output = &mut parse(TEST_INPUT);
        assert_eq!(part_2(parse_output), 46);
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
            assert_eq!(part_1(black_box(&parse_output)), 57075758);
        });
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let parse_output = &mut parse(MAIN_INPUT);
        b.iter(|| {
            assert_eq!(part_2(black_box(parse_output)), 31161857);
        });
    }
}
