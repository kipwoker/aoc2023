pub trait Solution {
    fn get_day(&self) -> &'static str;
    fn solve1(&self, input: String) -> String;
    fn solve2(&self, input: String) -> String;
}

#[derive(Debug)]
pub(crate) struct Cell {
    pub(crate) index: usize,
    pub(crate) value: i32,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub(crate) struct Point<T> {
    pub(crate) x: T,
    pub(crate) y: T,
}