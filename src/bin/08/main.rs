#![feature(test)]

use std::collections::HashMap;

type Solution = u64;

type Map<'a> = HashMap<&'a str, (&'a str, &'a str)>;
pub type ParseOutput<'a> = (Vec<char>, Map<'a>);
const MAIN_INPUT: &str = include_str!("main_input");
const TEST_INPUT: &str = include_str!("test_input");
const TEST_INPUT_2: &str = include_str!("test_input_2");

pub fn parse(file: &str) -> ParseOutput {
    let (i_str, m_str) = file.split_once("\n\n").unwrap();
    (
        i_str.chars().collect(),
        m_str
            .split("\n")
            .filter(|s| !s.is_empty())
            .map(|map_entry| {
                let (key, instructions) = map_entry.split_once("=").unwrap();
                (&key[0..=2], (&instructions[2..=4], &instructions[7..=9]))
            })
            .collect::<Map>(),
    )
}

fn part_1((instructions, map): &ParseOutput) -> Solution {
    let mut current_key = "AAA";
    let mut count = 0;
    loop {
        for i in instructions {
            current_key = match i {
                'L' => map.get(&current_key).unwrap().0.clone(),
                'R' => map.get(&current_key).unwrap().1.clone(),
                _ => panic!(),
            };
            count += 1;
            if (current_key == "ZZZ") {
                return count;
            }
        }
    }
}

type End<'a> = Option<&'a str>;
type LoopCount = Solution;
type Done = bool;
type CurrentPlace<'a> = &'a str;
type CountToEnd = Solution;

fn part_2((instructions, map): &mut ParseOutput) -> Solution {
    let mut all_starts: Vec<(CurrentPlace, End, CountToEnd, LoopCount, Done)> = map
        .keys()
        .filter(|k| k.as_bytes()[2] as char == 'A')
        .map(|k| (k.clone(), None, 0, 0, false))
        .collect();
    let mut count = 0;
    loop {
        for i in &mut *instructions {
            count += 1;
            for (
                ref mut current_place,
                ref mut end_option,
                ref mut count_from_start_to_end,
                ref mut loop_count,
                ref mut done,
            ) in all_starts.iter_mut()
            {
                if *done {
                    continue;
                }
                let new_place = match i {
                    'L' => map.get(current_place).unwrap().0.clone(),
                    'R' => map.get(current_place).unwrap().1.clone(),
                    _ => panic!(),
                };
                if new_place.as_bytes()[2] == 90 {
                    if end_option.is_some() {
                        *loop_count = count - *count_from_start_to_end;
                        *done = true;
                    } else {
                        *end_option = Some(new_place);
                        *count_from_start_to_end = count;
                    }
                }
                *current_place = new_place;
            }
        }
        if all_starts.iter().all(|(_, _, _, _, done)| *done) {
            break;
        }
    }
    lcm(&all_starts
        .iter()
        .map(|(_, _, _, loop_count, _)| *loop_count)
        .collect::<Vec<Solution>>())
}

pub fn lcm(nums: &[Solution]) -> Solution {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: Solution, b: Solution) -> Solution {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

fn main() {
    let parse_output = &mut parse(MAIN_INPUT);
    println!("Solution to part 1 is {}", part_1(parse_output));
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
        assert_eq!(part_1(&parse_output), 6);
    }

    #[test]
    pub fn test_part_2() {
        let parse_output = &mut parse(TEST_INPUT_2);
        assert_eq!(part_2(parse_output), 6);
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
            assert_eq!(part_1(black_box(&parse_output)), 21883);
        });
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let parse_output = &mut parse(MAIN_INPUT);
        b.iter(|| {
            assert_eq!(part_2(black_box(parse_output)), 12833235391111);
        });
    }
}
