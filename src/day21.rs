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
        let mut start = (0, 0, 0);
        for (y, row) in matrix.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if *cell == 'S' {
                    start = (y, x, 0);
                }
            }
        }

        matrix[start.0][start.1] = '.';

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
        String::new()
    }
}
