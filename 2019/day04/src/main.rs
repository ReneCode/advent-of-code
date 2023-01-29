use std::collections::HashMap;

fn main() {
    println!("Hello, day04!");

    const input: &str = "156218-652527";

    let from_to: Vec<i32> = input
        .split("-")
        .map(|s| s.parse::<i32>().unwrap() as i32)
        .collect();

    let result_1 = count_valid_numbers(&from_to, check_valid_password_part_1);
    println!("part-1 total: {}", result_1);

    let result_2 = count_valid_numbers(&from_to, check_valid_password_part_2);
    println!("part-2 total: {}", result_2);
}

fn count_valid_numbers(from_to: &Vec<i32>, check_fn: fn(i32) -> bool) -> i32 {
    let mut nr = from_to[0];
    let mut total = 0;
    while nr <= from_to[1] {
        if check_fn(nr) {
            total += 1;
        }
        nr += 1;
    }
    total
}

fn check_valid_password_part_1(nr: i32) -> bool {
    let numbers: Vec<i32> = nr
        .to_string()
        .chars()
        .map(|c| c as i32 - '0' as i32)
        .collect();

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

#[test]
fn test_check_part2() {
    assert_eq!(true, check_valid_password_part_2(112233));
    assert_eq!(false, check_valid_password_part_2(123444));
    assert_eq!(true, check_valid_password_part_2(111122));
}

fn check_valid_password_part_2(nr: i32) -> bool {
    let numbers: Vec<i32> = nr
        .to_string()
        .chars()
        .map(|c| c as i32 - '0' as i32)
        .collect();

    let mut prev_nr = -1;
    let mut counts = HashMap::new();
    for nr in numbers {
        if nr < prev_nr {
            return false;
        }
        if let Some(count) = counts.get_mut(&nr) {
            *count += 1;
        } else {
            counts.insert(nr, 1);
        }
        prev_nr = nr;
    }
    let mut ok = false;
    for (k, v) in counts {
        if v == 2 {
            ok = true;
        }
    }

    ok
}
