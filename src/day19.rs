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
        let reference: HashMap<&str, &Workflow> = workflows.iter().map(|w| (w.name.as_str(), w)).collect();
        let start_workflow = reference.get("in").unwrap();
        let count: i32 = states.iter()
            .map(|state| {
                let results = execute(&reference, state, start_workflow, 0);
                let mut sum = 0;
                for (result, new_state) in results {
                    if result {
                        sum += new_state.map.values().map(|x| x.from).sum::<i32>()
                    }
                }
                sum
            })
            .sum();

        count.to_string()
    }
    fn solve2(&self, input: String) -> String {
        let (workflows, _) = parse(input.as_str());
        let reference: HashMap<&str, &Workflow> = workflows.iter().map(|w| (w.name.as_str(), w)).collect();
        let start_workflow = reference.get("in").unwrap();
        let total = 4000;
        let start_state = State {
            map: HashMap::from([
                ('x', Range{ from: 1, to: total }),
                ('m', Range{ from: 1, to: total }),
                ('a', Range{ from: 1, to: total }),
                ('s', Range{ from: 1, to: total })
            ])
        };
        let results = execute(&reference, &start_state, start_workflow, 0);

        let mut count = 0u64;
        for (result, new_state) in results {
            if result {
                let nums = new_state.map.values().map(|range| {
                    (range.to - range.from + 1) as u64
                }).collect::<Vec<u64>>();

                let mut mul = 1u64;
                for num in nums {
                    mul *= num;
                }

                count += mul;
            }
        }
        count.to_string()
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

#[derive(Debug, Clone, Copy)]
struct Range {
    from: i32,
    to: i32
}

#[derive(Debug)]
struct Fork {
    yes: Option<Range>,
    no: Option<Range>
}

#[derive(Debug)]
enum Operation {
    Condition(char, Operator, i32, Return),
    Return(Return),
}

#[derive(Debug, Clone)]
struct State {
    map: HashMap<char, Range>
}

impl Range {
    pub fn apply(&self, operator: &Operator, val: &i32) -> Fork {
        match operator {
            Operator::More => {
                if &self.from > val {
                    Fork{
                        yes: Some(Range { from: self.from.clone(), to: self.to.clone() }),
                        no: None
                    }
                } else if &self.to < val {
                    Fork{
                        yes: None,
                        no: Some(Range { from: self.from.clone(), to: self.to.clone() })
                    }
                } else {
                    Fork{
                        yes: Some(Range { from: val + 1, to: self.to.clone() }),
                        no: Some(Range { from: self.from.clone(), to: val.clone() })
                    }
                }
            }
            Operator::Less => {
                if &self.from > val {
                    Fork{
                        yes: None,
                        no: Some(Range { from: self.from.clone(), to: self.to.clone() })
                    }
                } else if &self.to < val {
                    Fork{
                        yes: Some(Range { from: self.from.clone(), to: self.to.clone() }),
                        no: None
                    }
                } else {
                    Fork{
                        yes: Some(Range { from: self.from.clone(), to: val - 1 }),
                        no: Some(Range { from: val.clone(), to: self.to.clone() })
                    }
                }
            }
        }
    }
}

fn execute(reference: &HashMap<&str, &Workflow>, state: &State, workflow: &Workflow, operation_idx: usize) -> Vec<(bool, State)> {
    let operation = &workflow.operations[operation_idx];
    let mut result: Vec<(bool, State)> = Vec::new();
    match operation {
        Operation::Condition(var, operator, val, ret) => {
            let range = state.map.get(var).unwrap();
            let fork = range.apply(operator, val);
            if let Some(yes) = fork.yes {
                let mut state = state.clone();
                state.map.insert(var.clone(), yes);

                execute_return(&mut result, reference, state, ret);
            }

            if let Some(no) = fork.no {
                let mut state = state.clone();
                state.map.insert(var.clone(), no);

                let batch = execute(reference, &state, workflow, operation_idx + 1);
                for item in batch {
                    result.push(item);
                }
            }
        }
        Operation::Return(ret) => {
            execute_return(&mut result, reference, state.clone(), ret);
        }
    };

    result
}

fn execute_return(result: &mut Vec<(bool, State)>, reference: &HashMap<&str, &Workflow>, state: State, ret: &Return) {
    let batch = match ret {
        Return::Next(key) => {
            let next_workflow = *reference.get(key.as_str()).unwrap();
            execute(reference, &state, next_workflow, 0)
        }
        Return::Accept => { vec![(true, state)] }
        Return::Reject => { vec![(false, state)] }
    };
    for item in batch {
        result.push(item);
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

                map.insert(var, Range{ from: val, to: val.clone() });
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