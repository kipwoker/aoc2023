#![allow(unused_variables)]

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use Direction::{Down, Left, Up};
use crate::core::{Direction, parse_to_char_matrix, Solution};
use crate::core::Direction::Right;

pub struct Day17 {}

impl Solution for Day17 {
    fn get_day(&self) -> &'static str {
        "17"
    }

    fn solve1(&self, input: String) -> String {
        solve(input.as_str(), 1, 3)
    }
    fn solve2(&self, input: String) -> String {
        solve(input.as_str(), 4, 10)
    }
}

fn solve(input: &str, min_step: usize, max_step: usize) -> String {
    let matrix = parse_to_char_matrix(input);
    let matrix: Vec<Vec<u32>> = matrix.iter()
        .map(|row| row.iter().map(|x| x.to_digit(10).unwrap()).collect())
        .collect();

    let n = matrix.len();
    let start = State { cost: 0, vertex: (0, 0), direction: Down };
    let distance_cache = find_path(&matrix, &start, min_step, max_step);

    let result = distance_cache[n - 1][n - 1].iter().min().unwrap();

    result.to_string()
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct State {
    cost: u32,
    vertex: (usize, usize),
    direction: Direction,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


fn find_path(g: &Vec<Vec<u32>>, start: &State, min_step: usize, max_step: usize) -> Vec<Vec<Vec<u32>>> {
    let n = g.len();
    let mut distance: Vec<Vec<Vec<u32>>> = vec![vec![vec![u32::MAX; 4]; n]; n];
    let mut pq = BinaryHeap::new();

    distance[start.vertex.0][start.vertex.1][get_index(&start.direction)] = start.cost;
    pq.push(start.clone());

    while let Some(state) = pq.pop() {
        let cost = &state.cost;
        let vertex = &state.vertex;
        if cost > &distance[vertex.0][vertex.1][get_index(&state.direction)] {
            continue;
        }

        let steps = get_steps(g, &distance, &state, min_step, max_step);
        for step in steps {
            let point = &step.vertex;
            let next_cost = &step.cost;
            let direction_idx = get_index(&step.direction);
            let dist = &distance[point.0][point.1][direction_idx];
            if next_cost < dist {
                distance[point.0][point.1][direction_idx] = next_cost.clone();
                pq.push(step);
            }
        }
    }

    distance
}

fn get_index(direction: &Direction) -> usize {
    match direction {
        Up => {0}
        Right => {1}
        Down => {2}
        Left => {3}
    }
}

fn get_steps(g: &Vec<Vec<u32>>, d: &Vec<Vec<Vec<u32>>>, state: &State, min_step: usize, max_step: usize) -> Vec<State> {
    match state.direction {
        Up | Down => {
            [
                get_next(&state.direction, &Left, g, d, state.vertex, (0, -1), min_step, max_step),
                get_next(&state.direction, &Right, g, d, state.vertex, (0, 1), min_step, max_step)
            ].concat()
        }
        Right | Left => {
            [
                get_next(&state.direction, &Up, g, d, state.vertex, (-1, 0), min_step, max_step),
                get_next(&state.direction, &Down, g, d, state.vertex, (1, 0), min_step, max_step)
            ].concat()
        }
    }
}

fn get_next(
    from: &Direction,
    to: &Direction,
    g: &Vec<Vec<u32>>,
    d: &Vec<Vec<Vec<u32>>>,
    current: (usize, usize),
    diff: (i32, i32),
    min_step: usize,
    max_step: usize
) -> Vec<State> {
    let mut result = Vec::new();

    let n = g.len() as i32;
    let mut cost = d[current.0][current.1][get_index(from)].clone();

    for i in 1..=max_step {
        let i = i as i32;
        let shift = (current.0 as i32 + diff.0 * i, current.1 as i32 + diff.1 * i);
        if shift.0 < 0 || shift.0 >= n {
            continue;
        }
        if shift.1 < 0 || shift.1 >= n {
            continue;
        }

        let y = shift.0 as usize;
        let x = shift.1 as usize;

        cost += g[y][x].clone();

        let i = i as usize;
        if i >= min_step {
            result.push(State {
                cost,
                vertex: (y, x),
                direction: to.clone()
            });
        }
    }

    result
}