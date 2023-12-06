#![feature(test)]

type Solution = f64;
pub type ParseOutput<const T: usize> = [(Solution, Solution); T];
const MAIN_INPUT: ParseOutput<4> = [
    (40.0, 233.0),
    (82.0, 1011.0),
    (84.0, 1110.0),
    (92.0, 1487.0),
];
const MAIN_INPUT_2: ParseOutput<1> = [(40828492.0, 233101111101487.0)];
const TEST_INPUT: ParseOutput<3> = [(7.0, 9.0), (15.0, 40.0), (30.0, 200.0)];
const TEST_INPUT_2: ParseOutput<1> = [(71530.0, 940200.0)];

pub fn parse<const T: usize>(file: ParseOutput<T>) -> ParseOutput<T> {
    file
}

fn part_1<const T: usize>((race): &ParseOutput<T>) -> Solution {
    race.iter()
        .map(|(time, distance)| {
            let record_to_meet = distance + 1.0;
            let start_t = 0.5 * (*time - ((time * time) - 4.0 * record_to_meet).sqrt());
            let end_t = 0.5 * (*time + ((time * time) - 4.0 * record_to_meet).sqrt());
            end_t.floor() - start_t.ceil() + 1.0
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
        assert_eq!(part_1(&parse_output), 288.0);
    }

    #[test]
    pub fn test_part_2() {
        let parse_output = &mut parse(TEST_INPUT_2);
        assert_eq!(part_2(parse_output), 71503.0);
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
            assert_eq!(part_1(black_box(&parse_output)), 3316275.0);
        });
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let parse_output = &mut parse(MAIN_INPUT_2);
        b.iter(|| {
            assert_eq!(part_2(black_box(parse_output)), 27102791.0);
        });
    }
}
