#![allow(unused_variables)]

use std::collections::{HashMap, HashSet, VecDeque};
use Direction::Up;
use crate::core::{Cell, EventLog, parse_to_char_matrix, Solution};
use crate::day10::Direction::{Down, Left, Right};
use colored::Colorize;
use crate::core::Event::{ChangeColor, SetMatrix};

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
    start_point: HashSet<Direction>,
    last_point: (usize, usize),
    path1: Vec<(usize, usize)>,
    path2: Vec<(usize, usize)>
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
        let matrix = parse_to_char_matrix(input.as_str());
        let (g, start_point) = parse(&matrix);
        let result = find_far_point_length(start_point, &g).unwrap();
        let start_point_directions = result.start_point;

        result.length.to_string()
    }
    fn solve2(&self, input: String) -> String {
        let matrix = parse_to_char_matrix(input.as_str());
        let (mut g, start_point) = parse(&matrix);
        let result = find_far_point_length(start_point, &g).unwrap();

        g[start_point.0][start_point.1] = result.start_point.clone();

        let inner = find_inner(&g, &result);
        let count = inner.len();

        write_event_log(&matrix, &start_point, &result, &inner);

        count.to_string()
    }
}

fn write_event_log(matrix: &Vec<Vec<char>>, start_point: &(usize, usize), result: &Result, inner: &HashSet<(usize, usize)>) {
    let mut event_log = EventLog::new();
    let view = matrix
        .iter()
        .map(|row| row.iter().map(|c|
            Cell { color: "#e1e1f9".to_string(), content: c.to_string()}
        ).collect())
        .collect();
    event_log.append(SetMatrix(view));

    event_log.append(ChangeColor(start_point.0, start_point.1, "#fff813".to_string()));
    for i in 0..result.length {
        let path = result.path1[i];
        if start_point.0 != path.0 || start_point.1 != path.1 {
            let c = &matrix[path.0][path.1];
            event_log.append(ChangeColor(path.0, path.1, "#ff5c98".to_string()));
        }

        let path = result.path2[i];
        if start_point.0 != path.0 || start_point.1 != path.1 {
            let c = &matrix[path.0][path.1];
            event_log.append(ChangeColor(path.0, path.1, "#ec4cff".to_string()));
        }
    }

    event_log.append(ChangeColor(result.last_point.0, result.last_point.1, "#fff813".to_string()));

    for (y,x) in inner {
        event_log.append(ChangeColor(y.clone(), x.clone(), "#39ede5".to_string()));
    }
    event_log.dump_to_file("output.txt");
}

fn print_path(matrix: &Vec<Vec<char>>, points: &HashSet<(usize, usize)>, inner: &HashSet<(usize, usize)>) {
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

                let path_points: HashSet<(usize, usize)> = [path1.clone(), path2.clone()]
                    .concat()
                    .iter()
                    .map(|x| x.clone())
                    .collect();

                println!("S = {start_point:?}");

                let result = Result {
                    path_points,
                    length: step.length,
                    start_point,
                    last_point: key.clone(),
                    path1,
                    path2
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

fn parse(matrix: &Vec<Vec<char>>) -> (Vec<Vec<HashSet<Direction>>>, (usize, usize)) {
    let mut start_point: (usize, usize) = (0, 0);
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