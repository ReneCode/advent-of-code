// day20

use std::{collections::HashMap, str::FromStr};

use itertools::Itertools;

use crate::util::{io, parse};

const LOW: char = 'L';
const HIGH: char = 'H';
const EMPTY: char = ' ';

#[derive(PartialEq)]
enum ModuleType {
    FLIPFLOP,
    CONJUNCTION,
    BROADCAST,
}

struct Module {
    module_type: ModuleType,
    destinations: Vec<String>,
    inputs: Vec<String>,
}

pub fn day20() {
    println!("hello day20");

    let lines = io::read_lines("./src/day20/20.data").unwrap();

    let mut modules = read_modules(&lines);

    // let (total_low, total_high) = part_a(&modules);
    // println!(
    //     "Result A: {total_low} {total_high} -> {}",
    //     total_low * total_high
    // );

    part_b(&modules);
}

fn part_b(modules: &HashMap<String, Module>) {
    let mut current_values: HashMap<String, char> = HashMap::new();
    for (name, module) in modules.iter() {
        current_values.insert(name.clone(), LOW);
    }

    let mut round = 0;
    loop {
        round += 1;
        let (l, h) = part_round(modules, &mut current_values);
        if (l == 0 && h == 0) {
            println!("Result B: {round}");
            break;
        }
        if round % 10000 == 0 {
            println!("round: {round}");
        }
    }
}

fn part_a(modules: &HashMap<String, Module>) -> (usize, usize) {
    let mut current_values: HashMap<String, char> = HashMap::new();
    for (name, module) in modules.iter() {
        current_values.insert(name.clone(), LOW);
    }

    let mut total_low = 0;
    let mut total_high = 0;
    for i in 0..1000 {
        let (low_count, high_count) = part_round(modules, &mut current_values);

        println!("Result A: {low_count} {high_count}");
        total_low += low_count;
        total_high += high_count;
    }
    (total_low, total_high)
}

// fn part_b(modules: &HashMap<String, Module>) -> (usize, usize) {
//     let mut current_values: HashMap<String, char> = HashMap::new();
//     for (name, module) in modules.iter() {
//         current_values.insert(name.clone(), LOW);
//     }

//     let mut total_low = 0;
//     let mut total_high = 0;
//     for i in 0..10000 {
//         let (low_count, high_count) = part_a_round(modules, &mut current_values);

//         println!("Result B: {i}");
//         total_low += low_count;
//         total_high += high_count;
//     }
//     (total_low, total_high)
// }

fn part_round(
    modules: &HashMap<String, Module>,
    current_values: &mut HashMap<String, char>,
) -> (usize, usize) {
    let mut inputs: Vec<(String, char)> = Vec::new();
    inputs.push(("broadcaster".to_string(), LOW));

    let mut low_count = 0;
    let mut high_count = 0;
    while inputs.len() > 0 {
        low_count += inputs.iter().filter(|(_, p)| *p == LOW).count();
        high_count += inputs.iter().filter(|(_, p)| *p == HIGH).count();

        let work = inputs.clone();
        inputs.clear();
        for (name, input) in work {
            if name == "output" {
                continue;
            }

            if name == "rx" {
                if input == HIGH {
                    continue;
                } else {
                    return (0, 0);
                }
            }

            let module = modules.get(&name).unwrap();
            match module.module_type {
                ModuleType::BROADCAST => {
                    for child in module.destinations.iter() {
                        inputs.push((child.clone(), input));
                    }
                }
                ModuleType::FLIPFLOP => {
                    match input {
                        HIGH => {} // ignore high
                        LOW => {
                            let cur_val = current_values.get(&name).unwrap();
                            let output = match *cur_val {
                                HIGH => LOW,
                                LOW => HIGH,
                                _ => panic!("bad current value on flipflop"),
                            };
                            current_values.insert(name.clone(), output);
                            for child in module.destinations.iter() {
                                inputs.push((child.clone(), output));
                            }
                        }
                        _ => panic!("bad input for flipflop"),
                    }
                }

                ModuleType::CONJUNCTION => {
                    let all_inputs = module
                        .inputs
                        .iter()
                        .map(|n| {
                            let val = current_values.get(n).unwrap();
                            val
                            // let input_module = modules.get(n).unwrap();
                            // input_module.current_val
                        })
                        .collect_vec();
                    let mut output = HIGH;

                    if all_inputs.iter().all(|i| **i == HIGH) {
                        output = LOW;
                    }
                    current_values.insert(name.clone(), output);

                    for child in module.destinations.iter() {
                        inputs.push((child.clone(), output));
                    }
                }
            }
        }
    }
    (low_count, high_count)
}

fn read_modules(lines: &Vec<String>) -> HashMap<String, Module> {
    let mut modules: HashMap<String, Module> = HashMap::new();

    for line in lines {
        let (mut name, mut dest) = line.split_once("->").unwrap();

        name = name.trim();
        dest = dest.trim();

        let all_dest = parse::to_str(dest, ',')
            .iter()
            .map(|s| s.to_string())
            .collect_vec();

        if name == "broadcaster" {
            let broadcaster = Module {
                module_type: ModuleType::BROADCAST,
                destinations: all_dest,
                inputs: Vec::new(),
            };
            modules.insert(name.to_string(), broadcaster);
        } else {
            let prefix = name.chars().nth(0).unwrap();
            name = &name[1..];
            match prefix {
                '%' => {
                    // flipflop
                    let flipflop = Module {
                        module_type: ModuleType::FLIPFLOP,
                        destinations: all_dest,
                        inputs: Vec::new(),
                    };
                    modules.insert(name.to_string(), flipflop);
                }
                '&' => {
                    // conjunction
                    let conjuction = Module {
                        module_type: ModuleType::CONJUNCTION,
                        destinations: all_dest,
                        inputs: Vec::new(),
                    };
                    modules.insert(name.to_string(), conjuction);
                }
                _ => panic!("bad name prefix"),
            }
        }
    }

    // set input names for conjuction modules
    let mut conjuction_inputs: HashMap<String, Vec<String>> = HashMap::new();

    for (name, module) in modules.iter() {
        if module.module_type == ModuleType::CONJUNCTION {
            conjuction_inputs.insert(name.clone(), Vec::new());
        }
    }

    for (name, module) in modules.iter() {
        for out in module.destinations.iter() {
            if let Some(inputs) = conjuction_inputs.get_mut(out) {
                inputs.push(name.clone());
            }
        }
    }

    for (name, inputs) in conjuction_inputs.iter() {
        if let Some(module) = modules.get_mut(name) {
            module.inputs = inputs.clone();
        } else {
            panic!("conjunction module not found")
        }
    }

    modules
}
