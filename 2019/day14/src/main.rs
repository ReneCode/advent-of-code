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
    if count_needed >= count_target {
        let rest = count_needed % count_target;
        if rest > 0 {
            if rest <= stock {
                opt_stock = Some(stock - rest);
            } else {
                count_recipe += 1;
                opt_stock = Some(stock + count_target - rest)
            }
        }
    } else {
        if stock >= count_needed {
            count_recipe = 0;
            opt_stock = Some(stock - count_needed)
        } else {
            count_recipe = 1;
            let rest = count_target - count_needed;
            opt_stock = Some(stock + rest)
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
#[test]
fn test_5() {
    let (count_recipe, opt_stock) = calc_count_recipe_and_stock(7, 4, 1);
    assert_eq!(count_recipe, 1);
    assert_eq!(opt_stock, Some(4));
}
#[test]
fn test_5a() {
    let (count_recipe, opt_stock) = calc_count_recipe_and_stock(11, 7, 9);
    assert_eq!(count_recipe, 0);
    assert_eq!(opt_stock, Some(2));
}
#[test]
fn test_6() {
    let (count_recipe, opt_stock) = calc_count_recipe_and_stock(11, 7, 5);
    assert_eq!(count_recipe, 1);
    assert_eq!(opt_stock, Some(9));
}

fn get_data(filename: &str) -> Option<HashMap<String, Recipe>> {
    if let Some(lines) = io::get_lines(filename) {
        let l: Vec<&str> = lines.iter().map(|s| s.as_str()).collect();
        parse_lines(l)
    } else {
        None
    }
}

fn parse_lines(lines: Vec<&str>) -> Option<HashMap<String, Recipe>> {
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
}

fn main() {
    println!("Hello, day14!");
    if let Some(recipes) = get_data("./14.data") {
        part_1(&recipes);
    }
}

fn part_1(recipes: &HashMap<String, Recipe>) {
    let mut solver = Solver::new(recipes);
    let result = solver.solve();
    println!("part1 {result}");
}

#[test]
fn test_p1_1() {
    let input = r#"10 ORE => 10 A
    1 ORE => 1 B
    7 A, 1 B => 1 C
    7 A, 1 C => 1 D
    7 A, 1 D => 1 E
    7 A, 1 E => 1 FUEL"#;
    let lines: Vec<&str> = input.split('\n').collect();
    let recipes = parse_lines(lines).unwrap();
    let mut solver = Solver::new(&recipes);
    assert_eq!(solver.solve(), 31);
}

#[test]
fn test_p1_2() {
    let input = r#"9 ORE => 2 A
    8 ORE => 3 B
    7 ORE => 5 C
    3 A, 4 B => 1 AB
    5 B, 7 C => 1 BC
    4 C, 1 A => 1 CA
    2 AB, 3 BC, 4 CA => 1 FUEL"#;
    let lines: Vec<&str> = input.split('\n').collect();
    let recipes = parse_lines(lines).unwrap();
    let mut solver = Solver::new(&recipes);
    assert_eq!(solver.solve(), 165);
}

#[test]
fn test_p1_3() {
    let input = r#"157 ORE => 5 NZVS
    165 ORE => 6 DCFZ
    44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
    12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
    179 ORE => 7 PSHF
    177 ORE => 5 HKGWZ
    7 DCFZ, 7 PSHF => 2 XJWVT
    165 ORE => 2 GPVTF
    3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT"#;
    let lines: Vec<&str> = input.split('\n').collect();
    let recipes = parse_lines(lines).unwrap();
    let mut solver = Solver::new(&recipes);
    assert_eq!(solver.solve(), 13312);
}

#[test]
fn test_p1_4() {
    let input = r#"2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
    17 NVRVD, 3 JNWZP => 8 VPVL
    53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
    22 VJHF, 37 MNCFX => 5 FWMGM
    139 ORE => 4 NVRVD
    144 ORE => 7 JNWZP
    5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
    5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
    145 ORE => 6 MNCFX
    1 NVRVD => 8 CXFTF
    1 VJHF, 6 MNCFX => 4 RFSQX
    176 ORE => 6 VJHF"#;
    let lines: Vec<&str> = input.split('\n').collect();
    let recipes = parse_lines(lines).unwrap();
    let mut solver = Solver::new(&recipes);
    assert_eq!(solver.solve(), 180697);
}

#[test]
fn test_p1_5() {
    let input = r#"171 ORE => 8 CNZTR
    7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
    114 ORE => 4 BHXH
    14 VRPVC => 6 BMBT
    6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
    6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
    15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
    13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
    5 BMBT => 4 WPTQ
    189 ORE => 9 KTJDG
    1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
    12 VRPVC, 27 CNZTR => 2 XDBXC
    15 KTJDG, 12 BHXH => 5 XCVML
    3 BHXH, 2 VRPVC => 7 MZWV
    121 ORE => 7 VRPVC
    7 XCVML => 6 RJRHP
    5 BHXH, 4 VRPVC => 5 LTCX"#;
    let lines: Vec<&str> = input.split('\n').collect();
    let recipes = parse_lines(lines).unwrap();
    let mut solver = Solver::new(&recipes);
    assert_eq!(solver.solve(), 2210736);
}
