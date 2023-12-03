use std::collections::HashMap;
use crate::core::{Cell2, find, parse_to_char_matrix, Point, Solution};

pub struct Day03 {}

impl Solution for Day03 {
    fn get_day(&self) -> &'static str {
        "03"
    }

    fn solve1(&self, input: String) -> String {
        let matrix = parse_to_char_matrix(input);
        let numbers = find(&matrix, |_index, value| value.is_digit(10));
        let adjacent_numbers = filter_adjacent_numbers(&matrix, &numbers);
        let result: i64 = adjacent_numbers.iter().map(|x| *x as i64).sum();

        result.to_string()
    }
    fn solve2(&self, input: String) -> String {
        let matrix = parse_to_char_matrix(input);
        let numbers = find(&matrix, |_index, value| value.is_digit(10));
        let gear_groups = collect_gear_groups(&matrix, &numbers);
        //println!("Gear groups: {gear_groups:?}");
        let result: i64 = gear_groups.iter()
            .filter(|(_gear, values)| { values.len() == 2 })
            .map(|(_gear, values)| { (values[0] * values[1]) as i64 })
            .sum();

        result.to_string()
    }
}

fn parse_number(vec: &Vec<&char>) -> i32 {
    let str: String = vec.iter().map(|x| *x).collect();
    i32::from_str_radix(str.as_str(), 10).unwrap()
}

fn filter_adjacent_numbers(matrix: &Vec<Vec<char>>, all_nums: &Vec<Cell2<usize, Vec<&char>>>) -> Vec<i32> {
    let mut result = Vec::new();
    let rows_count = matrix.len();
    let cols_count = matrix[0].len();
    let limit = Point { x: cols_count - 1, y: rows_count - 1 };
    for cell in all_nums {
        let neighbors = get_neighbors(&limit, &cell.index, cell.value.len());
        if has_bind(matrix, &neighbors) {
            result.push(parse_number(&cell.value));
        }
    }

    result
}

fn collect_gear_groups(matrix: &Vec<Vec<char>>, all_nums: &Vec<Cell2<usize, Vec<&char>>>) -> HashMap<Point<usize>, Vec<i32>> {
    let mut result = HashMap::new();
    let rows_count = matrix.len();
    let cols_count = matrix[0].len();
    let limit = Point { x: cols_count - 1, y: rows_count - 1 };
    for cell in all_nums {
        let neighbors = get_neighbors(&limit, &cell.index, cell.value.len());
        for neighbor in neighbors {
            if matrix[neighbor.y][neighbor.x] == '*' {
                let value = parse_number(&cell.value);
                result
                    .entry(neighbor)
                    .and_modify(|v: &mut Vec<i32>| { v.push(value) })
                    .or_insert_with(|| { vec![value] });
            }
        }
    }

    result
}

fn has_bind(matrix: &Vec<Vec<char>>, points: &Vec<Point<usize>>) -> bool {
    for point in points {
        let c: char = matrix[point.y][point.x];
        if c != '.' && !c.is_digit(10) {
            return true;
        }
    }

    return false;
}

fn get_neighbors(limit: &Point<usize>, point: &Point<usize>, length: usize) -> Vec<Point<usize>> {
    let mut points = Vec::new();
    let point = Point { x: point.x as i32, y: point.y as i32 };
    let limit = Point { x: limit.x as i32, y: limit.y as i32 };
    let length = length as i32;
    //left border
    if point.x >= 0 {
        points.push(Point { x: point.x - 1, y: point.y });
        points.push(Point { x: point.x - 1, y: point.y + 1 });
        points.push(Point { x: point.x - 1, y: point.y - 1 });
    }
    // above & below
    for i in 0..length {
        points.push(Point { x: point.x + i, y: point.y + 1 });
        points.push(Point { x: point.x + i, y: point.y - 1 });
    }
    //right border
    points.push(Point { x: point.x + length, y: point.y });
    points.push(Point { x: point.x + length, y: point.y + 1 });
    points.push(Point { x: point.x + length, y: point.y - 1 });

    points.iter()
        .filter(|point|
            point.x >= 0 && point.x <= limit.x &&
            point.y >= 0 && point.y <= limit.y
        )
        .map(|p| Point {x: p.x as usize, y: p.y as usize })
        .collect()
}