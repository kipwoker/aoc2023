#![allow(unused_variables)]

use std::cmp::min;
use std::collections::HashSet;
use crate::core::{BFS, Direction, Matrix, parse_i32, Solution};
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
        let mut result = bfs(&matrix);
        result += points.len();

        result.to_string()
    }
    fn solve2(&self, input: String) -> String {
        let steps = parse(input.as_str());
        let steps: Vec<DigStep> = steps.iter().map(unwrap).collect();
        let pp = get_pivot_points(&steps);
        let s = s_gauss(&pp);

        s.to_string()
    }
}

#[derive(Debug)]
struct DigStep {
    direction: Direction,
    count: i32,
    color: String
}

fn unwrap(step: &DigStep) -> DigStep {
    let color = step.color.trim_start_matches(|x| x == '#');
    let hex = &color[0..5];
    let count = i32::from_str_radix(hex, 16).unwrap();
    let direction = match &color.chars().nth(5).unwrap() {
        '0' => Right,
        '1' => Down,
        '2' => Left,
        '3' => Up,
        _ => unreachable!()
    };

    DigStep {
        color: step.color.clone(),
        direction,
        count
    }
}

fn s_gauss(points: &Vec<(i64, i64)>) -> i64 {
    let n = points.len();
    let mut sum: i64 = 0;
    for i in 0..n {
        let i1 = (i + 1) % n;
        let xi = points[i].1;
        let yi = points[i].0;
        let yi1 = points[i1].0;
        let xi1 = points[i1].1;
        sum += xi * yi1;
        sum -= xi1 * yi;
        sum += abs(xi - xi1) + abs(yi - yi1);
    }

    sum = abs(sum.clone());
    sum /= 2;

    sum + 1
}

fn abs(sum: i64) -> i64 {
    if sum < 0 {-sum} else {sum}
}

fn get_pivot_points(steps: &Vec<DigStep>) -> Vec<(i64, i64)> {
    let mut result = Vec::new();

    let mut cursor = (0,0);
    for step in steps {
        let (dy,dx) = step.direction.get_matrix_offset();
        let y = cursor.0 + (dy as i64) * (step.count as i64);
        let x = cursor.1 + (dx as i64) * (step.count as i64);
        cursor = (y, x);
        result.push((y + 1, x + 1));
    }

    result
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

fn get_matrix(points: &HashSet<(i32, i32)>) -> Vec<Vec<char>> {
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

    let mut result = vec![vec!['.'; (max_x + 1) as usize]; (max_y + 1) as usize];
    for point in points {
        let y = (point.0 - shift) as usize;
        let x = (point.1 - shift) as usize;
        result[y][x] = '#';
    }

    result
}

fn find_pivot_point(matrix: &Vec<Vec<char>>) -> (usize, usize) {
    let n = matrix.len();
    let m = matrix[0].len();
    for i in 0..n {
        if matrix[i][0] == '#' && matrix[i][1] == '.' {
            return (i, 1);
        }
        if matrix[i][m - 1] == '#' && matrix[i][m - 2] == '.' {
            return (i, m - 2);
        }
    }

    for j in 0..m {
        if matrix[0][j] == '#' && matrix[1][j] == '.' {
            return (1, j);
        }
        if matrix[n - 1][j] == '#' && matrix[n - 2][j] == '.' {
            return (n - 2, j);
        }
    }

    unreachable!()
}

fn bfs(matrix: &Vec<Vec<char>>) -> usize {
    let start = find_pivot_point(matrix);
    matrix.bfs(
        start,
        |point|
            matrix
                .get_neighbors_4(point)
                .iter()
                .filter(|p|  matrix[p.0][p.1] != '#')
                .map(|x| *x)
                .collect()
    ).len()
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