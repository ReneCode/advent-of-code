// day02

use crate::util;

#[derive(Debug)]
struct Policy {
    letter: char,
    min_count: usize,
    max_count: usize,
}

impl std::convert::From<&str> for Policy {
    fn from(value: &str) -> Self {
        let token = value.split(" ").map(|t| t.trim()).collect::<Vec<&str>>();
        let count_token = token[0]
            .split("-")
            .map(|t| t.parse().unwrap())
            .collect::<Vec<usize>>();
        Policy {
            letter: token[1].chars().nth(0).unwrap(),
            min_count: count_token[0],
            max_count: count_token[1],
        }
    }
}

impl Policy {
    fn is_valid(&self, password: &str) -> bool {
        let count = password.chars().filter(|c| *c == self.letter).count();
        self.min_count <= count && count <= self.max_count
    }
}

pub fn day02() {
    println!("hello day02");

    let lines = util::io::read_lines("./02.data").unwrap();
    let result: usize = lines
        .iter()
        .map(|line| {
            let token = line.split(":").map(|l| l.trim()).collect::<Vec<&str>>();
            let pol = Policy::from(token[0]);
            let valid = pol.is_valid(token[1]);
            // println!("pol {:?}", pol);
            if valid {
                1
            } else {
                0
            }
        })
        .sum();

    println!("result {}", result);
}
