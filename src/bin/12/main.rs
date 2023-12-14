#![feature(test)]
#![feature(ascii_char)]

use std::collections::HashMap;
use std::hash::Hash;

type Solution = u64;
pub type ParseOutput = Vec<(Vec<u8>, Vec<u8>)>;
const MAIN_INPUT: &str = include_str!("main_input");
const TEST_INPUT: &str = include_str!("test_input");
pub type Memoization = HashMap<Cell, Solution>;
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Cell {
    template: Vec<u8>,
    inserts_left: Vec<u8>,
}

impl Cell {
    pub fn to_string(&self) -> String {
        format!(
            "[{} - {:?}]",
            self.template.iter().map(|c| *c as char).collect::<String>(),
            self.inserts_left
        )
    }
}

pub fn parse(file: &str) -> ParseOutput {
    file.lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let (arr, ins) = l.split_once(" ").unwrap();
            (
                arr.as_bytes().iter().map(|n| *n).collect(),
                ins.rsplit(",").map(|n| n.parse().unwrap()).collect(),
            )
        })
        .collect()
}

fn part_1(out: &ParseOutput) -> Solution {
    let mut memoization = HashMap::new();
    out.clone()
        .into_iter()
        .skip(1)
        .map(|(mut arr, ins)| {
            let sol = solve_rec(arr, ins, 0, &mut memoization);
            println!(
                "{}",
                memoization
                    .iter()
                    .map(|(c, s)| format!("{} - {}\n", c.to_string(), s))
                    .collect::<String>()
            );
            memoization.clear();
            panic!("one");
            sol
        })
        .sum()
}

fn part_2(out: &mut ParseOutput) -> Solution {
    let mut memoization = HashMap::new();
    out.clone()
        .into_iter()
        .map(|(mut arr, ins)| {
            let mut arr_2 = Vec::new();
            let mut ins_2 = Vec::new();
            for _ in 0..5 {
                arr_2.extend(arr.clone());
                arr_2.push(b'?');
                ins_2.extend(ins.clone());
            }
            arr_2.pop();
            let mut sol = solve_rec(arr_2, ins_2, 0, &mut memoization);
            memoization.clear();
            sol
        })
        .sum()
}

fn solve_rec(
    mut current_sol: Vec<u8>,
    mut left_ins: Vec<u8>,
    start_i: usize,
    memoization: &mut Memoization,
) -> Solution {
    fn handle_num(fill: &mut Option<u8>, left_ins: &mut Vec<u8>) -> bool {
        if fill.is_none() {
            *fill = left_ins.pop();
        }

        if let Some(mut c) = fill {
            if c == 0 {
                // Not enough to fill
                return false;
            }
            c -= 1;
            *fill = Some(c);
            true
        } else {
            false
        }
    }
    let mut current_fill = None;
    for i in start_i..current_sol.len() {
        match current_sol[i] {
            b'?' => {
                println!("Is ?");
                println!(
                    "we are at {:?}",
                    current_sol.iter().map(|c| *c as char).collect::<String>()
                );
                // . case
                println!(". case");
                match &current_fill {
                    Some(0) | None => {
                        println!("No current fill so we can proceed");
                        let cell = Cell {
                            template: current_sol[i..].to_owned(),
                            inserts_left: left_ins.clone(),
                        };
                        let mut c = current_sol.clone();
                        c[i] = b'.';
                        let next_i = match c[i + 1..].iter().position(|c| *c == b'?') {
                            Some(q_i) => i + 1 + q_i,
                            None => i + 1,
                        };
                        let cell_next = Cell {
                            template: c[next_i..].to_owned(),
                            inserts_left: left_ins.clone(),
                        };
                        println!(
                            "going deeper {} {} {}",
                            cell.to_string(),
                            cell_next.to_string(),
                            i
                        );
                        *memoization.entry(cell).or_default() += match memoization.get(&cell_next) {
                            Some(c) => {
                                println!("Cache hit? {}", c);

                                *c
                            }
                            None => {
                                let w = solve_rec(c.clone(), left_ins.clone(), i + 1, memoization);
                                println!(
                                    "back {} {:?} {:?}",
                                    w,
                                    current_sol[i..]
                                        .iter()
                                        .map(|c| *c as char)
                                        .collect::<String>(),
                                    left_ins
                                );
                                w
                            }
                        };
                    }
                    _ => {
                        println!("Fill active so skip");
                    }
                }
                println!(
                    "we are at after {:?} {}",
                    current_sol.iter().map(|c| *c as char).collect::<String>(),
                    i
                );
                // # case
                println!("# case");
                if !handle_num(&mut current_fill, &mut left_ins) {
                    println!("Break in ? {:?}", left_ins);
                    return 0;
                }
                current_sol[i] = b'#';
                println!(
                    "inserted # {:?}",
                    current_sol.iter().map(|c| *c as char).collect::<String>()
                );
            }
            b'#' => {
                println!("Is #");
                if !handle_num(&mut current_fill, &mut left_ins) {
                    println!("Break in # {:?}", left_ins);
                    return 0;
                }
            }
            b'.' => {
                println!("Is .");
                if let Some(c) = current_fill {
                    if c == 0 {
                        // back to no fill
                        current_fill = None;
                    } else {
                        // Break if we are filling up right now and we still have something left to fill
                        println!("Break in . {:?}", left_ins);
                        return 0;
                    }
                }
            }
            d => panic!("{}", d),
        }
    }

    let end = match (current_fill, left_ins.is_empty()) {
        (Some(0) | None, true) => 1,
        _ => 0,
    };
    println!(
        "Got to the end {} {} {}",
        end,
        current_sol.iter().map(|c| *c as char).collect::<String>(),
        current_sol[start_i..]
            .iter()
            .map(|c| *c as char)
            .collect::<String>()
    );
    end
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
        assert_eq!(part_1(&parse_output), 21);
    }

    #[test]
    pub fn test_part_2() {
        let parse_output = &mut parse(TEST_INPUT);
        assert_eq!(part_2(parse_output), 525152);
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
            assert_eq!(part_1(black_box(&parse_output)), 7694);
        });
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let parse_output = &mut parse(MAIN_INPUT);
        b.iter(|| {
            assert_eq!(part_1(black_box(parse_output)), 702152204842);
        });
    }

    #[test]
    pub fn test_solve_rec() {
        let mut sol = 0;
        let mut memoization = HashMap::new();
        solve_rec(
            vec![b'?', b'?', b'?', b'.', b'#', b'#', b'#'],
            vec![3, 1, 1],
            0,
            &mut memoization,
        );
        assert_eq!(sol, 1, "first");
        memoization.clear();
        let mut sol = 0;
        solve_rec(
            vec![
                b'.', b'?', b'?', b'.', b'.', b'?', b'?', b'.', b'.', b'.', b'?', b'#', b'#', b'.',
            ],
            vec![3, 1, 1],
            0,
            &mut memoization,
        );
        assert_eq!(sol, 4);
        memoization.clear();
        let mut sol = 0;
        solve_rec(
            vec![
                b'?', b'#', b'?', b'#', b'?', b'#', b'?', b'#', b'?', b'#', b'?', b'#', b'?', b'#',
                b'?',
            ],
            vec![6, 1, 3, 1],
            0,
            &mut memoization,
        );
        assert_eq!(sol, 1);
        memoization.clear();
        let mut sol = 0;
        solve_rec(
            vec![
                b'?', b'?', b'?', b'?', b'.', b'#', b'.', b'.', b'.', b'#', b'.', b'.', b'.',
            ],
            vec![1, 1, 4],
            0,
            &mut memoization,
        );
        assert_eq!(sol, 1);
        memoization.clear();
        let mut sol = 0;
        solve_rec(
            vec![
                b'?', b'?', b'?', b'?', b'.', b'#', b'#', b'#', b'#', b'#', b'#', b'.', b'.', b'#',
                b'#', b'#', b'#', b'#', b'.',
            ],
            vec![5, 6, 1],
            0,
            &mut memoization,
        );
        assert_eq!(sol, 4);
        memoization.clear();
        let mut sol = 0;
        solve_rec(
            vec![
                b'?', b'#', b'#', b'#', b'?', b'?', b'?', b'?', b'?', b'?', b'?', b'?',
            ],
            vec![1, 2, 3],
            0,
            &mut memoization,
        );
        assert_eq!(sol, 10);
        memoization.clear();
    }
}
