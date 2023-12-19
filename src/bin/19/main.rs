#![feature(test)]

use std::collections::{BTreeSet, HashMap};

type Solution = u64;

type Part = [(Solution, Solution); 4];
pub type ParseOutput = (HashMap<&'static str, Node>, Vec<Part>);

#[derive(Debug, Clone)]
pub struct Edge {
    prop_i: Option<usize>,
    is_greater: bool,
    num: Solution,
    destination: &'static str,
}
#[derive(Debug, Clone)]
pub struct Node {
    edges: Vec<Edge>,
    parts: BTreeSet<Part>,
}
const MAIN_INPUT: &str = include_str!("main_input");
const TEST_INPUT: &str = include_str!("test_input");

fn part_1(out: &ParseOutput) -> Solution {
    let (mut graph, mut parts) = out.clone();
    while let Some(p) = parts.pop() {
        let mut current_node = "in";
        while current_node != "R" && current_node != "A" {
            let node = graph.get_mut(current_node).unwrap();
            for c in &node.edges {
                if let Some(destination) = match c.prop_i {
                    Some(prop_i) => {
                        if (c.is_greater && p[prop_i].0 > c.num)
                            || (!c.is_greater && p[prop_i].0 < c.num)
                        {
                            Some(c.destination)
                        } else {
                            None
                        }
                    }
                    None => Some(c.destination),
                } {
                    current_node = destination;
                    node.parts.remove(&p);
                    graph.get_mut(destination).unwrap().parts.insert(p);
                    break;
                }
            }
        }
    }
    graph
        .get("A")
        .unwrap()
        .parts
        .iter()
        .map(|p| p.iter().map(|s| s.0).sum::<Solution>())
        .sum()
}

fn part_2(out: &mut ParseOutput) -> Solution {
    let (mut graph, mut parts) = out.clone();
    graph
        .get_mut("in")
        .unwrap()
        .parts
        .insert([(1, 4000), (1, 4000), (1, 4000), (1, 4000)]);
    while let Some(mut node_name) = graph
        .iter()
        .filter(|(name, _)| name != &&"R" && name != &&"A")
        .find_map(|(current_node, n)| {
            if !n.parts.is_empty() {
                Some(*current_node)
            } else {
                None
            }
        })
    {
        let node = graph.get_mut(node_name).unwrap();
        let mut p = node.parts.pop_first().unwrap();
        for c in &node.edges.clone() {
            if let Some(prop_i) = c.prop_i {
                if (p[prop_i].0 + 1..p[prop_i].1).contains(&c.num) {
                    let mut accepted_p = p.clone();
                    if c.is_greater {
                        accepted_p[prop_i] = (c.num + 1, p[prop_i].1);
                        p[prop_i] = (p[prop_i].0, c.num);
                    } else {
                        accepted_p[prop_i] = (p[prop_i].0, c.num - 1);
                        p[prop_i] = (c.num, p[prop_i].1);
                    };
                    graph
                        .get_mut(c.destination)
                        .unwrap()
                        .parts
                        .insert(accepted_p);
                }
            } else {
                graph.get_mut(c.destination).unwrap().parts.insert(p);
            }
        }
    }

    graph
        .get("A")
        .unwrap()
        .parts
        .iter()
        .map(|p| p.iter().map(|s| s.1 - s.0 + 1).product::<Solution>())
        .sum()
}

pub fn parse(file: &'static str) -> ParseOutput {
    let mut graph = HashMap::new();
    let mut parts_vec = Vec::new();
    let (nodes, parts) = file.split_once("\n\n").unwrap();
    for n in nodes.split("\n") {
        let (node_name, instructions) = n.split_once("{").unwrap();
        let mut node = Node {
            edges: Vec::new(),
            parts: BTreeSet::new(),
        };
        let ins = instructions.split(",");
        for i in ins {
            if let Some((condition, destination)) = i.split_once(":") {
                if let Some((prop, num)) = condition.split_once(">").or(condition.split_once("<")) {
                    node.edges.push(Edge {
                        is_greater: condition.contains('>'),
                        prop_i: Some(match prop {
                            "x" => 0,
                            "m" => 1,
                            "a" => 2,
                            "s" => 3,
                            _ => panic!(),
                        }),
                        num: num.parse().unwrap(),
                        destination: destination.into(),
                    });
                }
            } else {
                node.edges.push(Edge {
                    is_greater: false,
                    num: 0,
                    prop_i: None,
                    destination: i.trim_end_matches("}"),
                });
            }
        }
        graph.insert(node_name, node);
    }
    graph.insert(
        "A",
        Node {
            parts: BTreeSet::new(),
            edges: Vec::new(),
        },
    );
    graph.insert(
        "R",
        Node {
            parts: BTreeSet::new(),
            edges: Vec::new(),
        },
    );

    let mut p2 = parts.split("\n");
    for p3 in p2 {
        let mut part = [(0, 0); 4];
        for (p_i, p) in p3.split(",").enumerate() {
            let p1 = p.to_string().replace("{", "").replace("}", "");
            let n = p1[2..].parse().unwrap();
            part[p_i] = (n, n);
        }
        parts_vec.push(part);
    }

    (graph, parts_vec)
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
        assert_eq!(part_1(&parse_output), 19114);
    }

    #[test]
    pub fn test_part_2() {
        let parse_output = &mut parse(TEST_INPUT);
        assert_eq!(part_2(parse_output), 167409079868000);
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
            assert_eq!(part_1(black_box(&parse_output)), 495298);
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
