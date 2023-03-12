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
        if let Some(stock) = self.stock.get_mut(target.name.as_str()) {
            let (count_recipe, new_stock) =
                calc_count_recipe_and_stock(target.count, total_target_needed, *stock);

            if let Some(new_value) = new_stock {
                *stock = new_value;
            }

            count_recipe
        } else {
            let (count_recipe, new_stock) =
                calc_count_recipe_and_stock(target.count, total_target_needed, 0);
            if let Some(new_value) = new_stock {
                self.stock.insert(target.name.clone(), new_value);
            }
            count_recipe
        }
    }
}

fn calc_count_recipe_and_stock(
    count_target: i32,
    count_needed: i32,
    stock: i32,
) -> (i32, Option<i32>) {
    let mut count_recipe = count_needed / count_target;
    let mut opt_stock = None;
    let rest = count_needed % count_target;
    if rest > 0 {
        if rest <= stock {
            opt_stock = Some(stock - rest);
        } else {
            count_recipe += 1;
            opt_stock = Some(stock + count_target - rest)
        }
    }
    (count_recipe, opt_stock)
}

#[test]
fn test_1() {
    let (count_recipe, opt_stock) = calc_count_recipe_and_stock(2, 22, 0);
    assert_eq!(count_recipe, 11);
    assert_eq!(opt_stock, None);
}
#[test]
fn test_2() {
    let (count_recipe, opt_stock) = calc_count_recipe_and_stock(5, 7, 0);
    assert_eq!(count_recipe, 2);
    assert_eq!(opt_stock, Some(3));
}
#[test]
fn test_3() {
    let (count_recipe, opt_stock) = calc_count_recipe_and_stock(5, 7, 1);
    assert_eq!(count_recipe, 2);
    assert_eq!(opt_stock, Some(4));
}
#[test]
fn test_4() {
    let (count_recipe, opt_stock) = calc_count_recipe_and_stock(5, 7, 4);
    assert_eq!(count_recipe, 1);
    assert_eq!(opt_stock, Some(2));
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
}
