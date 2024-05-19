// day09

use core::num;

use crate::util::io;

pub fn day09() {
    let lines = io::read_lines("09.data").unwrap();

    let numbers: Vec<i64> = lines
        .iter()
        .map(|line| line.parse::<i64>().unwrap())
        .collect();

    let invalid_number = find_invalid_number(&numbers, 25).unwrap();
    println!("A: Invalid number: {}", invalid_number);

    let encryption_weakness = find_encryption_weakness(&numbers, invalid_number).unwrap();
    println!("B: Encryption weakness: {}", encryption_weakness);
}

fn find_encryption_weakness(numbers: &Vec<i64>, invalid_number: i64) -> Option<i64> {
    for i in 0..numbers.len() {
        let mut sum = numbers[i];
        for j in i + 1..numbers.len() {
            sum += numbers[j];
            if sum == invalid_number {
                let min = numbers[i..j].iter().min().unwrap();
                let max = numbers[i..j].iter().max().unwrap();
                let result = min + max;
                return Some(result);
            }
        }
    }
    return None;
}

fn find_invalid_number(numbers: &Vec<i64>, preamble_len: usize) -> Option<i64> {
    let mut start_index = 0;
    for index in preamble_len..numbers.len() {
        let preamble_numbers = &numbers[start_index..start_index + preamble_len];
        start_index += 1;
        let check_number = numbers[index];

        if !can_be_build_from_preamble(preamble_numbers, check_number) {
            return Some(check_number);
        }
    }
    return None;
}

fn can_be_build_from_preamble(preamble: &[i64], check_number: i64) -> bool {
    for i in 0..preamble.len() {
        for j in i..preamble.len() {
            if preamble[i] + preamble[j] == check_number {
                return true;
            }
        }
    }
    return false;
}
