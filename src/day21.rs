#![allow(unused_variables)]

use std::cell::RefCell;
use std::collections::HashSet;
use crate::core::{BFS, Matrix, parse_to_char_matrix, Solution};

pub struct Day21 {}

impl Solution for Day21 {
    fn get_day(&self) -> &'static str {
        "21"
    }

    fn solve1(&self, input: String) -> String {
        let mut matrix = parse_to_char_matrix(input.as_str());
        let start = matrix.find(|(y, x)| matrix[*y][*x] == 'S').unwrap();
        matrix[start.0][start.1] = '.';
        let start = (start.0, start.1, 0);

        let target = 64;
        let result = RefCell::new(HashSet::new());

        matrix.bfs(start, |state| {
            let state = state.clone();
            if state.2 == target {
                let _ = result.borrow_mut().insert(state);
                vec![]
            } else {
                let point = (state.0, state.1);
                let next = matrix.get_neighbors_4(&point);
                next.iter()
                    .filter(|p| matrix[(**p).0][(**p).1] == '.')
                    .map(|(y, x)| (y.clone(), x.clone(), state.2 + 1))
                    .collect()
            }
        });

        let result = result.borrow();
        result.len().to_string()
    }
    fn solve2(&self, input: String) -> String {
        let mut matrix = parse_to_char_matrix(input.as_str());
        let start = matrix.find(|(y, x)| matrix[*y][*x] == 'S').unwrap();
        matrix[start.0][start.1] = '.';

        let n = matrix.len() as i32;

        let mut cursor = HashSet::new();
        cursor.insert((start.0 as i32, start.1 as i32));

        let target = 26501365;
        let mut y = Vec::new();
        let mut outside = false;
        let mut x1 = 0i32;

        let mut prev_sum = 0;
        let mut i = 0i32;

        let direction = 1;
        let mut c = 0;
        let points_limit = 2;
        loop {
            let sum = cursor.len();

            if outside {
                if abs(i) % n == 0 {
                    y.push(sum as i32);
                    let x = y.len();
                    println!("x = {x} -> i = {i} -> y = {sum}");
                }
            }

            if y.len() == points_limit {
                break;
            }

            let mut next_q = HashSet::new();

            for point in cursor {
                let next = matrix.get_neighbors_4_infinity(&point);
                for p in next {
                    if !outside && leave(&n, &p) {
                        let prev = i - direction;
                        println!("x = 0 -> i = {prev} -> y = {prev_sum}");
                        println!("x = 1 -> i = {i} -> y = {sum}");

                        x1 = i;
                        i = 0;
                        c = prev_sum;

                        y.push(sum as i32);
                        outside = true;
                    }

                    let y = modulo(&p.0, &n);
                    let x = modulo(&p.1, &n);
                    if matrix[y][x] == '.' {
                        next_q.insert(p.clone());
                    }
                }
            }

            cursor = next_q;
            i += direction;
            prev_sum = sum;
        }

        println!("y: {y:?}");

        let x = &[1, 2];

        let x02 = x[0] * x[0];
        let x12 = x[1] * x[1];
        let c = c as i32;
        let b = (x12 * (y[0] - c) - x02 * (y[1] - c)) / (x[0] * x[1] * (x[1] - x[0]));
        let a = (y[0] - c - b * x[0]) / x02;

        println!("{a} x^2 + {b} x + {c}");

        let x = ((target - x1) / n + 1) as i64;
        println!("x = {x}");
        let a = a as i64;
        let b = b as i64;
        let c = c as i64;
        let result = a * x * x + b * x + c;

        result.to_string()
    }
}

fn leave(n: &i32, p: &(i32, i32)) -> bool {
    p.0 < 0 || p.0 >= *n || p.1 < 0 || p.1 >= *n
}

fn modulo(a: &i32, b: &i32) -> usize {
    let bb = *b;
    let c = *a % bb;
    if c >= 0 {
        c as usize
    } else {
        ((c + bb) % bb) as usize
    }
}

fn abs(x: i32) -> i32 {
    if x < 0 { -x } else { x }
}