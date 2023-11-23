//

mod day01;
mod util;

const CURRENT_DAY: i32 = 1;

fn main() {
    println!("Hello Advent of Code 2020!");

    match CURRENT_DAY {
        1 => day01::day01(),
        _ => println!("ups, no solution for day {CURRENT_DAY}"),
    }
}
