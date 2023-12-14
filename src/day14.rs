#![allow(unused_variables)]

use crate::core::{parse_to_char_matrix, print_matrix, revert_rows, Solution, transpose_in_place};

pub struct Day14 {}

impl Solution for Day14 {
    fn get_day(&self) -> &'static str {
        "14"
    }

    fn solve1(&self, input: String) -> String {
        let mut matrix = parse_to_char_matrix(input.as_str());
        transpose_in_place(&mut matrix);
        //print_matrix(&matrix);
        tilt(&mut matrix);
        transpose_in_place(&mut matrix);
        let result = calc_load(&matrix);

        result.to_string()
    }
    fn solve2(&self, input: String) -> String {
        let mut matrix = parse_to_char_matrix(input.as_str());
        for i in 0..500 {
            // if i % 1000 == 0 {
            //     println!("Progress {i}");
            // }
            cycle(&mut matrix);
            let result = calc_load(&matrix);
            println!("Cycle {i} = {result}");
            //print_matrix(&matrix);
        }

        let result = calc_load(&matrix);

        result.to_string()
    }
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
            //print!("{a} {i} ");
            if row[i] != '.' {
                //println!("-> next");
            } else {
                for j in (i + 1)..n {
                    let b = row[j];
                    //print!("-> {b} {j} ");
                    if row[j] == '#' {
                        i = j;
                        //print!("-> end with #");
                        break;
                    }

                    if row[j] == 'O' {
                        row[i] = 'O';
                        row[j] = '.';
                        //print!("-> swap {i} {j}");
                        break;
                    }
                }
            }

            if i == n - 1 {
                break;
            }
            i += 1;
        }

        //println!();
    }
}