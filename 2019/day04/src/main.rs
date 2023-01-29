use core::num;

fn main() {
    println!("Hello, day04!");

    const input: &str = "156218-652527";

    let from_to: Vec<i32> = input
        .split("-")
        .map(|s| s.parse::<i32>().unwrap() as i32)
        .collect();

    part_1(from_to);
}

fn part_1(from_to: Vec<i32>) {
    let mut nr = from_to[0];
    let mut total = 0;
    while nr <= from_to[1] {
        if check_valid_password(nr) {
            total += 1;
        }
        nr += 1;
    }
    println!("part-1 total: {}", total);
}

fn check_valid_password(nr: i32) -> bool {
    let numbers: Vec<i32> = nr
        .to_string()
        .chars()
        .map(|c| c as i32 - '0' as i32)
        .collect();

    let ok = true;
    let mut prev_nr = -1;
    let mut duplicate = false;
    for nr in numbers {
        if nr < prev_nr {
            return false;
        }
        if nr == prev_nr {
            duplicate = true;
        }
        prev_nr = nr;
    }
    if !duplicate {
        return false;
    }

    true
}
