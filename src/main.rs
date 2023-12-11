mod day01;
mod core;
mod day00;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;

use std::{env, fs};
use crate::core::Solution;
use crate::day11::Day11;

fn main() {
    let solution = Day11 {};
    let day = solution.get_day();

    let binding = env::current_dir().expect("Current directory not found");
    let current_dir = binding.display();
    println!("Launched from dir {current_dir}");
    println!("Solving day {day}... ");
    let input_test_path = format!("inputs/{day}.test.txt");
    let input_path = format!("inputs/{day}.txt");

    let input_paths = [input_test_path, input_path];

    println!("===================");
    for path in input_paths {
        let p = path.clone();
        println!("Use input: {p}");
        let input_content = fs::read_to_string(path.clone()).expect("File not found");
        let output1 = solution.solve1(input_content.clone());
        println!("Result 1: {output1}");
        let output2 = solution.solve2(input_content.clone());
        println!("Result 2: {output2}");
        println!("===================");
    }
}
