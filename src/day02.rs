#![allow(unused_variables)]

use std::cmp::max;
use std::collections::HashMap;
use crate::core::Solution;

pub struct Day02 {}

struct Game<'a> {
    id: i32,
    rounds: Vec<HashMap<&'a str, i32>>,
}

impl Solution for Day02 {
    fn get_day(&self) -> &'static str {
        "02"
    }

    fn solve1(&self, input: String) -> String {
        let games = parse(input.as_str());

        solve1(games)
    }

    fn solve2(&self, input: String) -> String {
        let games = parse(input.as_str());

        solve2(games)
    }
}

fn parse(input: &str) -> Vec<Game> {
    let lines = input.split("\n");
    let games: Vec<Game> = lines
        .map(|line| {
            let parts: Vec<&str> = line.split(":").collect();
            let header: Vec<&str> = parts[0].split(" ").collect();
            let id = header[1].parse::<i32>().unwrap();

            let rounds = parts[1]
                .split(";")
                .map(|round| {
                    round.split(",")
                        .map(|pair| {
                            let entry: Vec<&str> = pair.split_whitespace().collect();
                            let key = entry[1].trim();
                            let count = entry[0].trim().parse::<i32>().unwrap();
                            (key, count)
                        })
                        .collect::<HashMap<_, _>>()
                })
                .collect::<Vec<_>>();

            Game { id, rounds }
        })
        .collect();
    games
}

fn solve1(games: Vec<Game>) -> String {
    let limits = HashMap::from([
        ("red", 12),
        ("green", 13),
        ("blue", 14)
    ]);

    let valid = get_valid(games, limits);
    let result = valid.iter().map(|game| { game.id }).sum::<i32>();
    return result.to_string()
}

fn get_valid<'a>(games: Vec<Game<'a>>, limits: HashMap<&'a str, i32>) -> Vec<Game<'a>> {
    games.into_iter().filter(|game| {
        game.rounds.iter().all(|rounds| {
            rounds.iter().all(|(key, value)| {
                if let Some(limit) = limits.get(key) {
                    value <= limit
                } else {
                    let id = game.id;
                    println!("No limit {id} {key} {value}");
                    false
                }
            })
        })
    }).collect()
}

fn solve2(games: Vec<Game>) -> String {
    let sum: i64 = games.iter().map(|game| {
        let folded = game.rounds.iter().fold(HashMap::new(), |mut acc, round| {
            for (key, &value) in round.iter() {
                let y = acc.entry(key).or_insert(0);
                *acc.entry(key).or_insert(0) = max(value, *acc.get(key).unwrap_or(&0));
            }
            acc
        });

        let result: i64 = folded.values().map(|&x| x as i64).product();
        result
    }).sum();

    sum.to_string()
}
