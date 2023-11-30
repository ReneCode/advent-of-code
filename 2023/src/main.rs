//

mod day01;
mod day02;
mod day03;
mod util;

const CURRENT_DAY: i32 = 1;

fn main() {
    println!("Hello Advent of Code 2020!");

    match CURRENT_DAY {
        1 => day01::day01(),
        2 => day02::day02(),
        3 => day03::day03(),
        _ => println!("ups, no solution for day {CURRENT_DAY}"),
    }
}
