// day02

use std::collections::HashMap;

use itertools::Itertools;

use crate::util::io;

#[derive(Debug)]
struct Game {
    id: i32,
    cube_sets: Vec<HashMap<String, i32>>,
}

impl Game {
    fn new(id: i32) -> Self {
        Game {
            id: id,
            cube_sets: Vec::new(),
        }
    }

    fn is_valid(&self, cube_limits: &HashMap<String, i32>) -> bool {
        for (key_limit, value_limit) in cube_limits {
            for cube_set in self.cube_sets.iter() {
                if let Some(cube_count) = cube_set.get(key_limit) {
                    if cube_count > value_limit {
                        return false;
                    }
                }
            }
        }
        true
    }
}

pub fn day02() {
    println!("hello day02");

    let lines = io::read_lines("./src/day02/02.data").unwrap();

    let mut games: Vec<Game> = Vec::new();
    for line in lines {
        let game = parse_game(&line);
        // println!(">> game {:?}", game);
        games.push(game);
    }

    let mut cube_limits: HashMap<String, i32> = HashMap::new();
    cube_limits.insert("red".to_string(), 12);
    cube_limits.insert("green".to_string(), 13);
    cube_limits.insert("blue".to_string(), 14);
    let count_valid_games: i32 = games
        .iter()
        .filter(|game| game.is_valid(&cube_limits))
        .map(|game| game.id)
        .sum();
    println!("Result A: {count_valid_games}");
}

fn parse_game(line: &str) -> Game {
    let tok = line.split(':').map(|l| l.trim()).collect_vec();
    let t_first = tok[0].split(' ').map(|l| l.trim()).collect_vec();
    let id: i32 = t_first[1].parse().unwrap();

    let mut game = Game::new(id);
    let t_cube_sets = tok[1].split(';').map(|l| l.trim()).collect_vec();
    for cube_sets in t_cube_sets {
        let mut hash_cube: HashMap<String, i32> = HashMap::new();
        let t_cubes = cube_sets.split(',').map(|l| l.trim()).collect_vec();
        for cubes in t_cubes {
            let tok = cubes.split(' ').map(|l| l.trim()).collect_vec();
            let count = tok[0].parse().unwrap();
            let name = tok[1];
            hash_cube.insert(name.to_string(), count);
        }
        game.cube_sets.push(hash_cube)
    }
    game
}
