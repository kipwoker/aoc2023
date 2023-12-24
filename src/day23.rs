#![allow(unused_variables)]

use std::collections::{HashMap, HashSet};
use crate::core::{Cell, Event, EventLog, Matrix, parse_to_char_matrix, Point, Solution};
use crate::core::Event::{ChangeColor, SetMatrix};

pub struct Day23 {}

impl Solution for Day23 {
    fn get_day(&self) -> &'static str {
        "23"
    }

    fn solve1(&self, input: String) -> String {
        solve(input.as_str(), false)
    }
    fn solve2(&self, input: String) -> String {
        solve(input.as_str(), true)
    }
}

fn solve(input: &str, skip: bool) -> String {
    let matrix = parse_to_char_matrix(input);
    let n = matrix.len();
    let direction_map: HashMap<char, Point<isize>> = HashMap::from([
        ('<', Point { y: 0, x: -1 }),
        ('>', Point { y: 0, x: 1 }),
        ('v', Point { y: 1, x: 0 }),
        ('^', Point { y: -1, x: 0 })
    ]);

    let color_map = HashMap::from([
        ('#', "#df9b86"),
        ('.', "#EAECCC"),
        ('<', "#DBCC95"),
        ('>', "#DBCC95"),
        ('v', "#DBCC95"),
        ('^', "#DBCC95"),
    ]);

    let view = matrix
        .iter()
        .map(|row| row.iter().map(|c|
            Cell { color: color_map.get(c).unwrap().to_string(), content: c.to_string()}
        ).collect())
        .collect();
    let mut event_log = EventLog::new();
    event_log.append(SetMatrix(view));

    let forks = find_forks(&matrix, &direction_map);
    for (point, _) in &forks {
        event_log.append(ChangeColor(point.y, point.x, "#C3E2C2".to_string()));
    }

    let routes = find_routes(&matrix, &direction_map, &forks, skip);

    let n = routes.len();
    let mut visited: Vec<bool> = vec![false; n];
    let (distance, mut events) = find_max_path(&routes, &mut visited, 0, n - 1, 0).unwrap();

    reduce_events(&mut events);

    for event in events {
        event_log.append(event);
    }

    event_log.dump_to_file("output.txt");
    distance.to_string()
}

impl Point<usize> {
    fn add(&self, point: &Point<isize>, max: Option<usize>) -> Option<Point<usize>> {
        let y = self.y as isize + point.y;
        if y < 0 || max.is_some_and(|m| y >= m as isize) {
            return None;
        }

        let x = self.x as isize + point.x;
        if x < 0 || max.is_some_and(|m| x >= m as isize) {
            return None;
        }

        let y = y as usize;
        let x = x as usize;
        Some(Point { y, x })
    }
}

#[derive(Clone, Debug)]
struct Route {
    end: usize,
    distance: usize,
    points: Vec<Point<usize>>
}

fn is_fork(matrix: &Vec<Vec<char>>, direction_map: &HashMap<char, Point<isize>>, point: &Point<usize>) -> bool {
    let n = matrix.len();
    let mut count = 0;

    for next in matrix.get_neighbors_4(&(point.y, point.x)) {
        if let Some(direction) = direction_map.get(&matrix[next.0][next.1]) {
            count += 1;
            if count > 1 {
                return true
            }
        }
    }
    count > 1
}

fn find_route(
    matrix: &Vec<Vec<char>>,
    forks: &HashMap<Point<usize>, usize>,
    directions: &Vec<&Point<isize>>,
    distance: usize,
    prev: &Point<usize>,
    cursor: &Point<usize>
) -> Route {
    if let Some(&end) = forks.get(&cursor) {
        return Route { end, distance, points: vec![cursor.clone()] };
    }

    for &direction in directions {
        if let Some(next) = cursor.add(direction, Some(matrix.len())) {
            if next == *prev || matrix[next.y][next.x] == '#' {
                continue;
            }
            let mut route = find_route(matrix, forks, directions, distance + 1, cursor, &next);
            route.points.push(cursor.clone());
            return route;
        }
    }

    unreachable!();
}

fn find_forks(matrix: &Vec<Vec<char>>, direction_map: &HashMap<char, Point<isize>>) -> HashMap<Point<usize>, usize> {
    let n = matrix.len();
    let mut forks = Vec::new();
    for (y, row) in matrix.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if c != '.' {
                continue;
            }

            let point = Point { y, x };
            if y == 0 || y == n - 1 || is_fork(&matrix, direction_map, &point) {
                forks.push(point);
            }
        }
    }
    //println!("{forks:?}");

    let forks: HashMap<Point<usize>, usize> = forks
        .iter()
        .enumerate()
        .map(|(idx, point)| (point.clone(), idx))
        .collect();

    forks
}

fn find_routes(
    matrix: &Vec<Vec<char>>,
    direction_map: &HashMap<char, Point<isize>>,
    forks: &HashMap<Point<usize>, usize>,
    skip: bool
) -> Vec<Vec<Route>> {
    let directions: Vec<&Point<isize>> = direction_map.values().collect();

    let n = matrix.len();
    let mut routes = vec![Vec::new(); forks.len()];

    for (fork, i) in forks {
        for &direction in &directions {
            if let Some(point) = fork.add(direction, Some(n)) {
                let &cell = &matrix[point.y][point.x];
                if cell == '#' {
                    continue;
                }

                if skip || cell == '.' || direction == direction_map.get(&cell).unwrap() {
                    let mut end = find_route(&matrix, &forks, &directions, 1, fork, &point);
                    end.points.push(fork.clone());
                    routes[i.clone()].push(end);
                }
            }
        }
    }

    //println!("{routes:?}");

    routes
}

fn reduce_events(events: &mut Vec<Event>) {
    let mut cache = HashSet::new();
    let mut to_remove = Vec::new();
    for (i, event) in events.iter().enumerate() {
        match event {
            ChangeColor(y, x, _) => {
                let key = (y, x);
                if cache.contains(&key) {
                    to_remove.push(i);
                } else {
                    cache.insert(key);
                }
            }
            _ => {}
        }
    }

    for i in to_remove.iter().rev() {
        events.remove(*i);
    }
}

fn find_max_path(g: &Vec<Vec<Route>>, visited: &mut Vec<bool>, start: usize, end: usize, depth: usize) -> Option<(usize, Vec<Event>)> {
    if start == end {
        return Some((0, Vec::new()));
    }

    visited[start] = true;
    let mut max_length = -1isize;
    let mut full_path = Vec::new();

    let colors = ["#958dd9", "#8ACDD7", "#6dd7ba", "#6dd770", "#c26dd7", "#db82d8", "#FF90BC", "#67729D", "#FFA732", "#F4F27E", "#6DB9EF"];

    let mut i = 0;
    for route in &g[start] {
        let new_start = route.end;
        if !visited[new_start] {
            if let Some((length, path)) = find_max_path(g, visited, new_start, end, depth + i + 1) {
                max_length = max_length.max((length + route.distance) as isize);
                let points = route.points.iter().rev()
                    .map(|x| ChangeColor(x.y, x.x, colors[(depth + i) % colors.len()].to_string()))
                    .collect();
                full_path = [full_path, points, path].concat();
                i += 3;
            }
        }
    }
    visited[start] = false;

    if max_length == -1 {
        None
    } else {
        Some((max_length as usize, full_path))
    }
}
