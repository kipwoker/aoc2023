#![allow(unused_variables)]

use std::cmp::max;
use std::collections::{HashSet, VecDeque};
use crate::core::{parse_to_char_matrix, Solution, Direction};
use crate::core::Direction::{Left, Right, Up, Down};

pub struct Day16 {}


#[derive(Clone)]
struct Cell {
    directions: HashSet<Direction>
}

impl Solution for Day16 {
    fn get_day(&self) -> &'static str {
        "16"
    }

    fn solve1(&self, input: String) -> String {
        let matrix = parse_to_char_matrix(input.as_str());
        let paths = traverse(&matrix, (0, 0, Right));
        let count = count_passed(&paths);

        count.to_string()
    }
    fn solve2(&self, input: String) -> String {
        let matrix = parse_to_char_matrix(input.as_str());
        let n1 = matrix.len() - 1;

        let intervals = [
            (0..=n1, 0..=0, Right),
            (0..=n1, n1..=n1, Left),
            (0..=0, 0..=n1, Down),
            (n1..=n1, 0..=n1, Up)
        ];

        let mut max_count = 0;

        for (y_range, x_range, start_direction) in &intervals {
            for y in y_range.clone() {
                for x in x_range.clone() {
                    let paths = traverse(&matrix, (y, x, start_direction.clone()));
                    let count = count_passed(&paths);
                    max_count = max(max_count, count);
                }
            }
        }

        max_count.to_string()
    }
}

fn count_passed(paths: &Vec<Vec<Cell>>) -> usize {
    let mut result = 0;
    for row in paths {
        for cell in row {
            if !cell.directions.is_empty() {
                result += 1;
            }
        }
    }

    result
}

fn traverse(matrix: &Vec<Vec<char>>, start: (usize, usize, Direction)) -> Vec<Vec<Cell>> {
    let n = matrix.len();
    let mut result= vec![vec![Cell {directions: HashSet::new()}; n]; n];

    let mut q = VecDeque::new();

    q.push_back(start);

    while let Some(cursor) = q.pop_front() {
        let y = cursor.0;
        let x = cursor.1;
        let current_direction = cursor.2;

        let value = &matrix[y][x];

        let next_directions = get_next_directions(value, current_direction);
        for next_direction in next_directions {
            let inserted = result[y][x].directions.insert(next_direction);
            if inserted {
                if let Some(next_cell) = get_next_cell(y, x, next_direction, n) {
                    q.push_back((next_cell.0, next_cell.1, next_direction));
                }
            }
        }
    }

    result
}

fn get_next_directions(cell: &char, current_direction: Direction) -> Vec<Direction> {
    match cell {
        &'.' => vec![current_direction],
        &'/' => {
            let next_direction = match current_direction {
                Up => Right,
                Right => Up,
                Down => Left,
                Left => Down
            };
            vec![next_direction]
        },
        &'\\' => {
            let next_direction = match current_direction {
                Up => Left,
                Right => Down,
                Down => Right,
                Left => Up
            };
            vec![next_direction]
        },
        &'-' => {
            if current_direction == Up || current_direction == Down {
                vec![Left, Right]
            } else {
                vec![current_direction]
            }
        }
        &'|' => {
            if current_direction == Left || current_direction == Right {
                vec![Down, Up]
            } else {
                vec![current_direction]
            }
        }
        &_ => {panic!("Value unhandled: {cell}")}
    }
}

fn get_next_cell(y: usize, x: usize, direction: Direction, max_len: usize) -> Option<(usize, usize)> {
    let max_idx = max_len - 1;
    match direction {
        Up => if y == 0 {None} else {Some((y - 1, x))}
        Down => if y == max_idx {None} else {Some((y + 1, x))}

        Left => if x == 0 {None} else {Some((y, x - 1))}
        Right => if x == max_idx {None} else {Some((y, x + 1))}
    }
}