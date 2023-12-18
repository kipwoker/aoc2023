#![allow(unused_variables)]

use std::cmp::min;
use std::collections::HashSet;
use crate::core::{Direction, parse_i32, print_matrix, Solution};
use crate::core::Direction::{Down, Left, Right, Up};

pub struct Day18 {}

impl Solution for Day18 {
    fn get_day(&self) -> &'static str {
        "18"
    }

    fn solve1(&self, input: String) -> String {
        let steps = parse(input.as_str());
        let points = get_points(&steps);
        let matrix = get_matrix(&points);
        print_matrix(&matrix.data);

        String::new()
    }
    fn solve2(&self, input: String) -> String {
        String::new()
    }
}

struct DigStep {
    direction: Direction,
    count: i32,
    color: String
}

struct Matrix {
    start: (usize, usize),
    data: Vec<Vec<bool>>
}

fn get_points(steps: &Vec<DigStep>) -> HashSet<(i32, i32)> {
    let mut result = HashSet::new();
    result.insert((0,0));

    let mut cursor = (0,0);
    for step in steps {
        let (dy,dx) = step.direction.get_matrix_offset();
        for i in 1..=step.count {
            let y = cursor.0 + dy;
            let x = cursor.1 + dx;
            cursor = (y, x);
            result.insert(cursor.clone());
        }
    }

    result
}

fn get_matrix(points: &HashSet<(i32, i32)>) -> Matrix {
    let mut max_y = i32::MIN;
    let mut max_x = i32::MIN;
    let mut min_y = i32::MAX;
    let mut min_x = i32::MAX;
    for point in points {
        if point.0 > max_y {
            max_y = point.0.clone();
        }
        if point.0 < min_y {
            min_y = point.0.clone();
        }
        if point.1 > max_x {
            max_x = point.1.clone();
        }
        if point.1 < min_x {
            min_x = point.1.clone();
        }
    }

    let shift = min(min_y, min_x);
    max_x -= shift;
    max_y -= shift;

    let mut result = vec![vec![false; max_x as usize]; max_y as usize];
    for point in points {
        let y = (point.0 - shift) as usize;
        let x = (point.1 - shift) as usize;
        result[y][x] = true;
    }

    Matrix{
        start: (-shift as usize, -shift as usize),
        data: result
    }
}


fn parse(input: &str) -> Vec<DigStep> {
    input.split("\n").map(|line| {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let direction = match parts[0] {
            "L" => {Left}
            "R" => {Right}
            "U" => {Up}
            "D" => {Down}
            &_ => {panic!("Unhandled {}", parts[0])}
        };

        let count = parse_i32(parts[1]);
        let color = parts[2].trim_matches(|x| x == '(' || x == ')');
        let color = String::from(color);

        DigStep {
            direction,
            count,
            color
        }
    }).collect()
}