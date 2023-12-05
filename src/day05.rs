use std::collections::HashMap;
use crate::core::{parse_i64, Solution};

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
                return direction.to + shift
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
        // let maps = input.maps;
        // println!("{maps:?}");
        let mut min_value: i64 = i64::MAX;
        for seed in input.seeds {
            let mut cursor_value = seed;
            let mut cursor_key = "seed";
            let value = loop {
                let (destination, range_map) = input.maps.get(cursor_key).unwrap();
                let value = range_map.get(cursor_value);
                //println!("{cursor_key} -> {destination} :: {cursor_value} -> {value}");
                cursor_key = destination;
                cursor_value = value;
                if cursor_key == "location" {
                    break cursor_value;
                }
            };

            //println!("Value {value}");
            if value < min_value {
                min_value = value;
            }
        }

        min_value.to_string()
    }
    fn solve2(&self, _input: String) -> String {
        String::new()
    }
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
        //println!("{line}");
        if *line == "\n" || *line == "" {
            if !range_map.map.is_empty() {
                let cd = current_direction.clone();
                let mut range = range_map.map.clone();
                range.sort_by(|(a,_), (b, _)| a.from.cmp(&b.from));
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
        //println!("Parts {parts:?}");
        let key = parts[1];
        let value = parts[0];
        let range = parts[2];

        range_map.map.push((Direction { from: key, to: value }, range));
    }

    if !range_map.map.is_empty() {
        let cd = current_direction.clone();
        let mut range = range_map.map.clone();
        range.sort_by(|(a,_), (b, _)| a.from.cmp(&b.from));
        maps.insert(cd.from, (cd.to, RangeMap { map: range }));
    }

    Input { seeds, maps }
}