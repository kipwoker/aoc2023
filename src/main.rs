#![recursion_limit = "102400"]
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
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

use std::{env, fs};
use std::time::Instant;
use crate::core::Solution;

fn main() {
    let solution = day25::Day25 {};
    let day = solution.get_day();

    let binding = env::current_dir().expect("Current directory not found");
    let current_dir = binding.display();
    println!("Launched from dir {current_dir}");
    println!("Solving day {day}... ");
    let input_test_path = format!("inputs/{day}.test.txt");
    let input_path = format!("inputs/{day}.txt");

    let input_paths = [
        input_test_path,
        input_path
    ];

    let solvers: Vec<Box<dyn Fn(String) -> String>> = vec![
        Box::new(|x| solution.solve1(x)),
        Box::new(|x| solution.solve2(x)),
    ];

    println!("============================================");
    for path in input_paths {
        let p = path.clone();
        println!("| > {p}");
        let input_content = fs::read_to_string(path.clone()).expect("File not found");
        for (index, solver) in solvers.iter().enumerate() {
            println!("|-------------------------------------------");
            let start_time = Instant::now();
            let output = solver(input_content.clone());
            let elapsed_time = start_time.elapsed();
            let number = index + 1;
            println!("| Part {number}: {output}");
            println!("| Execution time: {:.2?}", elapsed_time);
        }
        println!("============================================");
    }
}