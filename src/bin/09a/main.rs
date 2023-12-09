#![feature(test)]
#![feature(generic_const_exprs)]

type Solution = i32;
pub type DiffVec<const N: usize> = [Solution; N * (N + 1) / 2];
pub type ParseOutput<const N: usize> = Vec<DiffVec<N>>;
const MAIN_INPUT: &str = include_str!("main_input");
const TEST_INPUT: &str = include_str!("test_input");

pub fn parse<const N: usize>(file: &str) -> ParseOutput<N>
where
    [(); N * (N + 1) / 2]:,
{
    file.lines()
        .map(|l| {
            let mut diff_vec = [0; N * (N + 1) / 2];
            for (i, num) in l
                .split(" ")
                .map(|n| n.parse::<Solution>().unwrap())
                .enumerate()
            {
                diff_vec[i] = num;
            }
            diff_vec
        })
        .collect()
}

fn part_1<const N: usize>(measure_categories: &ParseOutput<N>) -> Solution
where
    [(); N * (N + 1) / 2]:,
{
    measure_categories
        .clone()
        .iter_mut()
        .map(|mut measurements| get_next_value::<N>(&mut measurements, 0, N, 0))
        .sum()
}

fn part_2<const N: usize>(measure_categories: &ParseOutput<N>) -> Solution
where
    [(); N * (N + 1) / 2]:,
{
    measure_categories
        .clone()
        .iter_mut()
        .map(|mut measurements| get_prev_value::<N>(&mut measurements, 0, N, 0))
        .sum()
}

fn get_next_value<const N: usize>(
    measures: &mut DiffVec<N>,
    start: usize,
    end: usize,
    depth: usize,
) -> Solution
where
    [(); N * (N + 1) / 2]:,
{
    let mut sum_of_new_measures = 0;
    for i in start..end - 1 {
        measures[i + N - depth] = measures[i + 1] - measures[i];
        sum_of_new_measures += measures[i + N - depth];
    }
    if sum_of_new_measures == 0 {
        return measures[end - 1];
    }

    measures[end - 1] + get_next_value(measures, end, end + N - depth - 1, depth + 1)
}

fn get_prev_value<const N: usize>(
    measures: &mut DiffVec<N>,
    start: usize,
    end: usize,
    depth: usize,
) -> Solution
where
    [(); N * (N + 1) / 2]:,
{
    let mut sum_of_new_measures = 0;
    for i in start..end - 1 {
        measures[i + N - depth] = measures[i + 1] - measures[i];
        sum_of_new_measures += measures[i + N - depth];
    }
    if sum_of_new_measures == 0 {
        return measures[start];
    }

    measures[start] - get_prev_value(measures, end, end + N - depth - 1, depth + 1)
}

fn main() {
    let parse_output = &mut parse::<21>(MAIN_INPUT);
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
        let parse_output = parse::<6>(TEST_INPUT);
        assert_eq!(part_1(&parse_output), 114);
    }

    #[test]
    pub fn test_part_2() {
        let parse_output = &mut parse::<6>(TEST_INPUT);
        assert_eq!(part_2(parse_output), 2);
    }

    #[bench]
    fn bench_parse(b: &mut Bencher) {
        b.iter(|| {
            let _ = parse::<21>(MAIN_INPUT);
        });
    }

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        let parse_output = parse::<21>(MAIN_INPUT);
        b.iter(move || {
            assert_eq!(part_1(black_box(&parse_output)), 1798691765);
        });
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let parse_output = &mut parse::<21>(MAIN_INPUT);
        b.iter(|| {
            assert_eq!(part_2(black_box(parse_output)), 1104);
        });
    }
}
