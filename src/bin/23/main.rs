#![feature(test)]

use std::cmp::{max, Ordering};
use std::collections::BinaryHeap;
use std::hash::Hash;
use std::time::Instant;

use fnv::{FnvHashMap, FnvHashSet};

type Solution = i32;
type Grid = Vec<Vec<u8>>;
pub type ParseOutput = (Grid, (usize, usize));

type Graph = FnvHashMap<(usize, usize), FnvHashMap<(usize, usize), Solution>>;
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Field(usize, usize, Solution, FnvHashSet<(usize, usize)>);

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FieldN(
    usize,
    usize,
    Solution,
    FnvHashSet<(usize, usize)>,
    Vec<(usize, usize)>,
);
const MAIN_INPUT: &str = include_str!("main_input");
const TEST_INPUT: &str = include_str!("test_input");

fn part_1((grid, start): &ParseOutput) -> Solution {
    let mut current_steps: BinaryHeap<Field> = BinaryHeap::new();
    current_steps.push(Field(start.0, start.1, 0, FnvHashSet::default()));
    let mut max_path = Solution::MIN;
    while let Some(Field(y, x, path_length, mut visited)) = current_steps.pop() {
        visited.insert((y, x));
        max_path = max(max_path, path_length);
        for (n_y, n_x) in get_neighbours(grid, (y, x), &visited) {
            current_steps.push(Field(n_y, n_x, path_length + 1, visited.clone()));
        }
    }
    max_path
}

fn part_2(out: &mut ParseOutput) -> Solution {
    let before = Instant::now();
    let graph = transform_grid(out);
    let (grid, start) = out;
    let end = (grid.len() - 1, grid.len() - 2);
    let mut current_steps: BinaryHeap<Field> = BinaryHeap::new();
    current_steps.push(Field(start.0, start.1, 0, FnvHashSet::default()));
    let mut max_path = Solution::MIN;
    while let Some(Field(y, x, path_length, mut visited)) = current_steps.pop() {
        if visited.contains(&(y, x)) {
            continue;
        }
        visited.insert((y, x));
        if (y, x) == end {
            max_path = max(max_path, path_length);
            continue;
        }

        for (n_y, n_x, n_path_length) in get_neighbours_g(&graph, (y, x), &visited) {
            current_steps.push(Field(
                n_y,
                n_x,
                path_length + n_path_length,
                visited.clone(),
            ));
        }
    }
    println!(
        "Part 2 took {}ms",
        Instant::now()
            .duration_since(before)
            .as_millis()
            .to_string()
            .as_bytes()
            .rchunks(3)
            .rev()
            .map(std::str::from_utf8)
            .collect::<Result<Vec<&str>, _>>()
            .unwrap()
            .join(",")
    );
    max_path
}

fn transform_grid((grid, start): &mut ParseOutput) -> Graph {
    let mut graph = Graph::default();
    let mut current_steps: BinaryHeap<Field> = BinaryHeap::new();
    let mut visited: FnvHashSet<(usize, usize)> = FnvHashSet::default();
    current_steps.push(Field(start.0, start.1, 0, FnvHashSet::default()));
    graph.insert((start.0, start.1), FnvHashMap::default());
    while let Some(Field(y, x, path_length, v)) = current_steps.pop() {
        if visited.contains(&(y, x)) {
            continue;
        }
        visited.insert((y, x));
        let n = get_neighbours_climb(grid, (y, x), &v);a
        if n.len() > 2 {
            graph.insert((y, x), FnvHashMap::default());
        }
        for (n_y, n_x) in n.iter().filter(|na| !visited.contains(*na)) {
            current_steps.push(Field(*n_y, *n_x, path_length + 1, v.clone()));
        }
    }
    graph.insert((grid.len() - 1, grid.len() - 2), FnvHashMap::default());
    visited.clear();
    let mut edges_to_explore: Vec<((usize, usize), (usize, usize))> = Vec::new();
    edges_to_explore.push((start.clone(), (start.0 + 1, start.1)));
    visited.insert(*start);
    while let Some((start_node_id, mut current)) = edges_to_explore.pop() {
        let mut path_length = 1;
        loop {
            if graph.get_mut(&current).is_none() {
                visited.insert(current);
            }
            if let Some(end_node) = graph.get_mut(&current) {
                let n = get_neighbours(grid, current, &visited);
                end_node.insert(start_node_id, path_length);
                for e in n {
                    edges_to_explore.push((current, e))
                }
                break;
            }
            let n = get_neighbours(grid, current, &visited);
            current = match n.len() {
                0 => panic!("opfpo9sfkaeopsik"),
                1 => n[0],
                2 => *n.iter().find(|p| !graph.contains_key(p)).unwrap(),
                _ => panic!(),
            };
            path_length += 1;
        }
        assert!(graph.get_mut(&current).is_some());
        let end_node = graph.get_mut(&start_node_id).unwrap();
        end_node.insert(current, path_length);
    }
    graph
}

pub fn parse(file: &'static str) -> ParseOutput {
    let grid: Grid = file
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.as_bytes().iter().map(|c| *c).collect())
        .collect();
    (grid, (0, 1))
}

fn get_neighbours_g(
    graph: &Graph,
    (y, x): (usize, usize),
    visited: &FnvHashSet<(usize, usize)>,
) -> Vec<(usize, usize, Solution)> {
    graph
        .get(&(y, x))
        .unwrap()
        .iter()
        .filter_map(|((y, x), l)| {
            if !visited.contains(&(*y, *x)) {
                Some((*y, *x, *l))
            } else {
                None
            }
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

    use test::{black_box, Bencher};

    use super::*;

    #[test]
    pub fn test_part_1() {
        let parse_output = parse(TEST_INPUT);
        assert_eq!(part_1(&parse_output), 94);
    }

    #[test]
    pub fn test_part_2() {
        let parse_output = &mut parse(TEST_INPUT);
        assert_eq!(part_2(parse_output), 154);
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

pub fn manhattan_distance(p1: (usize, usize), p2: (usize, usize)) -> Solution {
    (p1.0.abs_diff(p2.0) + p1.1.abs_diff(p2.1)) as Solution
}

fn get_neighbours(
    grid: &Grid,
    (y, x): (usize, usize),
    visited: &FnvHashSet<(usize, usize)>,
) -> Vec<(usize, usize)> {
    let mut n = Vec::new();
    let n_x = x.checked_sub(1);
    if let Some(n_x) = n_x {
        let c = grid[y][n_x];
        if c == b'.' && !visited.contains(&(y, n_x)) {
            n.push((y, n_x))
        }
    }
    let n_x = x + 1;
    if n_x < grid.len() {
        let c = grid[y][n_x];
        if (c == b'.' || c == b'>') && !visited.contains(&(y, n_x)) {
            n.push((y, n_x))
        }
    }

    let n_y = y.checked_sub(1);
    if let Some(n_y) = n_y {
        let c = grid[n_y][x];
        if c == b'.' && !visited.contains(&(n_y, x)) {
            n.push((n_y, x))
        }
    }

    let n_y = y + 1;
    if n_y < grid.len() {
        let c = grid[n_y][x];
        if (c == b'.' || c == b'v') && !visited.contains(&(n_y, x)) {
            n.push((n_y, x))
        }
    }

    n
}

fn get_neighbours_climb(
    grid: &Grid,
    (y, x): (usize, usize),
    visited: &FnvHashSet<(usize, usize)>,
) -> Vec<(usize, usize)> {
    let mut n = Vec::new();
    let n_x = x.checked_sub(1);
    if let Some(n_x) = n_x {
        let c = grid[y][n_x];
        if c != b'#' && !visited.contains(&(y, n_x)) {
            n.push((y, n_x))
        }
    }
    let n_x = x + 1;
    if n_x < grid.len() {
        let c = grid[y][n_x];
        if c != b'#' && !visited.contains(&(y, n_x)) {
            n.push((y, n_x))
        }
    }

    let n_y = y.checked_sub(1);
    if let Some(n_y) = n_y {
        let c = grid[n_y][x];
        if c != b'#' && !visited.contains(&(n_y, x)) {
            n.push((n_y, x))
        }
    }

    let n_y = y + 1;
    if n_y < grid.len() {
        let c = grid[n_y][x];
        if c != b'#' && !visited.contains(&(n_y, x)) {
            n.push((n_y, x))
        }
    }

    n
}

impl PartialOrd for Field {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.2 > other.2 {
            return Some(Ordering::Less);
        }

        if self.2 == other.2 {
            return Some(Ordering::Equal);
        }

        return Some(Ordering::Greater);
    }
}

impl Ord for Field {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.2 > other.2 {
            return Ordering::Less;
        }

        if self.2 == other.2 {
            return Ordering::Equal;
        }

        return Ordering::Greater;
    }
}
