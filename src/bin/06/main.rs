#![feature(test)]

type Solution = i64;
pub type ParseOutput<const T: usize> = [(Solution, Solution); T];
const MAIN_INPUT: ParseOutput<4> = [(40, 233), (82, 1011), (84, 1110), (92, 1487)];
const MAIN_INPUT_2: ParseOutput<1> = [(40828492, 233101111101487)];
const TEST_INPUT: ParseOutput<3> = [(7, 9), (15, 40), (30, 200)];
const TEST_INPUT_2: ParseOutput<1> = [(71530, 940200)];

pub fn parse<const T: usize>(file: ParseOutput<T>) -> ParseOutput<T> {
    file
}

fn part_1<const T: usize>((race): &ParseOutput<T>) -> Solution {
    race.iter()
        .map(|(time, distance)| {
            let start_t = 0.5 * (*time as f64 - (((time * time) - 4 * distance) as f64).sqrt());
            let end_t = 0.5 * (*time as f64 + (((time * time) - 4 * distance) as f64).sqrt());
            let sol = end_t.floor()
                - start_t.ceil()
                - (1.0 - end_t.fract().ceil())
                - (1.0 - start_t.fract().ceil())
                + 1.0;
            sol as Solution
        })
        .product::<Solution>()
}

fn part_2<const T: usize>(races: &ParseOutput<T>) -> Solution {
    part_1(races)
}

fn main() {
    let parse_output = &mut parse(MAIN_INPUT);
    println!("Solution to part 1 is {}", part_1(parse_output));
    let parse_output = &mut parse(MAIN_INPUT_2);
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
        assert_eq!(part_1(&parse_output), 288);
    }

    #[test]
    pub fn test_part_2() {
        let parse_output = &mut parse(TEST_INPUT_2);
        assert_eq!(part_2(parse_output), 71503);
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
            assert_eq!(part_1(black_box(&parse_output)), 3316275);
        });
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let parse_output = &mut parse(MAIN_INPUT_2);
        b.iter(|| {
            assert_eq!(part_2(black_box(parse_output)), 27102791);
        });
    }
}
