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
    current_fill: Option<u8>,
}

impl Cell {
    pub fn new_template_starting_with(&self, c: u8) -> Cell {
        let mut cell = Cell {
            template: self.template.clone(),
            inserts_left: self.inserts_left.clone(),
            current_fill: self.current_fill.clone(),
        };
        cell.template[0] = c;
        cell
    }
    pub fn new_template_plus_1(&self) -> Cell {
        let mut cell = Cell {
            template: self.template[1..].to_owned(),
            inserts_left: self.inserts_left.clone(),
            current_fill: self.current_fill.clone(),
        };
        cell
    }
    pub fn consume_next_fill(&mut self) -> bool {
        if self.current_fill.is_none() {
            self.current_fill = self.inserts_left.pop();
        }
        match self.current_fill {
            Some(0) | None => false,
            Some(ref mut c) => {
                *c -= 1;
                true
            }
        }
    }

    pub fn reset_fill(&mut self) -> bool {
        match self.current_fill {
            Some(0) | None => {
                self.current_fill = None;
                true
            }
            _ => false,
        }
    }
    pub fn to_string(&self) -> String {
        format!(
            "[{} - {:?} #{:?}]",
            self.template.iter().map(|c| *c as char).collect::<String>(),
            self.inserts_left,
            self.current_fill
        )
    }
}

fn get_memo<'a, K, V>(map: &'a mut HashMap<K, V>, key: &K) -> &'a mut V
where
    K: Clone + Eq + Hash,
    V: Default,
{
    if !map.contains_key(key) {
        map.insert(key.clone(), V::default());
    }
    map.get_mut(key).unwrap()
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
        .map(|(mut arr, ins)| {
            let sol = solve_rec_2(
                Cell {
                    template: arr,
                    inserts_left: ins,
                    current_fill: None,
                },
                &mut memoization,
            );
            memoization.clear();
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
            let sol = solve_rec_2(
                Cell {
                    template: arr_2,
                    inserts_left: ins_2,
                    current_fill: None,
                },
                &mut memoization,
            );
            memoization.clear();
            sol
        })
        .sum()
}

fn solve_rec_2(mut current_cell: Cell, memo: &mut Memoization) -> Solution {
    if let Some(&result) = memo.get(&current_cell) {
        return result;
    }

    let current_cell_clone = current_cell.clone();
    let result: Solution = if current_cell.template.len() == 0 {
        if current_cell.inserts_left.len() == 0
            && (current_cell.current_fill.is_none() || Some(0) == current_cell.current_fill)
        {
            1
        } else {
            0
        }
    } else {
        match current_cell.template[0] {
            b'?' => {
                solve_rec_2(current_cell.new_template_starting_with(b'.'), memo)
                    + solve_rec_2(current_cell.new_template_starting_with(b'#'), memo)
            }
            b'#' => {
                let mut cell_copy = current_cell.clone();
                if !cell_copy.consume_next_fill() {
                    return 0;
                }

                solve_rec_2(cell_copy.new_template_plus_1(), memo)
            }
            b'.' => {
                let mut cell_copy = current_cell.clone();
                if !cell_copy.reset_fill() {
                    return 0;
                }

                solve_rec_2(cell_copy.new_template_plus_1(), memo)
            }
            _ => panic!(),
        }
    };
    memo.insert(current_cell_clone, result);
    result
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
            assert_eq!(part_2(black_box(parse_output)), 5071883216318);
        });
    }

    #[test]
    pub fn test_solve_rec_2() {
        let mut memoization: Memoization = HashMap::new();
        let sol = solve_rec_2(
            Cell {
                template: vec![b'?', b'?', b'?', b'.', b'#', b'#', b'#'],
                inserts_left: vec![3, 1, 1],
                current_fill: None,
            },
            &mut memoization,
        );
        println!(
            "{}",
            memoization
                .iter()
                .map(|(c, s)| format!("{} - {}\n", c.to_string(), s))
                .collect::<String>()
        );
        assert_eq!(sol, 1, "first");
        memoization.clear();
    }
}
