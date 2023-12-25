#![feature(test)]

use fnv::{FnvHashMap, FnvHashSet};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

type Solution = usize;
pub type Node = (&'static str, Vec<NodeId>);
pub type ParseOutput = Vec<Node>;
pub type NodeId = usize;
const MAIN_INPUT: &str = include_str!("main_input");
const TEST_INPUT: &str = include_str!("test_input");

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Field(NodeId, Solution);

fn part_1(graph: &ParseOutput, edges_to_remove: &[(Solution, Solution); 3]) -> Solution {
    let mut g = graph.clone();
    for (start_node_id, end_node_id) in edges_to_remove {
        g[*start_node_id].1 = g[*start_node_id]
            .1
            .iter()
            .filter(|node_id| *node_id != end_node_id)
            .map(|n| *n)
            .collect();
        g[*end_node_id].1 = g[*end_node_id]
            .1
            .iter()
            .filter(|node_id| *node_id != start_node_id)
            .map(|n| *n)
            .collect();
    }
    get_nodes_in_graph(&g, edges_to_remove[0].0) * get_nodes_in_graph(&g, edges_to_remove[0].1)
}

fn get_nodes_in_graph(graph: &ParseOutput, start: NodeId) -> Solution {
    let mut current_fields: BinaryHeap<Field> = BinaryHeap::new();
    let mut visited = FnvHashSet::<NodeId>::default();
    current_fields.push(Field(start, 0));
    while let Some(Field(node_id, path_length)) = current_fields.pop() {
        visited.insert(node_id);
        for n in &graph[node_id].1 {
            if !visited.contains(n) {
                current_fields.push(Field(*n, path_length + 1))
            }
        }
    }
    visited.len()
}

fn part_2(measures: &mut ParseOutput) -> Solution {
    0
}

pub fn parse(file: &'static str) -> ParseOutput {
    let mut nodes = Vec::new();
    let mut module_name_to_id_map: FnvHashMap<&'static str, NodeId> = FnvHashMap::default();
    let lines = file.lines().filter(|l| !l.is_empty());
    let mut module_id: NodeId = 0;
    for l in lines.clone() {
        let (main_module_name, connected_modules) = l.split_once(": ").unwrap();
        if !module_name_to_id_map.contains_key(main_module_name) {
            module_name_to_id_map.insert(main_module_name, module_id);
            println!(
                "{{\"caption\": \"{}\", \"id\": {}}},",
                main_module_name, module_id
            );
            nodes.push((main_module_name, Vec::new()));
            module_id += 1;
        }
        for module_name in connected_modules.split(" ") {
            if !module_name_to_id_map.contains_key(module_name) {
                println!(
                    "{{\"caption\": \"{}\", \"id\": {}}},",
                    module_name, module_id
                );
                module_name_to_id_map.insert(module_name, module_id);
                nodes.push((module_name, Vec::new()));
                module_id += 1;
            }
        }
    }
    for l in lines {
        let (module_name, connected_modules) = l.split_once(": ").unwrap();
        let main_node_id = module_name_to_id_map.get(module_name).unwrap();
        for module_name in connected_modules.split(" ") {
            let node_id = module_name_to_id_map.get(module_name).unwrap();
            println!(
                "{{\"source\": \"{}\", \"target\": {}}},",
                main_node_id, node_id
            );
            let connected_node = nodes.get_mut(*node_id).unwrap();
            connected_node.1.push(*main_node_id);
        }
        let node_id = module_name_to_id_map.get(module_name).unwrap();
        let main_node = nodes.get_mut(*node_id).unwrap();
        for module_name in connected_modules.split(" ") {
            let node_id = module_name_to_id_map.get(module_name).unwrap();
            main_node.1.push(*node_id);
        }
    }

    nodes
}
fn main() {
    let parse_output = &mut parse(MAIN_INPUT);
    let edges_to_remove = [(980, 294), (1020, 587), (479, 477)];
    println!(
        "Solution to part 1 is {}",
        part_1(parse_output, &edges_to_remove)
    );
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
        let edges_to_remove = [(8, 6), (12, 9), (3, 0)];
        assert_eq!(part_1(&parse_output, &edges_to_remove), 54);
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
        let edges_to_remove = [(980, 284), (1020, 587), (479, 477)];
        b.iter(move || {
            assert_eq!(
                part_1(black_box(&parse_output), black_box(&edges_to_remove)),
                1798691765
            );
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

impl PartialOrd for Field {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.1 > other.1 {
            return Some(Ordering::Less);
        }

        if self.1 == other.1 {
            return Some(Ordering::Equal);
        }

        return Some(Ordering::Greater);
    }
}

impl Ord for Field {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.1 > other.1 {
            return Ordering::Less;
        }

        if self.1 == other.1 {
            return Ordering::Equal;
        }

        return Ordering::Greater;
    }
}

/*fn part_1_old(graph: &ParseOutput) -> Solution {
    let mut in_between_count: FnvHashMap<(&'static str, &'static str), Solution> =
        FnvHashMap::default();
    let v: Vec<bool> = vec![false; graph.len()];
    let mut visited: Vec<bool> = v.clone();
    let mut current_fields: BinaryHeap<Field> = BinaryHeap::new();
    let mut predecessor_map: FnvHashMap<NodeId, NodeId> = FnvHashMap::default();
    let mut max = graph.len() * graph.len();
    let mut current = max;
    for start_node_id in 0..graph.len() {
        for end_node_id in 0..graph.len() {
            println!("{}", current);
            current -= 1;
            if start_node_id == end_node_id {
                continue;
            }
            current_fields.push(Field(start_node_id, 0));
            while let Some(Field(node_id, path_length)) = current_fields.pop() {
                visited[node_id] = true;
                if node_id == end_node_id {
                    break;
                }
                for n in &graph[node_id].1 {
                    if !visited[*n] {
                        predecessor_map.insert(*n, node_id);
                        current_fields.push(Field(*n, path_length + 1))
                    }
                }
            }
            let mut current = end_node_id;
            while let Some(&predecessor) = predecessor_map.get(&current) {
                *in_between_count
                    .entry((graph[current].0, graph[predecessor].0))
                    .or_insert(0) += 1;
                *in_between_count
                    .entry((graph[predecessor].0, graph[current].0))
                    .or_insert(0) += 1;
                current = predecessor;
                if current == start_node_id {
                    break;
                }
            }
            visited = v.clone();
            current_fields.clear();
            predecessor_map.clear();
        }
    }
    let mut edge_significance = in_between_count
        .iter()
        .map(|(a, b)| (*b, *a))
        .collect::<Vec<(Solution, (&'static str, &'static str))>>();
    edge_significance.sort();
    println!("{:?}", edge_significance);
    0
}
*/
