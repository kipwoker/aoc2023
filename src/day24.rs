#![allow(unused_variables)]

use crate::core::{parse_i64, Solution};

const EPS: f64 = 0.0001;

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
                    if la.satisfy(&intersection_point) &&
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
        let hailstones = parse(input.as_str());
        let result = bruteforce(&hailstones, 300);

        result.to_string()
    }
}

#[derive(Debug)]
struct Result {
    time1: f64,
    time2: f64,
    point: Point,
}

impl Result {
    fn is_infinite(&self) -> bool {
        self.time1.is_infinite() ||
            self.time2.is_infinite() ||
            self.point.x.is_infinite() ||
            self.point.y.is_infinite() ||
            self.point.z.is_infinite()
    }

    fn is_valid(&self) -> bool {
        !self.is_infinite() &&
        self.time1 >= 0.0 && self.time2 >= 0.0 &&
        self.is_round()
    }

    fn get_value(&self) -> usize {
        self.point.x as usize + self.point.y as usize + self.point.z as usize
    }
}

trait IsRound {
    fn is_round(&self) -> bool;
}


impl IsRound for f64 {
    fn is_round(&self) -> bool {
        (self.round() - self).abs() < EPS
    }
}

impl IsRound for Result {
    fn is_round(&self) -> bool {
        self.time1.is_round() ||
            self.time2.is_round() ||
            self.point.x.is_round() ||
            self.point.y.is_round() ||
            self.point.z.is_round()
    }
}


fn bruteforce(hailstones: &Vec<Hailstone>, range: i32) -> usize {
    let n = hailstones.len();

    for i in 0..n {
        let a = &hailstones[i];
        for j in (i + 1)..n {
            let b = &hailstones[j];

            for vx in -range..range {
                for vy in -range..range {
                    for vz in -range..range {
                        let vx = vx as f64;
                        let vy = vy as f64;
                        let vz = vz as f64;

                        if let Some(result) = try_this(a, b, vx, vy, vz) {
                            if !result.is_valid() {
                                continue;
                            }

                            if !compare(hailstones, &result, vx, vy, vz, j + 1) {
                                continue;
                            }

                            println!("{} {} {}", vx, vy, vz);
                            println!("{:?}", result);
                            return result.get_value();
                        }
                    }
                }
            }
        }
    }
    unreachable!()
}

fn is_zero(t: f64, pa: f64, va: f64, pb: f64, vb: f64) -> bool {
    (pa + t * (va - vb) - pb).abs() > EPS
}

fn compare(hailstones: &Vec<Hailstone>, result: &Result, vx: f64, vy: f64, vz: f64, start_idx: usize) -> bool {
    let n = hailstones.len();
    for i in start_idx..n {
        let h = &hailstones[i];

        let pxi = h.start.x;
        let pyi = h.start.y;
        let pzi = h.start.z;
        let vxi = h.velocity.x;
        let vyi = h.velocity.y;
        let vzi = h.velocity.z;

        let ti = (pxi - result.point.x) / (vx - vxi);

        if is_zero(ti, result.point.y, vy, pyi, vyi) || is_zero(ti, result.point.z, vz, pzi, vzi)
        {
            return false;
        }
    }

    true
}


fn try_this(a: &Hailstone, b: &Hailstone, vx: f64, vy: f64, vz: f64) -> Option<Result> {
    let px1 = a.start.x;
    let py1 = a.start.y;
    let pz1 = a.start.z;
    let vx1 = a.velocity.x;
    let vy1 = a.velocity.y;
    let vz1 = a.velocity.z;

    let px2 = b.start.x;
    let py2 = b.start.y;
    let pz2 = b.start.z;
    let vx2 = b.velocity.x;
    let vy2 = b.velocity.y;
    let vz2 = b.velocity.z;

    let dvy = vy1 - vy;
    let dvx = vx1 - vx;
    let dvz = vz1 - vz;
    let dvx2 = vx2 - vx;
    let dp = px2 - px1;

    let t_up = py2 - py1 - dvy * dp / dvx;
    let t_down = vy - vy2 + dvy * dvx2 / dvx;

    let t2 = t_up / t_down;
    let t1 = (dp + t2 * dvx2) / dvx;

    let px = px1 + t1 * dvx;
    let py = py1 + t1 * dvy;
    let pz = pz1 + t1 * dvz;

    if is_zero(t2, pz, vz, pz2, vz2) {
        None
    } else {
        let result = Result {
            time1: t1,
            time2: t2,
            point: Point {
                x: px,
                y: py,
                z: pz,
            },
        };
        Some(result)
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
