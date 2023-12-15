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
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod util;

const CURRENT_DAY: i32 = 15;

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
        11 => day11::day11(),
        12 => day12::day12(),
        13 => day13::day13(),
        14 => day14::day14(),
        15 => day15::day15(),
        _ => println!("ups, no solution for day {CURRENT_DAY}"),
    }
}
