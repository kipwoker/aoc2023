#![allow(unused_variables)]

use crate::core::{parse_i32, Solution};

pub struct Day09 {}

impl Solution for Day09 {
    fn get_day(&self) -> &'static str {
        "09"
    }

    fn solve1(&self, input: String) -> String {
        let input = parse(input.as_str());
        let result: i32 = input.iter().map(|x| predict_next(x)).sum();

        result.to_string()
    }
    fn solve2(&self, input: String) -> String {
        let input = parse(input.as_str());
        let result: i32 = input.iter().map(|x| predict_prev(x)).sum();

        result.to_string()
    }
}

fn predict_next(input: &Vec<i32>) -> i32 {
    let mut steps = vec![input.clone()];
    loop {
        let cursor = steps.last().unwrap();
        let next = reduce(cursor);
        if next.iter().all(|x| *x == 0) {
            let mut result = 0;
            for step in steps.iter().rev() {
                result += step.last().unwrap();
            }

            break result;
        }

        steps.push(next);
    }
}

fn predict_prev(input: &Vec<i32>) -> i32 {
    let mut steps = vec![input.clone()];
    loop {
        let cursor = steps.last().unwrap();
        let next = reduce(cursor);
        if next.iter().all(|x| *x == 0) {
            let mut result = 0;
            for step in steps.iter().rev() {
                result = step.first().unwrap() - result;
            }

            break result;
        }

        steps.push(next);
    }
}

fn reduce(input: &Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();
    for i in 1..input.len() {
        result.push(input[i] - input[i - 1]);
    }

    result
}

fn parse(input: &str) -> Vec<Vec<i32>>{
    input
        .split("\n")
        .map(|line|
            line
                .split_whitespace()
                .map(|x| parse_i32(x)).collect()
        )
        .collect()
}