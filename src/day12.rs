#![allow(unused_variables)]

use std::collections::{HashSet, VecDeque};
use crate::core::Solution;

pub struct Day12 {}

struct Row {
    key: String,
    damaged_groups: Vec<usize>,
}

impl Solution for Day12 {
    fn get_day(&self) -> &'static str {
        "12"
    }

    fn solve1(&self, input: String) -> String {
        let rows = parse(input.as_str());
        let sum = rows.iter().map(|row| search(&row)).sum::<i32>();

        sum.to_string()
    }
    fn solve2(&self, input: String) -> String {
        String::new()
    }
}

fn search(row: &Row) -> i32 {
    let cache: HashSet<String> = HashSet::new();

    let mut q = VecDeque::new();
    q.push_back(row.key.clone());

    let mut counter = 0;

    'ww: while let Some(key) = q.pop_front() {
        let mut i = 0;
        let mut fill = 0;
        let mut group = row.damaged_groups[i];
        let mut has_question = false;
        for (index, c) in key.chars().enumerate() {
            match c {
                '.' => {
                    if fill == group {
                        i = i + 1;
                        group = row.damaged_groups[i];
                    }
                    else if fill > 0 {
                        continue 'ww
                    }
                }
                '#' => {
                    fill += 1;
                }
                '?' => {
                    has_question = true;
                    for x in ['.', '#'] {
                        let mut chars: Vec<char> = key.chars().clone().collect();
                        *chars.get_mut(index).unwrap() = x;
                        let option: String = chars.into_iter().collect();
                        q.push_back(option);
                    }
                }
                _ => { panic!("Unknown {c}") }
            }
        }

        if fill == group && !has_question {
            counter += 1;
        }
    }

    counter
}

fn parse(input: &str) -> Vec<Row> {
    input
        .split("\n")
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let key = parts[0].to_string();
            let p1: Vec<&str> = parts[1]
                .split(",").collect();
            let damaged_groups =
                p1.iter()
                .map(|x| {
                    i32::from_str_radix(x, 10).unwrap()
                })
                .map(|x| x as usize)
                .collect();

            Row {
                key,
                damaged_groups,
            }
        })
        .collect()
}