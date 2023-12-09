#![feature(test)]

type Solution = i64;
pub type ParseOutput = Vec<Vec<Solution>>;
const MAIN_INPUT: &str = include_str!("main_input");
const TEST_INPUT: &str = include_str!("test_input");

fn part_1(measures: &ParseOutput) -> Solution {
    let mut diffs = get_measure_diffs(measures);

    diffs
        .iter()
        .map(|d| d.iter().rev().fold(0, |last, n| n.last().unwrap() + last))
        .sum()
}

fn part_2(measures: &mut ParseOutput) -> Solution {
    let mut m_diffs = get_measure_diffs(measures);

    m_diffs
        .iter()
        .map(|m_diff| m_diff.iter().rev().fold(0, |first_n, n| n[0] - first_n))
        .sum()
}

fn get_measure_diffs(measures: &ParseOutput) -> Vec<Vec<Vec<Solution>>> {
    let mut sol_vec = Vec::new();
    for measure in measures {
        let mut new_measures = vec![measure.clone()];
        let mut current_m_i = 0;
        while new_measures[current_m_i].iter().sum::<Solution>() != 0 {
            let mut new_measure = Vec::new();
            for m_n in 0..new_measures[current_m_i].len() - 1 {
                new_measure
                    .push(new_measures[current_m_i][m_n + 1] - new_measures[current_m_i][m_n]);
            }
            new_measures.push(new_measure);
            current_m_i += 1;
        }
        sol_vec.push(new_measures);
    }

    return sol_vec;
}

pub fn parse(file: &str) -> ParseOutput {
    file.lines()
        .map(|l| {
            l.split(" ")
                .map(|n| n.parse::<Solution>().unwrap())
                .collect()
        })
        .collect()
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
        assert_eq!(part_1(&parse_output), 114);
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
            assert_eq!(part_1(black_box(&parse_output)), 1798691765);
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
