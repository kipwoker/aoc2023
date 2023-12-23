#![allow(unused_variables)]

use std::collections::HashMap;
use crate::core::{Matrix, parse_to_char_matrix, Point, Solution};

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

    let forks = find_forks(&matrix, &direction_map);

    let routes = find_routes(&matrix, &direction_map, &forks, skip);
    let n = routes.len();
    let mut visited: Vec<bool> = vec![false; n];
    let distance = find_max_path(&routes, &mut visited, 0, n - 1).unwrap();

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
        return Route { end, distance };
    }

    for &direction in directions {
        if let Some(next) = cursor.add(direction, Some(matrix.len())) {
            if next == *prev || matrix[next.y][next.x] == '#' {
                continue;
            }
            return find_route(matrix, forks, directions, distance + 1, cursor, &next);
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
                    let end = find_route(&matrix, &forks, &directions, 1, fork, &point);
                    routes[i.clone()].push(end);
                }
            }
        }
    }

    //println!("{routes:?}");

    routes
}

fn find_max_path(g: &Vec<Vec<Route>>, visited: &mut Vec<bool>, start: usize, end: usize) -> Option<usize> {
    if start == end {
        return Some(0);
    }

    visited[start] = true;
    let mut max_length = -1isize;
    for route in &g[start] {
        let new_start = route.end;
        if !visited[new_start] {
            if let Some(path) = find_max_path(g, visited, new_start, end) {
                max_length = max_length.max((path + route.distance) as isize);
            }
        }
    }
    visited[start] = false;

    if max_length == -1 {
        None
    } else {
        Some(max_length as usize)
    }
}
