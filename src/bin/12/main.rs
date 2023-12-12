#![feature(test)]

type Solution = u64;
pub type ParseOutput = Vec<(Vec<u8>, Vec<u8>)>;
const MAIN_INPUT: &str = include_str!("main_input");
const TEST_INPUT: &str = include_str!("test_input");

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
    out.clone()
        .into_iter()
        .map(|(mut arr, ins)| {
            let mut sol = 0;
            solve_rec(&mut arr, ins, 0, &mut sol);

            sol
        })
        .sum()
}

fn part_2(out: &mut ParseOutput) -> Solution {
    out.clone()
        .into_iter()
        .map(|(mut arr, ins)| {
            let mut sol = 0;
            let mut arr_2 = Vec::new();
            let mut ins_2 = Vec::new();
            for _ in 0..5 {
                arr_2.extend(arr.clone());
                arr_2.push(b'?');
                ins_2.extend(ins.clone());
            }
            arr_2.pop();
            solve_rec(&mut arr_2, ins_2, 0, &mut sol);
            println!(
                "now! {:?} {:?}",
                arr_2.iter().map(|b| *b as char).collect::<String>(),
                sol,
            );
            println!("{}", sol);
            sol
        })
        .sum()
}

fn solve_rec(
    current_sol: &mut Vec<u8>,
    mut left_ins: Vec<u8>,
    start_i: usize,
    sol_count: &mut Solution,
) {
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
                // . case
                match &current_fill {
                    Some(0) | None => solve_rec(current_sol, left_ins.clone(), i + 1, sol_count),
                    _ => {}
                }
                // # case
                if !handle_num(&mut current_fill, &mut left_ins) {
                    return;
                }
            }
            b'#' => {
                if !handle_num(&mut current_fill, &mut left_ins) {
                    return;
                }
            }
            b'.' => {
                if let Some(c) = current_fill {
                    if c == 0 {
                        // back to no fill
                        current_fill = None;
                    } else {
                        // Break if we are filling up right now and we still have something left to fill
                        return;
                    }
                }
            }
            d => panic!("{}", d),
        }
    }
    match (current_fill, left_ins.is_empty()) {
        (Some(0) | None, true) => {
            *sol_count += 1;
        }
        _ => {}
    }
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
        solve_rec(
            &mut vec![b'?', b'?', b'?', b'.', b'#', b'#', b'#'],
            vec![3, 1, 1],
            0,
            &mut sol,
        );
        assert_eq!(sol, 1, "first");

        let mut sol = 0;
        solve_rec(
            &mut vec![
                b'.', b'?', b'?', b'.', b'.', b'?', b'?', b'.', b'.', b'.', b'?', b'#', b'#', b'.',
            ],
            vec![3, 1, 1],
            0,
            &mut sol,
        );
        assert_eq!(sol, 4);

        let mut sol = 0;
        solve_rec(
            &mut vec![
                b'?', b'#', b'?', b'#', b'?', b'#', b'?', b'#', b'?', b'#', b'?', b'#', b'?', b'#',
                b'?',
            ],
            vec![6, 1, 3, 1],
            0,
            &mut sol,
        );
        assert_eq!(sol, 1);

        let mut sol = 0;
        solve_rec(
            &mut vec![
                b'?', b'?', b'?', b'?', b'.', b'#', b'.', b'.', b'.', b'#', b'.', b'.', b'.',
            ],
            vec![1, 1, 4],
            0,
            &mut sol,
        );
        assert_eq!(sol, 1);

        let mut sol = 0;
        solve_rec(
            &mut vec![
                b'?', b'?', b'?', b'?', b'.', b'#', b'#', b'#', b'#', b'#', b'#', b'.', b'.', b'#',
                b'#', b'#', b'#', b'#', b'.',
            ],
            vec![5, 6, 1],
            0,
            &mut sol,
        );
        assert_eq!(sol, 4);

        let mut sol = 0;
        solve_rec(
            &mut vec![
                b'?', b'#', b'#', b'#', b'?', b'?', b'?', b'?', b'?', b'?', b'?', b'?',
            ],
            vec![1, 2, 3],
            0,
            &mut sol,
        );
        assert_eq!(sol, 10);
    }

    #[test]
    pub fn test_solve_rec_2() {
        let mut sol = 0;
        solve_rec(
            &mut vec![b'?', b'?', b'?', b'?', b'#', b'#', b'#', b'#', b'?', b'?'],
            vec![1, 4, 1],
            0,
            &mut sol,
        );
        assert_eq!(sol, 10);
    }
}
