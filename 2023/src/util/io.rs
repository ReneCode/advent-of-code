use std::fs::File;
use std::io::{self, BufRead};

pub fn read_lines(filename: &str) -> Option<Vec<String>> {
    let file = match File::open(filename) {
        Err(_) => {
            println!("can't open file {}", filename);
            return None;
        }
        Ok(f) => f,
    };
    let reader = io::BufReader::new(file);

    let r: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();
    Some(r)

    // let mut result = Vec::new();
    // for line in reader.lines() {
    //     if let Ok(l) = line {
    //         result.push(l)
    //     }
    // }
    // Some(result)
}
