use util::io;

use std::{collections::HashMap, convert::From};

const NAME_ORE: &str = "ORE";

struct CountName {
    count: i32,
    name: String,
}

impl From<&str> for CountName {
    fn from(line: &str) -> Self {
        let token: Vec<&str> = line.trim().split(' ').collect();
        let count = token[0].parse::<i32>().unwrap();
        let name = String::from(token[1]);
        CountName { count, name }
    }
}

struct Recipe {
    sources: Vec<CountName>,
    target: CountName,
}

fn get_data(filename: &str) -> Option<HashMap<String, Recipe>> {
    if let Some(lines) = io::get_lines(filename) {
        let mut result: HashMap<String, Recipe> = HashMap::new();
        for line in lines {
            let token: Vec<&str> = line.split(" => ").collect();

            let sources: Vec<CountName> = token[0]
                .split(',')
                .map(|l| l.trim())
                .map(CountName::from)
                .collect();

            let target = CountName::from(token[1]);
            let name = target.name.clone();
            let recipe = Recipe { sources, target };
            result.insert(name, recipe);
        }

        Some(result)
    } else {
        None
    }
}

fn main() {
    println!("Hello, day14!");
    if let Some(recipes) = get_data("./14-example.data") {
        part_1(&recipes);
    }
}

fn part_1(recipes: &HashMap<String, Recipe>) {
    let mut count_oer = 0;
    let mut targets: Vec<(&str, i32)> = Vec::new();
    let name = "FUEL";
    let mut stock: HashMap<&str, i32> = HashMap::new();
    targets.push((name, 1));
    while !targets.is_empty() {
        let (target, target_count) = targets.remove(0);

        println!("get target {target} {target_count}");
        if let Some(recipe) = recipes.get(target) {
            let mut count_recipe = 1;
            let recipe_target_count = recipe.target.count;
            while count_recipe * recipe_target_count < target_count {
                count_recipe += 1;
            }
            let not_needed = count_recipe * recipe_target_count - target_count;
            if not_needed > 0 {
                if let Some(entry) = stock.get_mut(target) {
                    *entry += not_needed;
                } else {
                    stock.insert(target, not_needed);
                }
                println!(">> add stock {target}");
            }

            for source in recipe.sources.iter() {
                if source.name == NAME_ORE {
                    count_oer += source.count * count_recipe;
                } else {
                    let mut count_for_this_source = count_recipe;
                    println!(">> look {}", source.name);
                    if let Some(entry) = stock.get_mut(source.name.as_str()) {
                        while *entry >= source.count {
                            *entry -= source.count;
                            count_for_this_source -= 1;
                        }
                    }

                    targets.push((source.name.as_str(), source.count * count_for_this_source));
                }
            }
        }
    }

    println!("stock {stock:?}");
    println!("part1 count ore: {count_oer}");
}
