#![feature(test)]
#![feature(let_chains)]

use fnv::{FnvHashMap as HashMap, FnvHashSet};

type Solution = u64;
pub type ParseOutput = (Vec<Module>, ModuleID, ModuleID, ModuleID);
pub type ModuleID = usize;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Cell {
    ff_states: Vec<bool>,
}

#[derive(Debug, Clone)]
pub struct FlipFlop {
    state: bool,
    destinations: Vec<ModuleID>,
}
#[derive(Debug, Clone)]
pub struct Conjunction {
    inputs: HashMap<ModuleID, bool>,
    destinations: Vec<ModuleID>,
}
#[derive(Debug, Clone)]
pub enum Module {
    Broadcaster(Vec<ModuleID>),
    FlipFlop(FlipFlop),
    Conjunction(Conjunction),
}
const MAIN_INPUT: &str = include_str!("main_input");

fn find_corresponding_end(
    modules: &mut Vec<Module>,
    c: usize,
    marked: &mut FnvHashSet<usize>,
    end_id: usize,
) -> Option<usize> {
    if !marked.insert(c) {
        return None;
    }

    for d in match modules.get(c).unwrap() {
        Module::FlipFlop(ff) => ff.destinations.clone(),
        Module::Conjunction(cj) => cj.destinations.clone(),
        Module::Broadcaster(b) => b.clone(),
    } {
        if d == end_id {
            return Some(c);
        }

        if let Some(f) = find_corresponding_end(modules, d, marked, end_id) {
            return Some(f);
        }
    }
    None
}

fn part_2(out: &mut ParseOutput) -> Solution {
    let (mut modules, broadcaster_id, _, before_output_id) = out.clone();
    let mut signals: Vec<(ModuleID, ModuleID, bool)> = Vec::new();
    let mut new_signals = Vec::new();
    let starts = if let Some(Module::Broadcaster(vec)) = modules.get(broadcaster_id) {
        vec.clone()
    } else {
        panic!("Starts!!")
    };

    let mut marked = FnvHashSet::<usize>::default();
    let ends = starts
        .iter()
        .filter_map(|s| find_corresponding_end(&mut modules, *s, &mut marked, before_output_id))
        .collect::<Vec<usize>>();
    let mut presses = 1;
    for (i, s) in starts.iter().enumerate() {
        let mut button_presses = 0;
        'outer: loop {
            button_presses += 1;
            signals.push((broadcaster_id, *s, false));
            while !signals.is_empty() {
                for (from, to, signal) in signals.drain(..) {
                    if to == ends[i] && !signal {
                        break 'outer;
                    }
                    match modules.get_mut(to).unwrap() {
                        Module::Broadcaster(b) => {
                            for d in b {
                                new_signals.push((to, *d, signal));
                            }
                        }
                        Module::FlipFlop(ff) => {
                            if !signal {
                                ff.state = !ff.state;
                                for d in &ff.destinations {
                                    new_signals.push((to, *d, ff.state));
                                }
                            }
                        }
                        Module::Conjunction(cj) => {
                            cj.inputs.insert(from, signal);
                            let pulse = !cj.inputs.values().all(|s| *s);
                            for d in &cj.destinations {
                                new_signals.push((to, *d, pulse));
                            }
                        }
                    }
                }
                signals.extend(new_signals.drain(..));
            }
        }
        presses *= button_presses;
    }

    presses
}

fn part_1(out: &ParseOutput) -> Solution {
    let (mut modules, broadcaster_id, _, _) = out.clone();
    let mut signals: Vec<(ModuleID, ModuleID, bool)> = Vec::new();
    let mut new_signals = Vec::new();
    let mut sig_map: [Solution; 2] = [0; 2];
    for _ in 0..1000 {
        sig_map[0] += 1;
        signals.push((999999999, broadcaster_id, false));
        while !signals.is_empty() {
            for (from, to, signal) in signals.drain(..) {
                match modules.get_mut(to).unwrap() {
                    Module::Broadcaster(b) => {
                        for d in b {
                            sig_map[signal as usize] += 1;
                            new_signals.push((to, *d, signal));
                        }
                    }
                    Module::FlipFlop(ff) => {
                        if !signal {
                            for d in &ff.destinations {
                                let pulse = !ff.state;
                                sig_map[pulse as usize] += 1;
                                new_signals.push((to, *d, pulse));
                            }
                            ff.state = !ff.state;
                        }
                    }
                    Module::Conjunction(cj) => {
                        cj.inputs.insert(from, signal);
                        let pulse = !cj.inputs.values().all(|s| *s);
                        for d in &cj.destinations {
                            sig_map[pulse as usize] += 1;
                            new_signals.push((to, *d, pulse));
                        }
                    }
                }
            }
            signals.extend(new_signals.drain(..));
        }
    }
    sig_map[0] * sig_map[1]
}

pub fn parse(file: &'static str) -> ParseOutput {
    let mut modules = Vec::new();
    let mut broadcaster_id = 0;
    let mut destination_map: HashMap<&'static str, usize> = HashMap::default();
    let lines = file.lines().filter(|l| !l.is_empty()).enumerate();
    for (c, l) in lines {
        let (module, _) = l.split_once(" ->").unwrap();
        modules.push(match &module[0..1] {
            "%" => {
                destination_map.insert(&module[1..], c);
                Module::FlipFlop(FlipFlop {
                    destinations: Vec::new(),
                    state: false,
                })
            }
            "&" => {
                destination_map.insert(&module[1..], c);
                Module::Conjunction(Conjunction {
                    inputs: HashMap::default(),
                    destinations: Vec::new(),
                })
            }
            _ => {
                destination_map.insert(&module, c);
                if module == "broadcaster" {
                    broadcaster_id = c;
                }
                Module::Broadcaster(Vec::new())
            }
        });
    }
    let lines = file.lines().filter(|l| !l.is_empty()).enumerate();
    let output_id = modules.len();
    let mut before_output_id = 0;
    modules.insert(output_id, Module::Broadcaster(Vec::new()));

    for (c, l) in lines {
        if let Some((_, destinations_unsplit)) = l.split_once(" -> ") {
            let destinations = destinations_unsplit.split(", ");
            let current_module = modules.get_mut(c).unwrap();
            for destination in destinations.clone() {
                match current_module {
                    Module::Broadcaster(d) => match destination_map.get(destination) {
                        Some(&d_id) => {
                            d.push(d_id);
                        }
                        None => {
                            before_output_id = c;
                            d.push(output_id);
                        }
                    },
                    Module::FlipFlop(ff) => match destination_map.get(destination) {
                        Some(&d_id) => {
                            ff.destinations.push(d_id);
                        }
                        None => {
                            before_output_id = c;
                            ff.destinations.push(output_id);
                        }
                    },
                    Module::Conjunction(cj) => match destination_map.get(destination) {
                        Some(&d_id) => {
                            cj.destinations.push(d_id);
                        }
                        None => {
                            before_output_id = c;
                            cj.destinations.push(output_id);
                        }
                    },
                }
            }
            for destination in destinations {
                let destination_module_id = match destination_map.get(destination) {
                    Some(&d_id) => d_id,
                    None => output_id,
                };
                let destination_module = modules.get_mut(destination_module_id).unwrap();
                if let Module::Conjunction(cj) = destination_module {
                    cj.inputs.insert(c, false);
                }
            }
        }
    }

    (modules, broadcaster_id, output_id, before_output_id)
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
    const TEST_INPUT: &str = include_str!("test_input");

    #[test]
    pub fn test_part_1() {
        let parse_output = parse(TEST_INPUT);
        assert_eq!(part_1(&parse_output), 11687500);
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
            assert_eq!(part_1(black_box(&parse_output)), 812721756);
        });
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let parse_output = &mut parse(MAIN_INPUT);
        b.iter(|| {
            assert_eq!(part_2(black_box(parse_output)), 233338595643977);
        });
    }
}
