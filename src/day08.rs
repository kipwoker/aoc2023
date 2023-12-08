#![allow(unused_variables)]

use std::collections::HashMap;
use crate::core::{lcm_of_vector, Solution};
use crate::day08::Direction::{Left, Right};

pub struct Day08 {}

#[derive(PartialEq)]
enum Direction {
    Left,
    Right
}

#[derive(Debug)]
struct Node {
    left: String,
    right: String
}


impl Solution for Day08 {
    fn get_day(&self) -> &'static str {
        "08"
    }

    fn solve1(&self, input: String) -> String {
        let (steps, map) = parse(input.as_str());
        let result = traverse(&steps, &map, "AAA", |x| x == "ZZZ");

        result.to_string()
    }
    fn solve2(&self, input: String) -> String {
        let (steps, map) = parse(input.as_str());
        let result = traverse2(&steps, &map);

        result.to_string()
    }
}

fn traverse<F>(steps: &Vec<Direction>, map: &HashMap<&str, Node>, start: &str, end: F) -> i32
where
    F: Fn(&str) -> bool
{
    let mut cursor = start;
    let mut index = 0;
    let mut counter = 0;
    loop {
        if end(cursor) {
            println!("Finish on {cursor}");
            break counter;
        }

        let step = &steps[index];
        let node = map.get(cursor).unwrap();
        if *step == Left {
            cursor = node.left.as_str();
        } else {
            cursor = node.right.as_str();
        }
        counter += 1;

        index += 1;
        if index >= steps.len() {
            index = 0;
        }
    }
}

fn traverse2(steps: &Vec<Direction>, map: &HashMap<&str, Node>) -> i64 {
    let cursors: Vec<&str> = map.keys()
        .filter(|x| x.chars().nth(2).unwrap() == 'A')
        .map(|x| *x)
        .collect();
    let finishes: Vec<&str> = map.keys()
        .filter(|x| x.chars().nth(2).unwrap() == 'Z')
        .map(|x| *x)
        .collect();
    println!("Starts: {cursors:?}");
    println!("Finishes: {finishes:?}");

    let mut results = Vec::new();
    for cursor in cursors {
        println!("Search for {cursor}");
        let result = traverse(&steps, &map, cursor, |x| x.chars().nth(2).unwrap() == 'Z');
        println!("Result {result}");
        results.push(result as i64);
    }

    let lcm = lcm_of_vector(&results).unwrap();

    lcm
}

fn parse(input: &str) -> (Vec<Direction>, HashMap<&str, Node>) {
    let lines: Vec<&str> = input.split("\n").collect();
    let steps: Vec<Direction> = lines[0].chars().map(|c| if c == 'L' {Left} else {Right}).collect();

    let map: HashMap<&str, Node> = lines.iter().skip(2).map(|x| {
        let parts: Vec<&str> = x.split(" = ").collect();
        let key = parts[0];

        let brackets: &[_] = &['(', ')'];
        let parts: Vec<&str> = parts[1].trim_matches(brackets).split(", ").collect();
        let node = Node {
            left: parts[0].to_string(),
            right: parts[1].to_string()
        };
        if node.left.len() != 3 || node.right.len() != 3
        {
            println!("Parse error: {x} -> {node:?}");
        }

        (key, node)
    }).collect();

    (steps, map)
}