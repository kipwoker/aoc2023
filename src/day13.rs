#![allow(unused_variables)]

use crate::core::{Solution, transpose};

pub struct Day13 {}

impl Solution for Day13 {
    fn get_day(&self) -> &'static str {
        "13"
    }

    fn solve1(&self, input: String) -> String {
        solve(input.as_str(), 0)
    }
    fn solve2(&self, input: String) -> String {
        solve(input.as_str(), 1)
    }
}

fn solve(input: &str, expected_diff: usize) -> String {
    let matrices = parse(input);

    let result: i64 = matrices.iter().enumerate().map(|(idx, matrix)| {
        let mut result = 0usize;
        if let Some(row_index) = find_mirror_row_index(matrix, expected_diff) {
            let row = row_index + 1;
            println!("Row {idx}: {row}");
            result += row * 100;
        }
        let transposed = transpose(matrix);
        if let Some(col_index) = find_mirror_row_index(&transposed, expected_diff) {
            let col = col_index + 1;
            println!("Col {idx}: {col}");
            result += col;
        }
        result as i64
    }).sum();

    result.to_string()
}

fn count_difference(a: &Vec<char>, b: &Vec<char>) -> usize {
    let mut result = 0;
    for i in 0..a.len() {
        if a[i] != b[i] {
            result += 1;
        }
    }

    result
}

fn find_mirror_row_index(matrix: &Vec<Vec<char>>, expected_diff: usize) -> Option<usize> {
    let mut cursor = 0;
    loop {
        let (candidate, result) = find_mirror_row_index_from(matrix, cursor, expected_diff);
        if result.is_some() {
            return result;
        }

        if candidate.is_none() {
            return candidate;
        }

        cursor = candidate.unwrap() + 1;
    }
}

fn find_mirror_row_index_from(matrix: &Vec<Vec<char>>, start: usize, expected_diff: usize) -> (Option<usize>, Option<usize>) {
    let mut cursor = &matrix[start];

    let mut mirror_index = None;
    let n = matrix.len();
    for i in (start + 1)..n {
        let row = &matrix[i];
        let diff = count_difference(cursor, row);
        if diff <= expected_diff {
            mirror_index = Some(i - 1);
            break
        }

        cursor = row;
    }

    //println!("Candidate {mirror_index:?}");
    if mirror_index == None {
        return (None, None)
    }

    let mut l = mirror_index.unwrap();
    let mut r = l + 1;
    let mut total_diff = 0;
    loop {
        let diff = count_difference(&matrix[l], &matrix[r]);
        //println!("Compare {l} {r} {total_diff} {diff} {expected_diff}");
        total_diff += diff;
        if total_diff > expected_diff {
            break (mirror_index, None)
        }

        if l == 0 || r == (n - 1) {
            if total_diff != expected_diff {
                break (mirror_index, None)
            } else {
                break (mirror_index, mirror_index)
            }
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