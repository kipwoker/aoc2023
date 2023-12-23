#![allow(unused_variables)]

use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use crate::core::{parse_i32, Solution};

pub struct Day22 {}

impl Solution for Day22 {
    fn get_day(&self) -> &'static str {
        "22"
    }

    fn solve1(&self, input: String) -> String {
        let mut bricks = parse(input.as_str());
        vacuum_bricks(&mut bricks);
        let (below, above) = get_relations(&bricks);
        let counter = calculate(&below, &above);

        counter.to_string()
    }
    fn solve2(&self, input: String) -> String {
        let mut bricks = parse(input.as_str());
        vacuum_bricks(&mut bricks);
        let (below, above) = get_relations(&bricks);
        let counter = calculate2(&bricks, &below);

        counter.to_string()
    }
}

fn calculate(below: &HashMap<usize, HashSet<usize>>, above: &HashMap<usize, HashSet<usize>>) -> i32 {
    let mut counter = 0;
    for (_, above_nodes) in above {
        if above_nodes.iter().all(|x| below.get(x).unwrap().len() > 1) {
            counter += 1;
        }
    }

    counter
}

fn calculate2(bricks: &Vec<Brick>, below: &HashMap<usize, HashSet<usize>>) -> usize {
    let n = bricks.len();

    let mut result = 0;
    for i in 0..n {
        let mut chain = HashSet::new();
        chain.insert(i);

        for j in (i + 1)..n {
            if bricks[j].start[2] <= 1 {
                continue;
            }

            if below.get(&j).unwrap().iter().all(|x| chain.contains(x)) {
                chain.insert(j);
            }
        }
        result += chain.len() - 1
    }

    result
}

fn get_relations(bricks: &Vec<Brick>) -> (HashMap<usize, HashSet<usize>>, HashMap<usize, HashSet<usize>>) {
    let n = bricks.len();
    let mut below = HashMap::new();
    let mut above = HashMap::new();
    for i in 0..n {
        below.insert(i, HashSet::new());
        above.insert(i, HashSet::new());
    }

    for i in 0..n {
        let brick = &bricks[i];
        for j in (i + 1)..n {
            let another_brick_in_the_wall = &bricks[j];
            let top = max(brick.start[2], brick.end[2]);
            let bottom = min(another_brick_in_the_wall.start[2], another_brick_in_the_wall.end[2]);
            let diff = bottom - top;
            if diff > 1 {
                break;
            }

            if diff == 1 && intersects(brick, another_brick_in_the_wall) {
                below.entry(j).and_modify(|v| { v.insert(i); });
                above.entry(i).and_modify(|v| { v.insert(j); });
            }
        }
    }

    (below, above)
}

fn vacuum_bricks(bricks: &mut Vec<Brick>) {
    bricks.sort_by(|x, y| min(x.start[2], x.end[2]).cmp(&min(y.start[2], y.end[2])));

    let n = bricks.len();
    for i in 0..n {
        vacuum_brick(bricks, i);
    }
}

fn vacuum_brick(bricks: &mut Vec<Brick>, idx: usize) {
    let brick = &bricks[idx];
    let bottom = min(brick.start[2], brick.end[2]);
    for i in (0..idx).rev() {
        let another_brick_in_the_wall = &bricks[i];
        if intersects(brick, another_brick_in_the_wall) {
            let top = max(another_brick_in_the_wall.start[2], another_brick_in_the_wall.end[2]);
            let diff = bottom - top - 1;
            bricks[idx].start[2] -= diff;
            bricks[idx].end[2] -= diff;
            return;
        }
    }

    let top = 0;
    let diff = bottom - top - 1;
    bricks[idx].start[2] -= diff;
    bricks[idx].end[2] -= diff;
}

fn intersects(a: &Brick, b: &Brick) -> bool {
    let intersect_x = intersects_by_points((a.start[0], a.end[0]), (b.start[0], b.end[0]));
    let intersect_y = intersects_by_points((a.start[1], a.end[1]), (b.start[1], b.end[1]));

    intersect_x > 0 && intersect_y > 0
}

fn intersects_by_points(a: (i32, i32), b: (i32, i32)) -> i32 {
    if !(a.0 > b.1 || b.0 > a.1) {
        min(a.1, b.1) - max(a.0, b.0) + 1
    } else {
        0
    }
}

#[cfg(test)]
mod day22_tests {
    use super::*;

    #[test]
    fn test_intersects_by_points() {
        assert_eq!(intersects_by_points((1, 1), (2, 2)), 0);
        assert_eq!(intersects_by_points((1, 1), (1, 2)), 1);
        assert_eq!(intersects_by_points((1, 1), (0, 2)), 1);
        assert_eq!(intersects_by_points((5, 8), (3, 10)), 4);
        assert_eq!(intersects_by_points((2, 5), (5, 10)), 1);
        assert_eq!(intersects_by_points((2, 5), (0, 2)), 1);
        assert_eq!(intersects_by_points((2, 5), (0, 1)), 0);
    }

    #[test]
    fn test_vacuum_bricks() {
        let mut bricks = vec![
            Brick { start: [0, 1, 2], end: [2, 2, 3] },
            Brick { start: [0, 1, 10], end: [2, 2, 11] },
        ];
        vacuum_bricks(&mut bricks);
        let expected = vec![
            Brick { start: [0, 1, 1], end: [2, 2, 2] },
            Brick { start: [0, 1, 3], end: [2, 2, 4] }
        ];
        assert_eq!(expected, bricks);

        let mut bricks = vec![
            Brick { start: [0, 0, 2], end: [0, 1, 2] },
            Brick { start: [0, 2, 4], end: [1, 2, 4] },
            Brick { start: [2, 1, 7], end: [2, 2, 7] },
            Brick { start: [1, 0, 10], end: [2, 0, 10] },
            Brick { start: [1, 1, 15], end: [1, 1, 15] },
        ];
        vacuum_bricks(&mut bricks);
        let expected = vec![
            Brick { start: [0, 0, 1], end: [0, 1, 1] },
            Brick { start: [0, 2, 1], end: [1, 2, 1] },
            Brick { start: [2, 1, 1], end: [2, 2, 1] },
            Brick { start: [1, 0, 1], end: [2, 0, 1] },
            Brick { start: [1, 1, 1], end: [1, 1, 1] }
        ];
        assert_eq!(expected, bricks);


        let mut bricks = vec![
            Brick { start: [0, 0, 2], end: [0, 1, 2] },
            Brick { start: [0, 2, 4], end: [1, 2, 4] },
            Brick { start: [2, 1, 7], end: [2, 2, 7] },
            Brick { start: [1, 0, 10], end: [2, 0, 10] },
            Brick { start: [1, 1, 15], end: [1, 2, 15] },
        ];
        vacuum_bricks(&mut bricks);
        let expected = vec![
            Brick { start: [0, 0, 1], end: [0, 1, 1] },
            Brick { start: [0, 2, 1], end: [1, 2, 1] },
            Brick { start: [2, 1, 1], end: [2, 2, 1] },
            Brick { start: [1, 0, 1], end: [2, 0, 1] },
            Brick { start: [1, 1, 2], end: [1, 2, 2] }
        ];
        assert_eq!(expected, bricks);


        let mut bricks = vec![
            Brick { start: [0, 0, 2], end: [0, 1, 2] },
            Brick { start: [0, 2, 4], end: [1, 2, 4] },
            Brick { start: [2, 1, 7], end: [2, 2, 7] },
            Brick { start: [1, 0, 10], end: [2, 0, 10] },
            Brick { start: [7, 7, 15], end: [8, 8, 15] },
        ];
        vacuum_bricks(&mut bricks);
        let expected = vec![
            Brick { start: [0, 0, 1], end: [0, 1, 1] },
            Brick { start: [0, 2, 1], end: [1, 2, 1] },
            Brick { start: [2, 1, 1], end: [2, 2, 1] },
            Brick { start: [1, 0, 1], end: [2, 0, 1] },
            Brick { start: [7, 7, 1], end: [8, 8, 1] }
        ];
        assert_eq!(expected, bricks);
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Brick {
    start: [i32; 3],
    end: [i32; 3],
}

fn parse(input: &str) -> Vec<Brick> {
    input.split("\n").map(|line| {
        let points: Vec<[i32; 3]> = line
            .split("~")
            .map(|x| {
                let coords = x.split(",").map(parse_i32).collect::<Vec<i32>>();
                let coords: [i32; 3] = match coords.as_slice() {
                    [a, b, c] => [*a, *b, *c],
                    _ => unreachable!(),
                };
                coords
            })
            .collect();
        Brick { start: points[0].clone(), end: points[1].clone() }
    }).collect()
}