use std::collections::HashMap;

#[derive(Debug)]
struct Cell {
    index: usize,
    value: i32,
}

pub(crate) fn solve(input: String) -> String {
    let lines = input.split("\n");

    let map = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9)
    ]);

    let find = |x: &str, y: &str| x.find(y);
    let reverse_find = |x: &str, y: &str| x.rfind(y);
    let le = |x: usize, y: usize| x <= y;
    let ge = |x: usize, y: usize| x >= y;

    let mut sum = 0;
    for line in lines {
        let mut right = Cell { index: 0, value: -1 };
        let mut left = Cell { index: line.len(), value: -1 };

        for entry in map.iter() {
            let (key, value) = entry;
            left = check(line, key, value, &left, find, ge).unwrap_or(left);
            right = check(line, key, value, &right, reverse_find, le).unwrap_or(right);

            let key = value.to_string();
            let key = key.as_str();
            left = check(line, &key, value, &left, find, ge).unwrap_or(left);
            right = check(line, &key, value, &right, reverse_find, le).unwrap_or(right);
        }

        if left.value == -1 || right.value == -1 {
            println!("Unexpected {line} {left:?} {right:?}")
        }

        let num = left.value * 10 + right.value;

        sum += num;
    }

    sum.to_string()
}

fn check<F1, F2>(
    line: &str,
    pattern: &&str,
    new_value: &i32,
    cell: &Cell,
    search: F1,
    comparer: F2
) -> Option<Cell>
    where
        F1: Fn(&str, &str) -> Option<usize>,
        F2: Fn(usize, usize) -> bool
{
    if let Some(index) = search(line, pattern) {
        if comparer(cell.index, index) {
            return Some(Cell { index, value: *new_value });
        }
    }
    None
}