#![allow(unused_variables)]

use crate::core::{parse_i32, Solution};

pub struct Day12 {}


#[derive(Debug, Clone)]
struct Row {
    key: Vec<char>,
    damaged_groups: Vec<usize>,
}

impl Solution for Day12 {
    fn get_day(&self) -> &'static str {
        "12"
    }

    fn solve1(&self, input: String) -> String {
        let rows = parse(input.as_str());
        let sum = rows.iter()
            .map(|row| search(&row))
            .sum::<i64>();

        sum.to_string()
    }
    fn solve2(&self, input: String) -> String {
        let rows = parse(input.as_str());
        let sum = rows.iter()
            .map(|row| unfold(&row, 5))
            .map(|row| search(&row))
            .sum::<i64>();

        sum.to_string()
    }
}

fn unfold(row: &Row, multiplier: usize) -> Row {
    let mut row = row.clone();
    row.key.push('?');
    let mut key = row.key.repeat(multiplier);
    key.pop();

    Row {
        key,
        damaged_groups: row.damaged_groups.repeat(multiplier),
    }
}

fn search(row: &Row) -> i64 {
    let n = row.key.len();
    let m = row.damaged_groups.len();

    let mut cache: Vec<Vec<i64>> = vec![vec![0; m + 1]; n + 1];

    cache[0][0] = 1;
    for i in 0..n {
        let c = row.key[i];
        for j in 0..=m {
            let cursor = cache[i][j];
            if c != '#' {
                cache[i + 1][j] += cursor;
            }

            if j == m {
                continue;
            }

            let expected = row.damaged_groups[j];
            let rest = n - i;
            if expected > rest {
                continue;
            }

            let left = i;
            let right = i + expected;
            let key = &row.key;
            if key[left..right].contains(&'.') {
                continue;
            }

            if n == right {
                cache[right][j + 1] += cursor;
            } else if key[right] != '#' {
                cache[right + 1][j + 1] += cursor;
            }
        }
    }

    let result = cache[n][m];
    //println!("Row {result}");
    result
}

fn parse(input: &str) -> Vec<Row> {
    input
        .split("\n")
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let key = parts[0].chars().collect();
            let damaged_groups = parts[1]
                .split(",")
                .map(|x| parse_i32(x))
                .map(|x| x as usize)
                .collect();

            Row {
                key,
                damaged_groups,
            }
        })
        .collect()
}