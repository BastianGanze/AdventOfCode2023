#![feature(test)]

type Solution = u32;
pub type ParseOutput<'a> = (Vec<String>, [WordNumberSearchToken<'a>; 9]);
const MAIN_INPUT: &str = include_str!("main_input");
const TEST_INPUT: &str = include_str!("test_input");
const TEST_INPUT_2: &str = include_str!("test_input_2");

pub fn parse(file: &str) -> ParseOutput {
    (
        file.lines().map(|l| l.to_string()).collect(),
        [
            WordNumberSearchToken::new("one", 1),
            WordNumberSearchToken::new("two", 2),
            WordNumberSearchToken::new("three", 3),
            WordNumberSearchToken::new("four", 4),
            WordNumberSearchToken::new("five", 5),
            WordNumberSearchToken::new("six", 6),
            WordNumberSearchToken::new("seven", 7),
            WordNumberSearchToken::new("eight", 8),
            WordNumberSearchToken::new("nine", 9),
        ],
    )
}

pub struct WordNumberSearchToken<'a> {
    pub current_cursor: usize,
    pub chars: &'a [u8],
    pub value: u32,
}

impl<'a> WordNumberSearchToken<'a> {
    pub fn new(string: &str, value: u32) -> WordNumberSearchToken {
        WordNumberSearchToken {
            current_cursor: 0,
            chars: string.as_bytes(),
            value,
        }
    }

    pub fn check_token_for_value(&mut self, c: char) -> Option<u32> {
        if self.chars[self.current_cursor] as char == c {
            self.current_cursor += 1;
            if self.current_cursor == self.chars.len() {
                return Some(self.value);
            }
        } else if self.chars[0] as char == c {
            self.current_cursor = 1;
        } else {
            self.reset();
        }

        None
    }

    pub fn check_token_for_value_rev(&mut self, c: char) -> Option<u32> {
        if self.chars[self.current_cursor] as char == c {
            if self.current_cursor == 0 {
                return Some(self.value);
            } else {
                self.current_cursor -= 1;
            }
        } else if self.chars[self.chars.len() - 1] as char == c {
            self.current_cursor = self.chars.len() - 2;
        } else {
            self.reset_rev();
        }

        None
    }

    pub fn reset(&mut self) {
        self.current_cursor = 0;
    }

    pub fn reset_rev(&mut self) {
        self.current_cursor = self.chars.len() - 1;
    }
}

fn part_1(parse_output: &ParseOutput) -> Solution {
    let mut solution = 0;
    let (lines, _) = parse_output;
    for l in lines {
        let first = l.chars().find_map(|c| c.to_digit(10)).unwrap();
        let last = l.chars().rev().find_map(|c| c.to_digit(10)).unwrap();
        solution = solution + (first * 10) + last;
    }
    solution
}

fn part_2(parse_output: &mut ParseOutput) -> Solution {
    let mut solution = 0;
    let (lines, tokens) = parse_output;

    for l in lines {
        tokens.iter_mut().for_each(|t| t.reset());
        let first = l
            .chars()
            .find_map(|c| {
                c.to_digit(10)
                    .or(tokens.iter_mut().find_map(|t| t.check_token_for_value(c)))
            })
            .unwrap();
        tokens.iter_mut().for_each(|t| t.reset_rev());
        let last = l
            .chars()
            .rev()
            .find_map(|c| {
                c.to_digit(10).or(tokens
                    .iter_mut()
                    .find_map(|t| t.check_token_for_value_rev(c)))
            })
            .unwrap();
        solution = solution + (first * 10) + last;
    }
    solution
}

fn part_2_old(parse_output: &ParseOutput) -> Solution {
    let mut solution = 0;
    let mut current_word: String = "".into();
    let (lines, _) = parse_output;
    for l in lines {
        let mut first = 0;
        let mut last = 0;
        for c in l.chars() {
            let mut num = c.to_digit(10);
            if num.is_none() {
                current_word.push(c);
                let n = if current_word.contains("one") {
                    Some(1)
                } else if current_word.contains("two") {
                    Some(2)
                } else if current_word.contains("three") {
                    Some(3)
                } else if current_word.contains("four") {
                    Some(4)
                } else if current_word.contains("five") {
                    Some(5)
                } else if current_word.contains("six") {
                    Some(6)
                } else if current_word.contains("seven") {
                    Some(7)
                } else if current_word.contains("eight") {
                    Some(8)
                } else if current_word.contains("nine") {
                    Some(9)
                } else {
                    None
                };
                if n.is_some() {
                    current_word.clear();
                    current_word.push(c);
                }
                num = n;
            } else {
                current_word.clear();
            }

            if let Some(d) = num {
                current_word.clear();
                if c.to_digit(10).is_none() {
                    current_word.push(c);
                }
                if first == 0 {
                    first = d * 10;
                }
                last = d;
            }
        }
        solution = solution + first + last;
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
    use super::*;
    use test::{black_box, Bencher};

    #[test]
    pub fn test_part_1() {
        let parse_output = parse(TEST_INPUT);
        assert_eq!(part_1(&parse_output), 142);
    }

    #[test]
    pub fn test_part_2() {
        let parse_output = &mut parse(TEST_INPUT_2);
        assert_eq!(part_2(parse_output), 281);
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
            assert_eq!(part_1(black_box(&parse_output)), 54877);
        });
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let parse_output = &mut parse(MAIN_INPUT);
        b.iter(|| {
            assert_eq!(part_2(black_box(parse_output)), 54100);
        });
    }
}
