#![allow(unused_variables)]

use std::collections::HashMap;
use crate::core::Solution;

pub struct Day15 {}

impl Solution for Day15 {
    fn get_day(&self) -> &'static str {
        "15"
    }

    fn solve1(&self, input: String) -> String {
        let sum = input.split(",").map(|x| hash(&x.chars().collect())).sum::<i64>();

        sum.to_string()
    }
    fn solve2(&self, input: String) -> String {
        let commands = input.split(",").collect::<Vec<&str>>();
        let mut boxes = vec![Vec::new(); 256];
        let mut map = HashMap::new();
        for command in commands {
            if command.ends_with('-') {
                let label = command.trim_end_matches(|c| c == '-');
                let box_number = get_box_number(label);
                let key = (box_number, label);
                if let Some(index) = map.get(&key) {
                    println!("Remove {label} from box {box_number} at {index}");
                    boxes[box_number][*index] = '!';
                    map.remove(&key);
                }
            } else {
                let parts = command.split('=').collect::<Vec<&str>>();
                let label = parts[0];
                let focal_length: Vec<char> = parts[1].chars().collect();
                let focal_length = focal_length[0];
                let box_number = get_box_number(label);

                let key = (box_number, label);
                if let Some(index) = map.get(&key) {
                    boxes[box_number][*index] = focal_length;
                    println!("Change {label} in box {box_number} at {index}: {focal_length}");
                } else {
                    boxes[box_number].push(focal_length);
                    let index = boxes[box_number].len() - 1;
                    map.insert(key, index);
                    println!("Insert {label} in box {box_number} at {index}: {focal_length}");
                }
            }
        }


        let mut result = 0i64;

        for (box_idx, b) in boxes.iter().enumerate() {
            let bb: Vec<&char> = b.iter().filter(|x| **x != '!').collect();
            for (idx, focal_length) in bb.iter().enumerate() {
                let num = (((*focal_length).clone() as u8) - ('1' as u8) + 1) as i64;
                let a = box_idx as i64 + 1;
                let b = idx as i64 + 1;
                let mul = a * b * num;
                println!("{a} * {b} * {num} = {mul}");
                result += mul;
            }
        }

        result.to_string()
    }
}

fn get_box_number(label: &str) -> usize {
    hash(&label.chars().collect()) as usize
}

fn hash(v: &Vec<char>) -> i64 {
    let mut result: i64 = 0;
    for c in v {
        let code = c.clone() as u8;
        result += code as i64;
        result *= 17;
        result %= 256;
    }

    result
}