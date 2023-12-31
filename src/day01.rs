use std::collections::HashMap;
use crate::{
    core::Solution,
    core::Cell1
};

pub struct Day01 {}

impl Solution for Day01 {
    fn get_day(&self) -> &'static str {
        "01"
    }
    fn solve1(&self, _input: String) -> String {
        String::new()
    }

    fn solve2(&self, input: String) -> String {
        let lines = input.split("\n");

        let map = HashMap::from([
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8),
            ("nine", 9)
        ]);

        let find = |x: &str, y: &str| x.find(y);
        let reverse_find = |x: &str, y: &str| x.rfind(y);
        let le = |x: usize, y: usize| x <= y;
        let ge = |x: usize, y: usize| x >= y;

        let mut sum = 0;
        for line in lines {
            let mut right = Cell1 { index: 0, value: -1 };
            let mut left = Cell1 { index: line.len(), value: -1 };

            for entry in map.iter() {
                let (key, value) = entry;
                left = check(line, key, value, &left, find, ge).unwrap_or(left);
                right = check(line, key, value, &right, reverse_find, le).unwrap_or(right);

                let key = value.to_string();
                let key = key.as_str();
                left = check(line, &key, value, &left, find, ge).unwrap_or(left);
                right = check(line, &key, value, &right, reverse_find, le).unwrap_or(right);
            }

            if left.value == -1 || right.value == -1 {
                println!("Unexpected {line} {left:?} {right:?}")
            }

            let num = left.value * 10 + right.value;

            sum += num;
        }

        sum.to_string()
    }
}

fn check<F1, F2>(
    line: &str,
    pattern: &&str,
    new_value: &i32,
    cell: &Cell1,
    search: F1,
    comparer: F2
) -> Option<Cell1>
    where
        F1: Fn(&str, &str) -> Option<usize>,
        F2: Fn(usize, usize) -> bool
{
    if let Some(index) = search(line, pattern) {
        if comparer(cell.index, index) {
            return Some(Cell1 { index, value: *new_value });
        }
    }
    None
}