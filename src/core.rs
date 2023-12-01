pub trait Solution {
    fn get_day(&self) -> &'static str;
    fn solve(&self, input: String) -> String;
}

#[derive(Debug)]
pub(crate) struct Cell {
    pub(crate) index: usize,
    pub(crate) value: i32,
}