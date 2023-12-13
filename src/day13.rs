#![allow(unused_variables)]

use crate::core::{Solution, transpose};

pub struct Day13 {}

impl Solution for Day13 {
    fn get_day(&self) -> &'static str {
        "13"
    }

    fn solve1(&self, input: String) -> String {
        let matrices = parse(input.as_str());

        let result: i64 = matrices.iter().map(|matrix| {
            let mut result = 0usize;
            if let Some(row_index) = find_mirror_row_index(matrix) {
                let row = row_index + 1;
                println!("Row {row}");
                result += row * 100;
            }
            let transposed = transpose(matrix);
            if let Some(col_index) = find_mirror_row_index(&transposed) {
                let col = col_index + 1;
                println!("Col {col}");
                result += col;
            }
            result as i64
        }).sum();

        result.to_string()
    }
    fn solve2(&self, input: String) -> String {
        String::new()
    }
}

fn is_equal(a: &Vec<char>, b: &Vec<char>) -> bool {
    if a.len() != b.len() {
        return false;
    }

    for i in 0..a.len() {
        if a[i] != b[i] {
            return false;
        }
    }

    true
}

fn find_mirror_row_index(matrix: &Vec<Vec<char>>) -> Option<usize> {
    let mut cursor = &matrix[0];

    let mut mirror_index = None;
    let n = matrix.len();
    for i in 1..n {
        let row = &matrix[i];
        if is_equal(cursor, row) {
            mirror_index = Some(i - 1);
            break
        }

        cursor = row;
    }

    if mirror_index == None {
        return None
    }

    let mut l = mirror_index.unwrap();
    let mut r = l + 1;
    loop {
        if !is_equal(&matrix[l], &matrix[r]) {
            break None
        }

        if l == 0 || r == (n - 1) {
            break mirror_index
        }

        l -= 1;
        r += 1;
    }
}

fn parse(input: &str) -> Vec<Vec<Vec<char>>> {
    let lines: Vec<&str> = input.split("\n").collect();
    let mut result = Vec::new();
    let mut cursor = Vec::new();
    for line in lines {
        if line.is_empty() || line == "\n" {
            result.push(cursor.clone());
            cursor = Vec::new();
            continue;
        }

        let row: Vec<char> = line.chars().collect();
        cursor.push(row);
    }

    if !cursor.is_empty() {
        result.push(cursor);
    }

    result
}