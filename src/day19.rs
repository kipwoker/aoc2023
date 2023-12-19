#![allow(unused_variables)]

use std::collections::HashMap;
use crate::core::{parse_i32, Solution};

pub struct Day19 {}

impl Solution for Day19 {
    fn get_day(&self) -> &'static str {
        "19"
    }

    fn solve1(&self, input: String) -> String {
        let (workflows, states) = parse(input.as_str());

        let count: i32 = states.iter()
            .map(|state| if execute(&workflows, state) { calculate(state) } else {0})
            .sum();

        count.to_string()
    }
    fn solve2(&self, input: String) -> String {
        String::new()
    }
}

#[derive(Debug)]
struct Workflow {
    name: String,
    operations: Vec<Operation>,
}

#[derive(Debug)]
enum Operator {
    More,
    Less,
}

#[derive(Debug)]
enum Return {
    Next(String),
    Accept,
    Reject,
}


#[derive(Debug)]
enum Operation {
    Condition(char, Operator, i32, Return),
    Return(Return),
}

#[derive(Debug, Clone)]
struct State {
    map: HashMap<char, i32>
}

fn calculate(state: &State) -> i32 {
    state.map.values().sum()
}

fn execute(workflows: &Vec<Workflow>, state: &State) -> bool {
    let reference: HashMap<&str, &Vec<Operation>> = workflows.iter().map(|w| (w.name.as_str(), &w.operations)).collect();
    let mut operations = *reference.get("in").unwrap();

    loop {
        for operation in operations {
            match operation {
                Operation::Condition(var, operator, val, ret) => {
                    if match operator {
                        Operator::More => {state.map.get(var).unwrap() > val}
                        Operator::Less => {state.map.get(var).unwrap() < val}
                    } {
                        match ret {
                            Return::Next(key) => { operations = reference.get(key.as_str()).unwrap(); break; }
                            Return::Accept => { return true; }
                            Return::Reject => { return false; }
                        }
                    }
                }
                Operation::Return(ret) => {
                    match ret {
                        Return::Next(key) => { operations = reference.get(key.as_str()).unwrap(); break; }
                        Return::Accept => { return true; }
                        Return::Reject => { return false; }
                    }
                }
            }
        }
    }
}

fn parse(input: &str) -> (Vec<Workflow>, Vec<State>) {
    let lines: Vec<&str> = input.split("\n").collect();

    let (workflows, next) = parse_workflows(&lines);
    let states = parse_state(&lines, next);

    (workflows, states)
}

fn parse_state(lines: &Vec<&str>, from: usize) -> Vec<State> {
    let mut result = Vec::new();
    for i in from..lines.len() {
        let mut map = HashMap::new();
        let line = lines[i];
        line
            .trim_start_matches("{")
            .trim_end_matches("}")
            .split(",")
            .for_each(|x| {
                let parts: Vec<&str> = x.split("=").collect();
                let var = parts[0].chars().next().unwrap();
                let val = parse_i32(parts[1]);

                map.insert(var, val);
            });
        result.push(State { map });
    }

    result
}

fn parse_workflows(lines: &Vec<&str>) -> (Vec<Workflow>, usize) {
    let mut workflows = Vec::new();

    for (index, line) in lines.iter().enumerate() {
        if *line == "" {
            return (workflows, index + 1)
        }

        let parts = line.split("{").collect::<Vec<&str>>();
        let name = String::from(parts[0]);
        let operations = parts[1];
        let operations = &operations[0..operations.len() - 1];
        let operations: Vec<Operation> = operations.split(",").map(|op| {
            if op.contains(">") || op.contains("<") {
                let chars: Vec<char> = op.chars().collect();
                let var = chars[0];
                let operator = if chars[1] == '<' { Operator::Less } else { Operator::More };
                let rest: String = chars[2..].iter().collect();
                let rest: Vec<&str> = rest.split(":").collect();
                let val = parse_i32(rest[0]);
                let ret = parse_return(rest[1]);

                Operation::Condition(var, operator, val, ret)
            } else {
                let ret = parse_return(op);
                Operation::Return(ret)
            }
        }).collect();


        workflows.push(Workflow{name, operations});
    }

    unreachable!()
}

fn parse_return(input: &str) -> Return {
    match input {
        "A" => Return::Accept,
        "R" => Return::Reject,
        workflow => Return::Next(String::from(workflow))
    }
}