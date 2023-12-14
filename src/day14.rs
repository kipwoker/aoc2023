#![allow(unused_variables)]

use std::collections::HashMap;
use crate::core::{parse_to_char_matrix, revert_rows, Solution, transpose_in_place};

pub struct Day14 {}

impl Solution for Day14 {
    fn get_day(&self) -> &'static str {
        "14"
    }

    fn solve1(&self, input: String) -> String {
        let mut matrix = parse_to_char_matrix(input.as_str());

        //north
        transpose_in_place(&mut matrix);
        tilt(&mut matrix);

        //backtracking
        transpose_in_place(&mut matrix);

        let result = calc_load(&matrix);

        result.to_string()
    }
    fn solve2(&self, input: String) -> String {
        let mut cache = HashMap::new();
        let mut matrix = parse_to_char_matrix(input.as_str());
        let big_boss = 1000000000;
        for i in 0..big_boss {
            cycle(&mut matrix);
            let key = calc_hash(&matrix);
            if let Some(index) = cache.get(&key) {
                let length = i - index;
                let shift = (big_boss - i) % length - 1;
                for k in 0..shift {
                    cycle(&mut matrix);
                }
                break;
            } else {
                cache.insert(key, i);
            }
        }

        let result = calc_load(&matrix);

        result.to_string()
    }
}

fn calc_hash(matrix: &Vec<Vec<char>>) -> String {
    let mut result = String::new();
    for row in matrix {
        for cell in row {
            result.push(cell.clone());
        }
    }

    result
}

fn cycle(matrix: &mut Vec<Vec<char>>) {
    //north
    transpose_in_place(matrix);
    tilt(matrix);

    //west
    transpose_in_place(matrix);
    tilt(matrix);

    //south
    transpose_in_place(matrix);
    revert_rows(matrix);
    tilt(matrix);

    //east
    transpose_in_place(matrix);
    revert_rows(matrix);
    tilt(matrix);

    //backtracking
    revert_rows(matrix);
    transpose_in_place(matrix);
    revert_rows(matrix);
    transpose_in_place(matrix);
}

fn calc_load(matrix: &Vec<Vec<char>>) -> usize {
    let mut result = 0;
    let n= matrix.len();
    for (i, row) in matrix.iter().enumerate() {
        let mul = n - i;
        let count = row.iter().filter(|x| **x == 'O').count();
        result += mul * count;
    }

    result
}

fn tilt(matrix: &mut Vec<Vec<char>>) {
    for row in matrix {
        let mut i = 0;
        let n = row.len();
        loop {
            let a = row[i];
            if row[i] == '.' {
                for j in (i + 1)..n {
                    let b = row[j];
                    if row[j] == '#' {
                        i = j;
                        break;
                    }

                    if row[j] == 'O' {
                        row[i] = 'O';
                        row[j] = '.';
                        break;
                    }
                }
            }

            if i == n - 1 {
                break;
            }
            i += 1;
        }
    }
}