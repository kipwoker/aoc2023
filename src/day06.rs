use crate::core::{parse_i64, Solution};

pub struct Day06 {}

impl Solution for Day06 {
    fn get_day(&self) -> &'static str {
        "06"
    }

    fn solve1(&self, input: String) -> String {
        let lines: Vec<&str> = input.split("\n").collect();
        let times: Vec<i64> = parse_nums(lines[0]);
        let distances: Vec<i64> = parse_nums(lines[1]);

        let total = calculate(times, distances);

        total.to_string()
    }
    fn solve2(&self, input: String) -> String {
        let lines: Vec<&str> = input.split("\n").collect();
        let times: Vec<i64> = parse_nums2(lines[0]);
        let distances: Vec<i64> = parse_nums2(lines[1]);

        let total = calculate(times, distances);

        total.to_string()
    }
}

fn parse_nums(line: &str) -> Vec<i64>{
    line.split(":").nth(1).unwrap().split_whitespace().map(|x| parse_i64(x)).collect()
}

fn parse_nums2(line: &str) -> Vec<i64>{
    let str = line.split(":").nth(1).unwrap().replace(" ", "");
    let num = parse_i64(str.as_str());
    let vec = vec![num];
    vec
}

fn has_fraction(num: f64) -> bool {
    num != num.floor()
}

fn calculate(times: Vec<i64>, distances: Vec<i64>) -> i64 {
    let n= times.len();

    let mut total = 1;
    for i in 0..n {
        let t = times[i];
        let d = distances[i];
        let discriminant = t * t - 4 * d;
        let discriminant = (discriminant as f64).sqrt();

        let t = t as f64;
        let left: f64 = (t - discriminant) / 2.0;
        let right: f64 = (t + discriminant) / 2.0;

        let left = if has_fraction(left) { left.ceil() as i64 } else { (left + 1.0) as i64 };
        let right = if has_fraction(right) { right.floor() as i64 } else { (right - 1.0) as i64 };

        let count = right - left + 1;

        total *= count;
    }

    total
}