#![allow(unused_variables)]

use std::cmp::{max, min};
use crate::core::{parse_to_char_matrix, Solution};

pub struct Day11 {}

impl Solution for Day11 {
    fn get_day(&self) -> &'static str {
        "11"
    }

    fn solve1(&self, input: String) -> String {
        let matrix = parse_to_char_matrix(input.as_str());
        let (ext_rows, ext_cols) = find_extensions(&matrix);
        let coords = find_coords(&matrix);
        let result = calc_distance(&coords, &ext_rows, &ext_cols, 2 - 1);
        let sum: i64 = result.iter().sum();

        sum.to_string()
    }
    fn solve2(&self, input: String) -> String {
        let matrix = parse_to_char_matrix(input.as_str());
        let (ext_rows, ext_cols) = find_extensions(&matrix);
        let coords = find_coords(&matrix);
        let result = calc_distance(&coords, &ext_rows, &ext_cols, 1000000 - 1);
        let sum: i64 = result.iter().sum();

        sum.to_string()
    }
}

fn abs(a: i32) -> i32 {
    if a < 0 { -a } else { a }
}

fn calc_distance(points: &Vec<(usize, usize)>, ext_rows: &Vec<usize>, ext_cols: &Vec<usize>, multiplier: i64) -> Vec<i64> {
    let n = points.len();
    let mut result = Vec::new();
    for i in 0..n {
        let a = points[i];
        for j in (i + 1)..n {
            let b = points[j];
            let a0 = a.0 as i32;
            let a1 = a.1 as i32;
            let b0 = b.0 as i32;
            let b1 = b.1 as i32;

            let s = abs(a0 - b0) + abs(a1 - b1);
            //println!("Original [{a0}; {a1}] -> [{b0}; {b1}] == {s}");

            let max_y = max(a.0, b.0);
            let min_y = min(a.0, b.0);
            let max_x = max(a.1, b.1);
            let min_x = min(a.1, b.1);

            let y_count = ext_rows.iter().filter(|y| min_y <= **y && **y <= max_y).count();
            let x_count = ext_cols.iter().filter(|x| min_x <= **x && **x <= max_x).count();
            let y_count = (y_count as i64) * multiplier;
            let x_count = (x_count as i64) * multiplier;
            let s1: i64 = (s as i64) + y_count + x_count;
            //println!("Extended [{a0}; {a1}] -> [{b0}; {b1}] == {s} + {y_count} + {x_count} == {s1}");
            result.push(s1);
        }
    }

    result
}

fn find_extensions(matrix: &Vec<Vec<char>>) -> (Vec<usize>, Vec<usize>) {
    let n = matrix.len();
    let m = matrix[0].len();

    let mut extension_cols = Vec::new();
    let mut extension_rows = Vec::new();

    for i in 0..n {
        if matrix[i].iter().all(|x| *x == '.') {
            extension_rows.push(i);
        }
    }

    for j in 0..m {
        let mut flag = false;
        for i in 0..n {
            if matrix[i][j] == '#' {
                flag = true;
            }
        }

        if !flag {
            extension_cols.push(j);
        }
    }

    println!("Cols {extension_cols:?}");
    println!("Rows {extension_rows:?}");

    (extension_rows, extension_cols)
}

fn find_coords(matrix: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut result = Vec::new();

    for (i, row) in matrix.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if *cell == '#' {
                result.push((i, j));
            }
        }
    }

    result
}