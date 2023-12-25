#![allow(unused_variables)]

use std::collections::{HashMap, HashSet, VecDeque};
use crate::core::Solution;

pub struct Day25 {}

impl Solution for Day25 {
    fn get_day(&self) -> &'static str {
        "25"
    }

    fn solve1(&self, input: String) -> String {
        let g = parse(input.as_str());

        let result = solve(&g);

        result.to_string()
    }
    fn solve2(&self, input: String) -> String {
        String::new()
    }
}

fn parse(input: &str) -> Graph {
    let lines: Vec<_> = input.split("\n").collect();

    let mut connections = HashMap::new();

    for line in lines {
        let parts: Vec<_> = line.split_whitespace().collect();

        let key = &parts[0][..3];
        let key_nodes = connections.entry(key).or_insert(HashSet::new());

        let nodes = &parts[1..];

        for &node in nodes {
            key_nodes.insert(node);
        }

        for &node in nodes {
            let entry = connections.entry(node).or_insert(HashSet::new());
            entry.insert(key);
        }
    }

    Graph {
        connections
    }
}

struct Graph<'a> {
    connections: HashMap<&'a str, HashSet<&'a str>>
}

fn solve(g: &Graph) -> usize {
    let mut cache = HashMap::new();

    for &start in g.connections.keys() {
        let mut next_q = VecDeque::new();
        let mut visited = HashSet::new();

        next_q.push_back(start);
        visited.insert(start);

        while let Some(node) = next_q.pop_front() {
            for &next in g.connections.get(node).unwrap() {
                if visited.insert(next) {
                    let key = if node < next { (node, next) } else { (next, node) };

                    cache.entry(key).and_modify(|v| *v = *v + 1).or_insert(1);

                    next_q.push_back(next);
                }
            }
        }
    }

    let mut sorted: Vec<_> = cache.iter().collect();
    sorted.sort_unstable_by_key(|e| e.1);
    sorted.reverse();

    let to_remove: HashSet<_> = sorted.iter().take(3).map(|p| (p.0.0, p.0.1)).collect();
    let start = *g.connections.keys().next().unwrap();
    let mut first = 1;

    let mut next_q = VecDeque::new();
    let mut visited = HashSet::new();
    next_q.push_back(start);
    visited.insert(start);

    while let Some(node) = next_q.pop_front() {
        for &next in &g.connections[node] {
            let key = if node < next { (node, next) } else { (next, node) };

            if to_remove.contains(&key) {
                continue;
            }

            if visited.insert(next) {
                first += 1;
                next_q.push_back(next);
            }
        }
    }

    let second = g.connections.len() - first;

    first * second
}

