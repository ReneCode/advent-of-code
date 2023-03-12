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

struct Solver<'a> {
    recipes: &'a HashMap<String, Recipe>,
    stock: HashMap<String, i32>,
    count_oer: i32,
}

impl<'a> Solver<'_> {
    fn new(recipes: &HashMap<String, Recipe>) -> Solver {
        Solver {
            recipes,
            stock: HashMap::new(),
            count_oer: 0,
        }
    }
    fn solve(&mut self) -> i32 {
        self.stock.clear();
        self.count_oer = 0;
        self.build("FUEL", 1);
        let stock = &self.stock;
        println!("stock {stock:?}");

        self.count_oer
    }

    fn build(&mut self, name: &str, count: i32) {
        println!("build {count} {name}");
        if let Some(recipe) = self.recipes.get(name) {
            let count_recipe = self.calc_count_recipe(&recipe.target, count);

            let target = &recipe.target;
            let not_needed = count_recipe * target.count - count;
            if not_needed > 0 {
                self.add_stock(&target.name, not_needed);
            }

            // if not_needed > 0 {
            //     if let Some(entry) = self.stock.get_mut(target.name.as_str()) {
            //         *entry += not_needed;
            //     } else {
            //         self.stock.insert(target.name, not_needed);
            //     }
            // }

            for source in recipe.sources.iter() {
                if source.name == NAME_ORE {
                    let count_oer = source.count * count_recipe;
                    println!("need oer {count_oer}");
                    self.count_oer += source.count * count_recipe;
                } else {
                    let mut count_source = count_recipe;
                    if let Some(entry) = self.stock.get_mut(source.name.as_str()) {
                        while *entry >= source.count {
                            *entry -= source.count;
                            count_source -= 1;
                        }
                    }
                    if count_source > 0 {
                        self.build(source.name.as_str(), source.count * count_source);
                    }
                }
            }
        }
    }

    fn calc_count_recipe(&mut self, target: &CountName, total_target_needed: i32) -> i32 {
        // total_target_needed = 8
        // recipe.target.count = 7
        // stock = 4
        // -> 1
        let mut in_stock = 0;
        let mut count_recipe = 1;
        while count_recipe * target.count < total_target_needed {
            count_recipe += 1;
        }

        if let Some(stock) = self.stock.get_mut(target.name.as_str()) {
            if (count_recipe - 1) * target.count + *stock >= total_target_needed {
                count_recipe -= 1;
                *stock = total_target_needed - (count_recipe * target.count);
            }
        }

        count_recipe
    }

    fn add_stock(&mut self, name: &String, count: i32) {
        if count > 0 {
            if let Some(entry) = self.stock.get_mut(name) {
                *entry += count;
            } else {
                self.stock.insert(name.clone(), count);
            }

            let result = self.stock.get(name).unwrap();
            println!("in stock: {name} {result}");
        }
    }
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
    let mut solver = Solver::new(recipes);
    let result = solver.solve();
    println!("part1 {result}");

    return;

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
