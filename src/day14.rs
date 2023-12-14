#![allow(unused_variables)]

use crate::core::{parse_to_char_matrix, Solution, transpose};

pub struct Day14 {}

impl Solution for Day14 {
    fn get_day(&self) -> &'static str {
        "14"
    }

    fn solve1(&self, input: String) -> String {
        let matrix = parse_to_char_matrix(input.as_str());
        // print_matrix(&matrix);
        // println!();
        let mut matrix = transpose(&matrix);
        // print_matrix(&matrix);
        // println!();
        tilt(&mut matrix);
        //print_matrix(&matrix);
        //println!();
        // let matrix = transpose(&matrix);
        // print_matrix(&matrix);
        // println!();
        let matrix = transpose(&matrix);
        let result = calc_load(&matrix);

        result.to_string()
    }
    fn solve2(&self, input: String) -> String {
        String::new()
    }
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