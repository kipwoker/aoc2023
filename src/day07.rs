use std::cmp::{max, Ordering};
use std::cmp::Ordering::{Equal, Greater, Less};
use std::collections::{HashSet, VecDeque};
use crate::core::{group_by, parse_i32, Solution};

pub struct Day07 {}


#[derive(Debug)]
struct Bid {
    hand: Vec<i32>,
    value: i32,
    state: i32,
}

impl Solution for Day07 {
    fn get_day(&self) -> &'static str {
        "07"
    }

    fn solve1(&self, input: String) -> String {
        let result = calculate_result(input, false);

        result.to_string()
    }
    fn solve2(&self, input: String) -> String {
        let result = calculate_result(input, true);

        result.to_string()
    }
}

fn calculate_result(input: String, with_joker: bool) -> i64 {
    let mut bids = parse(input.as_str(), with_joker);
    bids.sort_by(|a, b| { compare(a, b, with_joker) });
    println!("{bids:?}");

    let mut result: i64 = 0;
    for (index, bid) in bids.iter().enumerate() {
        let place = (index + 1) as i32;
        result += (place * bid.value) as i64;
    }

    result
}

fn compare(a: &Bid, b: &Bid, with_joker: bool) -> Ordering {
    if a.state == b.state {
        let n = a.hand.len();
        for i in 0..n {
            let a = a.hand[i];
            let a = if with_joker && a == 11 { 1 } else { a };
            let b = b.hand[i];
            let b = if with_joker && b == 11 { 1 } else { b };
            if a < b {
                return Less;
            } else if a > b {
                return Greater;
            }
        }

        return Equal;
    }

    if a.state < b.state {
        return Less;
    }

    return Greater;
}

fn calculate_state(hand: &Vec<i32>) -> i32 {
    let group = group_by(hand.clone(), |x| { x.clone() });
    let set: HashSet<usize> = group.values().map(|v| v.len()).collect();

    //Five of a kind
    if set.contains(&5) {
        return 7;
    }

    // Four of a kind
    if set.contains(&4) {
        return 6;
    }

    // Full house
    if set.contains(&3) && set.contains(&2) {
        return 5;
    }

    // Three of a kind
    if set.contains(&3) {
        return 4;
    }

    // Two pairs
    let pairs: Vec<usize> = group.values()
        .map(|v| v.len())
        .filter(|x| x == &2).collect();

    if pairs.len() == 2 {
        return 3;
    }

    // One Pair
    if pairs.len() == 1 {
        return 2;
    }

    // High card
    1
}

fn calculate_state_with_joker(hand: &Vec<i32>) -> i32 {
    let mut cards: HashSet<&i32> = hand.iter().filter(|x| **x != 11).collect();
    if cards.is_empty() {
        cards = HashSet::from([&14]);
    }

    let mut queue = VecDeque::new();
    queue.push_back(hand.clone());

    let mut max_value = 0;
    loop {
        let item = queue.pop_front();
        if item.is_none() {
            break max_value;
        }
        let item = item.unwrap();

        let mut is_clean = true;
        for (index, card) in item.iter().enumerate() {
            if *card == 11 {
                is_clean = false;
                for (_, x) in cards.iter().enumerate() {
                    let mut new_hand = item.clone();
                    new_hand[index] = (**x).clone();
                    queue.push_back(new_hand);
                }
            }
        }

        if is_clean {
            let res = calculate_state(&item);
            max_value = max(max_value, res);
        }
    }
}

fn parse(input: &str, with_joker: bool) -> Vec<Bid> {
    let lines: Vec<&str> = input.split("\n").collect();
    lines.iter().map(|line| {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let hand: Vec<i32> = parts[0].chars().map(|c| {
            match c {
                '2' => 2,
                '3' => 3,
                '4' => 4,
                '5' => 5,
                '6' => 6,
                '7' => 7,
                '8' => 8,
                '9' => 9,
                'T' => 10,
                'J' => 11,
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                _ => {
                    println!("Unknown {c}");
                    -100000
                }
            }
        }).collect();

        let value = parse_i32(parts[1]);
        let state = if with_joker { calculate_state_with_joker(&hand) } else { calculate_state(&hand) };

        Bid {
            hand,
            value,
            state,
        }
    }).collect()
}