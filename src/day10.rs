#![allow(unused_variables)]

use std::collections::{HashMap, HashSet, VecDeque};
use Direction::Up;
use crate::core::{parse_to_char_matrix, Solution};
use crate::day10::Direction::{Down, Left, Right};
use colored::Colorize;

pub struct Day10 {}

#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
struct Step {
    y: usize,
    x: usize,
    start_direction: Direction,
    from_direction: Direction,
    path: Vec<(usize, usize)>,
    length: usize
}

struct Result {
    path_points: HashSet<(usize, usize)>,
    length: usize,
    start_point: HashSet<Direction>
}

struct VisitState {
    from_directions: HashSet<Direction>,
    path: Vec<(usize, usize)>
}

impl Solution for Day10 {
    fn get_day(&self) -> &'static str {
        "10"
    }

    fn solve1(&self, input: String) -> String {
        let (g, start_point) = parse(input.as_str());
        println!("{g:?}");
        let result = find_far_point_length(start_point, &g).unwrap();
        let start_point_directions = result.start_point;

        result.length.to_string()
    }
    fn solve2(&self, input: String) -> String {
        let (mut g, start_point) = parse(input.as_str());
        //println!("{g:?}");
        let result = find_far_point_length(start_point, &g).unwrap();

        g[start_point.0][start_point.1] = result.start_point.clone();

        let inner = find_inner(&g, &result);
        let count = inner.len();

        print_path(input.as_str(), &result.path_points, &inner);

        count.to_string()
    }
}

fn print_path(input: &str, points: &HashSet<(usize, usize)>, inner: &HashSet<(usize, usize)>) {
    let matrix = parse_to_char_matrix(input);
    for (y, row) in matrix.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            let key = &(y, x);
            if points.contains(key) {
                print!("{}", cell.to_string().red());
            } else if inner.contains(key) {
                print!("{}", cell.to_string().bright_green());
            } else {
                print!("{}", cell);
            }
        }
        println!();
    }
}

fn find_inner(g: &Vec<Vec<HashSet<Direction>>>, result: &Result) -> HashSet<(usize, usize)> {
    let mut inner = HashSet::new();
    for (y, row) in g.iter().enumerate() {
        for x in 0..row.len() {
            let intersections_count = count_intersections(row, y, x, result);
            if (intersections_count & 1) == 1 {
                inner.insert((y,x));
            }
        }
    }

    inner
}

fn swap(set: &HashSet<Direction>) -> HashSet<Direction> {
    set.iter().map(|x| invert_direction(x)).collect()
}

fn count_intersections(row: &Vec<HashSet<Direction>>, start_y: usize, start_x: usize, result: &Result) -> i32 {
    let mut counter = 0;
    if result.path_points.contains(&(start_y, start_x)) {
        return counter;
    }

    let mut leverage = None;
    for i in (start_x + 1)..row.len() {
        if result.path_points.contains(&(start_y, i)) {
            let cell = &row[i];
            if cell.contains(&Down) && cell.contains(&Up) {
                counter += 1;
                leverage = None;
                continue;
            }

            if (cell.contains(&Down) || cell.contains(&Up)) && cell.contains(&Right) {
                if leverage.is_none() {
                    leverage = Some(swap(cell));
                } else {
                    leverage = None;
                }
            }

            if (cell.contains(&Down) || cell.contains(&Up)) && cell.contains(&Left) {
                leverage = match leverage {
                    None => None,
                    Some(expected_directions) => {
                        if cell.eq(&&expected_directions) {
                            counter += 1;
                        }
                        None
                    }
                };
                continue;
            }
        } else {
            leverage = None;
        }
    }

    counter
}

fn find_far_point_length(start_point: (usize, usize), g: &Vec<Vec<HashSet<Direction>>>) -> Option<Result> {
    let max_x = g.len() - 1;
    let max_y = g[0].len() - 1;

    let start_step = Step {
        y: start_point.0,
        x: start_point.1,
        start_direction: Up,
        from_direction: Up,
        path: Vec::from([(start_point.0, start_point.1)]),
        length: 0
    };

    let mut q = VecDeque::new();
    q.push_back(start_step);

    let mut visited: HashMap<(usize, usize), VisitState> = HashMap::new();

    while let Some(step) = q.pop_front() {
        //print!("Visit {step:?}");
        let key = (step.y, step.x);
        if let Some(v) = visited.get(&key) {
            if v.from_directions.contains(&step.start_direction) {
                //println!(" -> already came from this direction");
                continue;
            } else {
                //println!(" -> met");
                let mut start_point = v.from_directions.clone();
                start_point.insert(step.start_direction);

                let path1 = v.path.clone();
                let path2 = step.path;

                let path_points: HashSet<(usize, usize)> = [path1, path2]
                    .concat()
                    .iter()
                    .map(|x| x.clone())
                    .collect();

                println!("S = {start_point:?}");

                let result = Result {
                    path_points,
                    length: step.length,
                    start_point
                };
                return Some(result)
            }
        }

        let directions = &g[step.y][step.x];
        if !directions.contains(&step.from_direction) {
            //println!(" -> dead end");
            continue;
        }

        if step.length == 0 {
            let state = VisitState {
                from_directions: HashSet::from([Up, Down, Left, Right]),
                path: step.path.clone()
            };
            visited.insert(key, state);
        } else {
            visited.entry(key)
                .and_modify(|v| {
                    let mut path = step.path.clone();
                    v.from_directions.insert(step.start_direction);
                    v.path.append(&mut path);
                })
                .or_insert_with(|| VisitState{
                    from_directions: HashSet::from([step.start_direction]),
                    path: step.path.clone()
                });
        }

        //println!(" -> follow {directions:?}");

        for direction in directions {
            let next_point = match direction {
                Up => { if step.y > 0 {Some((step.y - 1, step.x))} else {None} }
                Right => { if step.x < max_x {Some((step.y, step.x + 1))} else {None} }
                Down => { if step.y < max_y {Some((step.y + 1, step.x))} else {None} }
                Left => { if step.x > 0 {Some((step.y, step.x - 1))} else {None} }
            };

            let start_direction = if step.length == 0 {
                direction.clone()
            } else {
                step.start_direction
            };

            if let Some(point) = next_point {
                let mut path = step.path.clone();
                path.push(point);
                let step = Step {
                    y: point.0,
                    x: point.1,
                    start_direction,
                    from_direction: invert_direction(direction),
                    path,
                    length: step.length + 1
                };
                q.push_back(step);
            }
        }
    }

    None
}

fn invert_direction(direction: &Direction) -> Direction {
    match direction {
        Up => Down,
        Right => Left,
        Down => Up,
        Left => Right
    }
}

fn parse(input: &str) -> (Vec<Vec<HashSet<Direction>>>, (usize, usize)) {
    let mut start_point: (usize, usize) = (0, 0);
    let matrix = parse_to_char_matrix(input);
    let matrix = matrix
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.iter().enumerate().map(|(x, c)| {
                match c {
                    '.' => HashSet::new(),
                    'S' => {
                        start_point = (y, x);
                        HashSet::from([Up, Right, Down, Left])
                    }
                    '|' => HashSet::from([Up, Down]),
                    '-' => HashSet::from([Left, Right]),
                    'L' => HashSet::from([Up, Right]),
                    'J' => HashSet::from([Up, Left]),
                    'F' => HashSet::from([Right, Down]),
                    '7' => HashSet::from([Left, Down]),
                    _ => { panic!("{c} not recognized") }
                }
            }).collect()
        })
        .collect();

    (matrix, start_point)
}