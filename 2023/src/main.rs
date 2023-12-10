//

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod util;

const CURRENT_DAY: i32 = 10;

fn main() {
    println!("Hello Advent of Code 2023!");

    match CURRENT_DAY {
        1 => day01::day01(),
        2 => day02::day02(),
        3 => day03::day03(),
        4 => day04::day04(),
        5 => day05::day05(),
        6 => day06::day06(),
        7 => day07::day07(),
        8 => day08::day08(),
        9 => day09::day09(),
        10 => day10::day10(),
        _ => println!("ups, no solution for day {CURRENT_DAY}"),
    }
}
