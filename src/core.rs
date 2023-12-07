use std::cmp::Ordering::Greater;
use std::collections::HashMap;

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

pub(crate) fn parse_to_char_matrix(input: String) -> Vec<Vec<char>> {
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

pub(crate) fn group_by<T, F, K>(vec: Vec<T>, key: F) -> HashMap<K, Vec<T>>
    where
        F: Fn(&T) -> K,
        K: Eq + std::hash::Hash,
{
    let mut map: HashMap<K, Vec<T>> = HashMap::new();
    for item in vec {
        let k = key(&item);
        map.entry(k).or_insert_with(Vec::new).push(item);
    }
    map
}