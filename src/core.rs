#![allow(dead_code)]
use serde::{Deserialize, Serialize};
use std::cmp::Ordering::Greater;
use std::collections::HashMap;
use std::fmt::Display;
use std::{fs, ptr};
use std::hash::Hash;

pub trait Solution {
    fn get_day(&self) -> &'static str;
    fn solve1(&self, input: String) -> String;
    fn solve2(&self, input: String) -> String;
}

#[derive(Debug)]
pub(crate) struct Cell1 {
    pub(crate) index: usize,
    pub(crate) value: i32,
}

#[derive(Debug)]
pub(crate) struct Cell2<TIndex, TValue> {
    pub(crate) index: Point<TIndex>,
    pub(crate) value: TValue,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub(crate) struct Point<T> {
    pub(crate) x: T,
    pub(crate) y: T,
}

pub(crate) fn parse_to_char_matrix(input: &str) -> Vec<Vec<char>> {
    input.split("\n").map(|line: &str| {
        line.chars().collect()
    }).collect()
}

fn create_cell<T>(point: Point<usize>, chars: Vec<&T>) -> Cell2<usize, Vec<&T>> {
    let x = point.x - chars.len();
    let point = Point { x, y: point.y };
    Cell2 {
        index: point,
        value: chars
    }
}

pub(crate) fn find<T, F1>(matrix: &Vec<Vec<T>>, matcher: F1) -> Vec<Cell2<usize, Vec<&T>>>
    where F1: Fn(Point<usize>, &T) -> bool
{
    let mut result = Vec::new();

    for (y, row) in matrix.iter().enumerate() {
        let mut buffer = Vec::new();
        for (x, cell) in row.iter().enumerate() {
            let point = Point { x, y };
            if matcher(point, cell) {
                buffer.push(cell);
            } else if !buffer.is_empty() {
                result.push(create_cell(point, buffer.clone()));
                buffer.clear();
            }
        }

        if !buffer.is_empty() {
            let point = Point { x: row.len(), y };
            result.push(create_cell(point, buffer.clone()));
            buffer.clear();
        }
    }

    result
}

pub(crate) fn parse_i32(input: &str) -> i32 {
    if let Ok(output) = i32::from_str_radix(input, 10) {
        output
    } else {
        println!("Cannot parse {input}");
        -1000000
    }
}

pub(crate) fn parse_i64(input: &str) -> i64 {
    if let Ok(output) = i64::from_str_radix(input, 10) {
        output
    } else {
        println!("Cannot parse {input}");
        -1000000
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub(crate) struct Interval<T> {
    pub(crate) left: T,
    pub(crate) right: T
}

impl<T: PartialOrd + Copy> Interval<T> {
    pub fn merge(intervals: &Vec<Interval<T>>) -> Vec<Interval<T>> {
        if intervals.is_empty() {
            return vec![];
        }

        let mut intervals = intervals.clone();
        intervals.sort_by(|a, b| a.left.partial_cmp(&b.left).unwrap());

        let mut merged_intervals = vec![intervals[0]];

        for i in 1..intervals.len() {
            let current_interval = intervals[i];
            let last_merged = merged_intervals.last_mut().unwrap();

            if current_interval.left <= last_merged.right {
                let result= last_merged.right.partial_cmp(&current_interval.right).unwrap();
                last_merged.right = if result == Greater { last_merged.right } else { current_interval.right };
            } else {
                merged_intervals.push(current_interval);
            }
        }

        merged_intervals
    }
}

pub(crate) trait Grouping<T> {
    fn group_by<F, K>(&self, key: F) -> HashMap<K, Vec<T>>
        where
            F: Fn(&T) -> K,
            K: Eq + Hash;
}

impl<T> Grouping<T> for Vec<T> where T: Clone {
    fn group_by<F, K>(&self, key: F) -> HashMap<K, Vec<T>>
        where F: Fn(&T) -> K, K: Eq + Hash
    {
        let mut map: HashMap<K, Vec<T>> = HashMap::new();
        for item in self {
            let k = key(&item);
            map.entry(k).or_insert_with(Vec::new).push(item.clone());
        }
        map
    }
}

pub(crate) fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

pub(crate) fn gcd_of_vector(numbers: &[i64]) -> Option<i64> {
    if numbers.is_empty() {
        None
    } else {
        let mut result = numbers[0];
        for &num in numbers.iter().skip(1) {
            result = gcd(result, num);
        }
        Some(result)
    }
}

pub(crate) fn lcm(a: i64, b: i64) -> i64 {
    let gcd_val = gcd(a, b);
    if gcd_val != 0 {
        (a * b) / gcd_val
    } else {
        0
    }
}

pub(crate) fn lcm_of_vector(numbers: &[i64]) -> Option<i64> {
    if numbers.is_empty() {
        None
    } else {
        let mut result = numbers[0];
        for &num in numbers.iter().skip(1) {
            result = lcm(result, num);
        }
        Some(result)
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct Cell {
    pub(crate) color: String,
    pub(crate) content: String
}

#[derive(Serialize, Deserialize)]
pub(crate) enum Event {
    SetMatrix(Vec<Vec<Cell>>),
    ChangeColor(usize, usize, String)
}

#[derive(Serialize, Deserialize)]
pub(crate) struct EventLog {
    events: Vec<Event>
}

impl EventLog {
    pub(crate) fn new() -> EventLog {
        EventLog {
            events: Vec::new()
        }
    }

    pub(crate) fn dump_to_file(&self, path: &str) {
        let val = serde_json::to_string(&self.events).unwrap();
        fs::write(path, val).unwrap();
    }

    pub(crate) fn append(&mut self, event: Event) {
        self.events.push(event)
    }
}

pub(crate) fn transpose<T>(v: &Vec<Vec<T>>) -> Vec<Vec<T>>
where T : Clone {
    let n = v.len();
    let m = v[0].len();

    let mut result = Vec::new();
    for j in 0..m  {
        let mut row = Vec::new();
        for i in 0..n {
            row.push(v[i][j].clone());
        }
        result.push(row);
    }

    result
}

pub(crate) fn transpose_in_place<T>(v: &mut Vec<Vec<T>>)
where T : Clone {
    let n = v.len();
    let m = v[0].len();
    if m != n {
        panic!("Matrix is not square {n} != {m}");
    }

    for i in 0..n  {
        for j in (i+1)..n {
            swap(v, i, j);
        }
    }
}

fn swap<T>(v: &mut Vec<Vec<T>>, i: usize, j: usize) {
    unsafe {
        let pointer_a: *mut T = &mut v[i][j];
        let pointer_b: *mut T = &mut v[j][i];
        ptr::swap(pointer_a, pointer_b);
    }
}

pub(crate) fn revert_rows<T>(v: &mut Vec<Vec<T>>) {
    for row in v {
        row.reverse()
    }
}

pub(crate) fn print_matrix<T>(v: &Vec<Vec<T>>) where T:Display {
    for row in v {
        for col in row {
            print!("{} ", col);
        }
        println!();
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
pub(crate) enum Direction {
    Up,
    Right,
    Down,
    Left,
}