#![allow(unused_variables)]

use std::collections::HashSet;
use crate::core::{parse_i32, Solution};

pub struct Day04 {}


#[derive(Debug)]
struct Card {
    wins: HashSet<i32>,
    nums: Vec<i32>
}

impl Solution for Day04 {
    fn get_day(&self) -> &'static str {
        "04"
    }

    fn solve1(&self, input: String) -> String {
        let cards = parse(input);

        let mut result = 0;
        for card in cards {
            let win_rate = card.nums.iter()
                .filter(|num|{ card.wins.contains(num) })
                .count() as i32;

            let score = if win_rate == 0 { 0 } else { 1 << (win_rate - 1) };
            result += score;
        }

        result.to_string()
    }
    fn solve2(&self, input: String) -> String {
        let cards = parse(input);

        let mut counts: Vec<i32> = cards.iter().map(|_| {1}).collect();
        for (index, card) in cards.iter().enumerate() {
            let win_rate = card.nums.iter()
                .filter(|num|{ card.wins.contains(num) })
                .count();

            let count = counts[index];
            for i in (index + 1)..(index + 1 + win_rate) {
                counts[i] += count;
            }
        }

        let result: i32 = counts.iter().sum();

        result.to_string()
    }
}

fn parse(input: String) -> Vec<Card> {
    input.split("\n").map(|line| {
        let parts: Vec<&str> = line.split(":").collect();
        let body = parts[1];
        let parts: Vec<&str> = body.split("|").collect();
        let wins: HashSet<i32> = parts[0]
            .split_whitespace()
            .map(|x| { parse_i32(x)})
            .collect();
        let nums: Vec<i32> = parts[1]
            .split_whitespace()
            .map(|x| { parse_i32(x)})
            .collect();

        Card {
            wins,
            nums
        }
    }).collect()
}