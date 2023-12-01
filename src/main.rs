mod day01;
mod core;
mod day00;

use std::{env, fs};
use crate::core::Solution;
use crate::day01::Day01;

fn main() {
    let solution = Day01 {};
    let day = solution.get_day();

    let binding = env::current_dir().expect("Current directory not found");
    let current_dir = binding.display();
    println!("Launched from dir {current_dir}");
    println!("Solving day {day}... ");
    let input_test_path = format!("inputs/{day}.test.txt");
    let input_path = format!("inputs/{day}.txt");

    let input_paths = [input_test_path, input_path];

    for path in input_paths {
        let input_content = fs::read_to_string(path).expect("File not found");
        let output = solution.solve(input_content);
        println!("Result: {output}")
    }
}
