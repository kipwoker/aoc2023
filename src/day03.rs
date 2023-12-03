use std::collections::HashMap;
use crate::core::{Point, Solution};

pub struct Day03 {}

impl Solution for Day03 {
    fn get_day(&self) -> &'static str {
        "03"
    }

    fn solve1(&self, input: String) -> String {
        let matrix = parse(input);
        let numbers = find_numbers(&matrix);
        let adjacent_numbers = filter_adjacent_numbers(&matrix, &numbers);
        let result: i64 = adjacent_numbers.iter().map(|x| *x as i64).sum();

        result.to_string()
    }
    fn solve2(&self, input: String) -> String {
        let matrix = parse(input);
        let numbers = find_numbers(&matrix);
        let gear_groups = collect_gear_groups(&matrix, &numbers);
        //println!("Gear groups: {gear_groups:?}");
        let result: i64 = gear_groups.iter()
            .filter(|(_gear, values) | { values.len() == 2 })
            .map(|(_gear, values)| { (values[0] * values[1]) as i64 })
            .sum();

        result.to_string()
    }
}

fn parse(input: String) -> Vec<Vec<char>> {
    input.split("\n").map(|line: &str| {
        line.chars().collect()
    }).collect()
}

fn create_number(from: usize, y: usize, num: Vec<&char>) -> (Point<i32>, Vec<&char>) {
    let x = from - num.len();
    let point = Point { x: x as i32, y: y as i32 };
    (
        point,
        num
    )
}

fn find_numbers(matrix: &Vec<Vec<char>>) -> Vec<(Point<i32>, Vec<&char>)> {
    let mut result = Vec::new();

    for (y, row) in matrix.iter().enumerate() {
        let mut num = Vec::new();
        for (x, cell) in row.iter().enumerate() {
            if cell.is_digit(10) {
                num.push(cell);
            } else if !num.is_empty() {
                result.push(create_number(x, y, num.clone()));
                num.clear();
            }
        }

        if !num.is_empty() {
            result.push(create_number(row.len(), y, num.clone()));
            num.clear();
        }

    }

    result
}

fn parse_number(vec: &Vec<&char>) -> i32 {
    let str: String = vec.iter().map(|x| *x).collect();
    i32::from_str_radix(str.as_str(), 10).unwrap()
}

fn filter_adjacent_numbers(matrix: &Vec<Vec<char>>, all_nums: &Vec<(Point<i32>, Vec<&char>)>) -> Vec<i32> {
    let mut result = Vec::new();
    let rows_count = matrix.len() as i32;
    let cols_count = matrix[0].len() as i32;
    let limit = Point {x: cols_count - 1, y: rows_count - 1};
    for (point, num) in all_nums {
        let neighbors = get_neighbors(&limit, point, num.len() as i32);
        if has_bind(matrix, &neighbors) {
            result.push(parse_number(num));
        }
    }

    result
}

fn collect_gear_groups(matrix: &Vec<Vec<char>>, all_nums: &Vec<(Point<i32>, Vec<&char>)>) -> HashMap<Point<i32>, Vec<i32>> {
    let mut result = HashMap::new();
    let rows_count = matrix.len() as i32;
    let cols_count = matrix[0].len() as i32;
    let limit = Point {x: cols_count - 1, y: rows_count - 1};
    for (point, num) in all_nums {
        let neighbors = get_neighbors(&limit, point, num.len() as i32);
        for neighbor in neighbors {
            if matrix[neighbor.y as usize][neighbor.x as usize] == '*' {
                let value = parse_number(num);
                result
                    .entry(neighbor)
                    .and_modify(|v: &mut Vec<i32> | { v.push(value) })
                    .or_insert_with(|| { vec![value] });
            }
        }
    }

    result
}

fn has_bind(matrix: &Vec<Vec<char>>, points: &Vec<Point<i32>>) -> bool {
    for point in points {
        let c: char = matrix[point.y as usize][point.x as usize];
        if c != '.' && !c.is_digit(10) {
            return true
        }
    }

    return false
}

fn get_neighbors(limit: &Point<i32>, point: &Point<i32>, length: i32) -> Vec<Point<i32>> {
    let mut points = Vec::new();
    //left border
    points.push(Point {x: point.x - 1, y: point.y });
    points.push(Point {x: point.x - 1, y: point.y + 1 });
    points.push(Point {x: point.x - 1, y: point.y - 1 });
    // above & below
    for i in 0..length {
        points.push(Point {x: point.x + i, y: point.y + 1 });
        points.push(Point {x: point.x + i, y: point.y - 1 });
    }
    //right border
    points.push(Point {x: point.x + length, y: point.y });
    points.push(Point {x: point.x + length, y: point.y + 1 });
    points.push(Point {x: point.x + length, y: point.y - 1 });

    points.iter()
        .filter(|point|
            point.x >= 0 && point.x <= limit.x &&
            point.y >= 0 && point.y <= limit.y
        )
        .map(|p| *p)
        .collect()
}