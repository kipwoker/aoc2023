#![allow(unused_variables)]

use crate::core::{parse_i64, Solution};

pub struct Day24 {}

impl Solution for Day24 {
    fn get_day(&self) -> &'static str {
        "24"
    }

    fn solve1(&self, input: String) -> String {
        let hailstones = parse(input.as_str());
        //println!("{hailstones:?}");

        let n = hailstones.len();
        let zipped: Vec<(&Hailstone, Func2D, Limit)> = hailstones.iter().map(|h| (h, get_func_2d(h), get_limits(h))).collect();

        let hard_min = 200000000000000.0;
        let hard_max = 400000000000000.0;

        let hard_limit = Limit {
            min_x: Some(hard_min),
            min_y: Some(hard_min),
            max_x: Some(hard_max),
            max_y: Some(hard_max),
        };

        let mut counter = 0i64;
        for i in 0..n {
            let (_, fa, la) = &zipped[i];
            for j in (i + 1)..n {
                let (_, fb, lb) = &zipped[j];
                if let Some(intersection_point) = find_intersection_point_2d(fa, fb) {
                    if  la.satisfy(&intersection_point) &&
                        lb.satisfy(&intersection_point) &&
                        hard_limit.satisfy(&intersection_point)
                    {
                        counter += 1;
                    }
                }
            }
        }

        counter.to_string()
    }
    fn solve2(&self, input: String) -> String {
        String::new()
    }
}

#[derive(Debug, Clone)]
struct Func2D {
    k: f64,
    b: f64,
}

#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Debug, Clone)]
struct Hailstone {
    start: Point,
    velocity: Point,
}

#[derive(Debug, Clone)]
struct Limit {
    min_x: Option<f64>,
    min_y: Option<f64>,
    max_x: Option<f64>,
    max_y: Option<f64>,
}

impl Limit {
    fn satisfy(&self, point: &Point) -> bool {
        if let Some(x) = self.max_x {
            if point.x > x {
                return false;
            }
        }
        if let Some(x) = self.min_x {
            if point.x < x {
                return false;
            }
        }
        if let Some(y) = self.max_y {
            if point.y > y {
                return false;
            }
        }
        if let Some(y) = self.min_y {
            if point.y < y {
                return false;
            }
        }

        true
    }
}

fn get_limits(h: &Hailstone) -> Limit {
    let (min_x, max_x) = if h.velocity.x > 0.0 { (Some(h.start.x), None) } else { (None, Some(h.start.x)) };
    let (min_y, max_y) = if h.velocity.y > 0.0 { (Some(h.start.y), None) } else { (None, Some(h.start.y)) };

    return Limit {
        min_x,
        min_y,
        max_x,
        max_y,
    };
}

fn find_intersection_point_2d(f1: &Func2D, f2: &Func2D) -> Option<Point> {
    let dk = f1.k - f2.k;
    if f1.k == f2.k {
        return None;
    }

    let db = f2.b - f1.b;

    let x = db / dk;
    let y = f1.k * x + f1.b;

    Some(Point { x, y, z: -30000.0 })
}

fn get_func_2d(h: &Hailstone) -> Func2D {
    let k = h.velocity.y / h.velocity.x;
    let b = h.start.y - h.start.x * k;

    return Func2D { k, b };
}

fn parse(input: &str) -> Vec<Hailstone> {
    input.split("\n").map(|line| {
        let parts: Vec<Vec<f64>> = line.split(" @ ")
            .map(|part| part.split(", ")
                .map(|x| x.trim())
                .map(parse_i64)
                .map(|x| x as f64)
                .collect()
            ).collect();
        let start = Point {
            x: parts[0][0],
            y: parts[0][1],
            z: parts[0][2],
        };
        let velocity = Point {
            x: parts[1][0],
            y: parts[1][1],
            z: parts[1][2],
        };

        Hailstone {
            start,
            velocity,
        }
    }).collect()
}
