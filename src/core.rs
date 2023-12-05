pub trait Solution {
    fn get_day(&self) -> &'static str;
    fn solve1(&self, input: String) -> String;
    fn solve2(&self, input: String) -> String;
}

#[derive(Debug)]
pub(crate) struct Cell1 {
    pub(crate) index: usize,
    pub(crate) value: i32,
}

#[derive(Debug)]
pub(crate) struct Cell2<TIndex, TValue> {
    pub(crate) index: Point<TIndex>,
    pub(crate) value: TValue,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub(crate) struct Point<T> {
    pub(crate) x: T,
    pub(crate) y: T,
}

pub(crate) fn parse_to_char_matrix(input: String) -> Vec<Vec<char>> {
    input.split("\n").map(|line: &str| {
        line.chars().collect()
    }).collect()
}

fn create_cell<T>(point: Point<usize>, chars: Vec<&T>) -> Cell2<usize, Vec<&T>> {
    let x = point.x - chars.len();
    let point = Point { x, y: point.y };
    Cell2 {
        index: point,
        value: chars
    }
}

pub(crate) fn find<T, F1>(matrix: &Vec<Vec<T>>, matcher: F1) -> Vec<Cell2<usize, Vec<&T>>>
    where F1: Fn(Point<usize>, &T) -> bool
{
    let mut result = Vec::new();

    for (y, row) in matrix.iter().enumerate() {
        let mut buffer = Vec::new();
        for (x, cell) in row.iter().enumerate() {
            let point = Point { x, y };
            if matcher(point, cell) {
                buffer.push(cell);
            } else if !buffer.is_empty() {
                result.push(create_cell(point, buffer.clone()));
                buffer.clear();
            }
        }

        if !buffer.is_empty() {
            let point = Point { x: row.len(), y };
            result.push(create_cell(point, buffer.clone()));
            buffer.clear();
        }
    }

    result
}

pub(crate) fn parse_int(input: &str) -> i32 {
    if let Ok(output) = i32::from_str_radix(input, 10) {
        output
    } else {
        println!("Cannot parse {input}");
        -1000000
    }
}

pub(crate) fn parse_i64(input: &str) -> i64 {
    if let Ok(output) = i64::from_str_radix(input, 10) {
        output
    } else {
        println!("Cannot parse {input}");
        -1000000
    }
}