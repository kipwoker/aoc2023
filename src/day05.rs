use std::cmp::{max, min};
use std::collections::HashMap;
use crate::core::{Interval, parse_i64, Solution};

pub struct Day05 {}


#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Direction<T> {
    from: T,
    to: T,
}

#[derive(Debug)]
struct RangeMap {
    map: Vec<(Direction<i64>, i64)>,
}

impl RangeMap {
    fn get(&self, key: i64) -> i64 {
        for (direction, range) in &self.map {
            let left = direction.from;
            let right = direction.from + range - 1;
            if key >= left && key <= right {
                let shift = key - left;

                let value = direction.to + shift;
                return value.clone();
            }

            if key < left {
                return key;
            }
        }

        key
    }
}

#[derive(Debug)]
struct Input<'a> {
    seeds: Vec<i64>,
    maps: HashMap<&'a str, (&'a str, RangeMap)>,
}

impl Solution for Day05 {
    fn get_day(&self) -> &'static str {
        "05"
    }

    fn solve1(&self, input: String) -> String {
        let input = parse(input.as_str());
        let mut min_value: i64 = i64::MAX;
        for seed in &input.seeds {
            let value = calculate(*seed, &input);

            if value < min_value {
                min_value = value;
            }
        }

        min_value.to_string()
    }
    fn solve2(&self, input: String) -> String {
        let input = parse(input.as_str());

        let intervals = get_intervals(&input.seeds);
        let intervals = compress(&intervals, input);
        let min_value = intervals.iter().map(|interval| interval.left).min().unwrap();

        min_value.to_string()
    }
}

fn compress(intervals: &Vec<Interval<i64>>, input: Input) -> Vec<Interval<i64>> {
    let mut source = "seed";
    let mut cursor: Vec<Interval<i64>> = intervals.clone();
    let result = loop {
        if source == "location" {
            break cursor;
        }

        let (target, range_map) = input.maps.get(source).unwrap();
        let mut state = cursor.clone();
        let mut next = Vec::new();

        for (direction, range) in &range_map.map {
            let left = direction.from;
            let right = direction.from + range;
            let diff = direction.to - direction.from;
            for interval in &cursor {
                if interval.right >= left && interval.left < right {
                    let source_left = max(interval.left, left);
                    let source_right = min(interval.right, right);
                    let target_left = diff + source_left;
                    let target_right = diff + source_right;
                    let source = Interval { left: source_left, right: source_right };
                    let target = Interval {left: target_left, right: target_right };

                    let new_state: Vec<Interval<i64>> = state.iter()
                        .flat_map(|interval| subtract(interval, &source))
                        .collect();
                    state = new_state;
                    next.push(target);
                }
            }
        }
        source = target;
        for interval in state {
            next.push(interval);
        }
        cursor = Interval::merge(&next);
    };

    result
}

fn subtract(from: &Interval<i64>, slice: &Interval<i64>) -> Vec<Interval<i64>> {
    let left = from.left;
    let right = from.right;

    let mut result = Vec::new();
    if slice.left <= left && right <= slice.right {
        return result;
    }

    if right < slice.left || left > slice.right {
        result.push(from.clone());
    } else if left < slice.left && slice.right < right {
        result.push(Interval { left, right: slice.left - 1 });
        result.push(Interval { left: slice.right + 1, right });
    } else if slice.left > left {
        result.push(Interval { left, right: slice.left - 1 });
    } else if slice.right < right {
        result.push(Interval { left: slice.right + 1, right });
    }

    result
}

fn calculate(seed: i64, input: &Input) -> i64 {
    let mut cursor_value = seed;
    let mut source = "seed";
    let value = loop {
        let (target, range_map) = input.maps.get(source).unwrap();
        let value = range_map.get(cursor_value);
        source = target;
        cursor_value = value;
        if source == "location" {
            break cursor_value;
        }
    };
    value
}

fn get_intervals(input: &Vec<i64>) -> Vec<Interval<i64>> {
    let mut result = Vec::new();
    let n = input.len() / 2;
    for i in 0..n {
        let index = i + i;
        let left = input[index];
        let right = input[index] + input[index + 1];
        let interval = Interval { left, right };
        result.push(interval);
    }

    let result = Interval::merge(&result);

    result
}


fn parse(input: &str) -> Input {
    let lines: Vec<&str> = input.split("\n").collect();
    let seeds: Vec<i64> = lines[0]
        .split(": ")
        .nth(1).unwrap()
        .split_whitespace()
        .map(|x| parse_i64(x))
        .collect();

    let mut maps = HashMap::new();
    let mut current_direction = Direction { from: "", to: "" };
    let mut range_map = RangeMap { map: Vec::new() };
    for line in lines.iter().skip(2) {
        if *line == "\n" || *line == "" {
            if !range_map.map.is_empty() {
                let cd = current_direction.clone();
                let mut range = range_map.map.clone();
                range.sort_by(|(a, _), (b, _)| a.from.cmp(&b.from));
                maps.insert(cd.from, (cd.to, RangeMap { map: range }));
            }
            continue;
        }

        if line.contains("map:") {
            let parts: Vec<&str> = line.split(" ").nth(0).unwrap().split("-to-").collect();
            current_direction = Direction { from: parts[0], to: parts[1] };
            range_map = RangeMap { map: Vec::new() };
            continue;
        }

        let parts: Vec<i64> = line.split_whitespace().map(|x| parse_i64(x)).collect();
        let key = parts[1];
        let value = parts[0];
        let range = parts[2];

        range_map.map.push((Direction { from: key, to: value }, range));
    }

    if !range_map.map.is_empty() {
        let cd = current_direction.clone();
        let mut range = range_map.map.clone();
        range.sort_by(|(a, _), (b, _)| a.from.cmp(&b.from));
        maps.insert(cd.from, (cd.to, RangeMap { map: range }));
    }

    Input { seeds, maps }
}