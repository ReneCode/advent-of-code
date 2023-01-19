use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn read_lines(filename: &str) -> io::Lines<BufReader<File>> {
    let file = File::open(filename).unwrap();
    return io::BufReader::new(file).lines();
}

fn main() {
    println!("Hello, day01");

    let mut total_fuel_value: f32 = 0.0;
    let lines = read_lines("./01.data");
    for line in lines {
        match line.unwrap().parse::<f32>() {
            Ok(val) => {
                let fuel_value = ((val / 3.0) - 0.5).round() - 2.0;
                total_fuel_value = total_fuel_value + fuel_value;
                // println!("{} / {}", val, result);
            }
            Err(_) => {}
        }
    }
    println!("total fuel value {}", total_fuel_value);
}
