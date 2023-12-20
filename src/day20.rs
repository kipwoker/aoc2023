#![allow(unused_variables)]

use std::collections::{HashMap, HashSet};
use crate::core::{lcm_of_vector, Solution};
use crate::day20::ModuleType::{Broadcast, Conjunction, FlipFlop};
use crate::day20::Pulse::{High, Low};

pub struct Day20 {}

impl Solution for Day20 {
    fn get_day(&self) -> &'static str {
        "20"
    }

    fn solve1(&self, input: String) -> String {
        let mut module_map = parse(input.as_str());
        let link = &mut module_map;

        let mut pulse_map = HashMap::from([
            (0, 0),
            (1, 0)
        ]);
        for _ in 0..1000 {
            send(link, &mut pulse_map, "mock_value", "broadcaster", Low);
        }

        println!("{pulse_map:?}");

        let x = pulse_map.get(&0).unwrap().clone() as u64;
        let y = pulse_map.get(&1).unwrap().clone() as u64;

        let result = x * y;

        result.to_string()
    }
    fn solve2(&self, input: String) -> String {
        let mut module_map = parse(input.as_str());

        let last = "rx".to_string();
        let (last_conjunction, last_conjunction_module) = module_map.iter().find(|(name, module)| module.output.contains(&last)).unwrap();
        let last_conjunction = last_conjunction.clone();
        let last_conjunction = last_conjunction.as_str();
        let mut end_modules = match &last_conjunction_module.module_type {
            Conjunction(input_map) => {
                input_map.keys().map(|x| x.clone()).collect::<HashSet<String>>()
            }
            _ => { unreachable!() }
        };

        println!("{end_modules:?}");

        let mut i = 1;
        let mut caught_iterations = HashSet::new();
        loop {
            send2(&mut module_map, &mut caught_iterations, &i, &mut end_modules, last_conjunction,"mock_value", "broadcaster", Low);

            if end_modules.is_empty() {
                let vec: Vec<i64> = caught_iterations.iter().map(|x| x.clone() as i64).collect();
                let total = lcm_of_vector(&vec).unwrap();
                break total.to_string()
            }

            i += 1;
        }
    }
}

fn send2(
    module_map: &mut HashMap<String, Module>,
    caught_iterations: &mut HashSet<u64>,
    iteration: &u64,
    end_modules: &mut HashSet<String>,
    last_conjunction: &str,
    sender: &str,
    receiver: &str,
    pulse: Pulse,
) {
    if let High = pulse {
        if receiver == last_conjunction && end_modules.contains(sender) {
            let mul = if iteration & 1 == 0 { iteration + 1 } else { iteration.clone() };
            println!("High {sender} {receiver} on {iteration} -> {mul}");
            caught_iterations.insert(mul);
            end_modules.remove(sender);
        }
    }

    if let Some(module) = module_map.get_mut(receiver) {
        let next = module.send_signal(sender, pulse);
        for (next_module_name, next_pulse) in next {
            send2(
                module_map,
                caught_iterations,
                iteration,
                end_modules,
                last_conjunction,
                receiver,
                next_module_name.as_str(),
                next_pulse,
            );
        }
    }
}

fn send(module_map: &mut HashMap<String, Module>, pulse_map: &mut HashMap<i32, i32>, sender: &str, receiver: &str, pulse: Pulse) {
    pulse_map.entry(if let Low = pulse { 0 } else { 1 }).and_modify(|v| *v = *v + 1);
    if let Some(module) = module_map.get_mut(receiver) {
        let next = module.send_signal(sender, pulse);
        for (next_module_name, next_pulse) in next {
            send(module_map, pulse_map, receiver, next_module_name.as_str(), next_pulse);
        }
    }
}

#[derive(Debug, Clone)]
enum Pulse {
    Low,
    High,
}

#[derive(Debug, Clone)]
enum ModuleType {
    FlipFlop(bool),
    Conjunction(HashMap<String, Pulse>),
    Broadcast,
}

#[derive(Debug, Clone)]
struct Module {
    name: String,
    output: Vec<String>,
    module_type: ModuleType,
}

impl Module {
    fn send_signal(&mut self, sender: &str, pulse: Pulse) -> Vec<(String, Pulse)> {
        match &mut self.module_type {
            FlipFlop(state) => {
                if let Low = pulse {
                    *state = !*state;
                    let new_pulse = if *state { High } else { Low };
                    self.output.iter().map(|x| (x.clone(), new_pulse.clone())).collect()
                } else {
                    Vec::new()
                }
            }
            Conjunction(state) => {
                state.entry(sender.to_string()).and_modify(|x| *x = pulse);
                let all_high = state.values().all(|x| if let High = x { true } else { false });
                let new_pulse = if all_high { Low } else { High };
                self.output.iter().map(|x| (x.clone(), new_pulse.clone())).collect()
            }
            Broadcast => {
                self.output.iter().map(|x| (x.clone(), pulse.clone())).collect()
            }
        }
    }
}

fn parse(input: &str) -> HashMap<String, Module> {
    let mut module_map = parse_init(input);
    let input_node_map = get_conjunction_input_nodes(&module_map);
    let link = &mut module_map;
    fill_conjunction_input(link, &input_node_map);
    module_map
}

fn parse_init(input: &str) -> HashMap<String, Module> {
    input.split("\n").map(|line| {
        let parts = line.split(" -> ").collect::<Vec<&str>>();
        let header = parts[0];
        let output = parts[1];
        let output: Vec<String> = output.split(", ").map(|x| x.to_string()).collect();
        let module_type = &header.chars().nth(0).unwrap();

        let module = match module_type {
            &'%' => {
                let name: String = header[1..].chars().collect();
                Module {
                    name,
                    output,
                    module_type: FlipFlop(false),
                }
            }
            &'&' => {
                let name: String = header[1..].chars().collect();
                Module {
                    name,
                    output,
                    module_type: Conjunction(HashMap::new()),
                }
            }
            &'b' => {
                let name = header.to_string();
                Module {
                    name,
                    output,
                    module_type: Broadcast,
                }
            }
            &_ => { unreachable!() }
        };

        (module.name.clone(), module)
    }).collect()
}

fn get_conjunction_input_nodes(map: &HashMap<String, Module>) -> HashMap<String, Vec<String>> {
    let mut result = HashMap::new();
    for (inn, module) in map {
        for out in &module.output {
            if let Some(next) = map.get(out.as_str()) {
                if let Conjunction(_) = &next.module_type {
                    result
                        .entry(out.clone())
                        .and_modify(|v: &mut Vec<String>| v.push(inn.clone()))
                        .or_insert_with(|| vec![inn.clone()]);
                }
            } else {
                println!("{out} not found as input");
            }
        }
    }

    result
}

fn fill_conjunction_input(module_map: &mut HashMap<String, Module>, input_map: &HashMap<String, Vec<String>>) {
    for (out, ins) in input_map {
        let module = module_map.get_mut(out.as_str()).unwrap();
        if let Conjunction(pulse_map) = &mut module.module_type {
            for inn in ins {
                pulse_map.insert(inn.clone(), Low);
            }
        }
    }
}