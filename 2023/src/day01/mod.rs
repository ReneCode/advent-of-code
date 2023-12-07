// day01

use std::collections::HashMap;

use crate::util::io;

pub fn day01() {
    println!("hello day 01");

    let lines = io::read_lines("./01.data").unwrap();

    let sum: i32 = lines.iter().map(|line| get_nr_from_digit(line)).sum();
    println!("Result A: {sum}");

    let sum: i32 = lines
        .iter()
        .map(|line| get_nr_from_digit_or_name(line))
        .sum();
    println!("Result B: {sum}");
}

fn get_nr_from_digit(line: &str) -> i32 {
    let first = line.chars().find(|c| c.is_digit(10)).unwrap();
    let last = line.chars().rev().find(|c| c.is_digit(10)).unwrap();

    let mut complete: String = String::new();
    complete.push(first);
    complete.push(last);
    let nr = complete.parse().unwrap();
    nr
}

fn get_nr_from_digit_or_name(line: &str) -> i32 {
    let mut str_to_val: HashMap<String, i32> = HashMap::new();
    str_to_val.insert("0".to_string(), 0);
    str_to_val.insert("1".to_string(), 1);
    str_to_val.insert("2".to_string(), 2);
    str_to_val.insert("3".to_string(), 3);
    str_to_val.insert("4".to_string(), 4);
    str_to_val.insert("5".to_string(), 5);
    str_to_val.insert("6".to_string(), 6);
    str_to_val.insert("7".to_string(), 7);
    str_to_val.insert("8".to_string(), 8);
    str_to_val.insert("9".to_string(), 9);
    str_to_val.insert("one".to_string(), 1);
    str_to_val.insert("two".to_string(), 2);
    str_to_val.insert("three".to_string(), 3);
    str_to_val.insert("four".to_string(), 4);
    str_to_val.insert("five".to_string(), 5);
    str_to_val.insert("six".to_string(), 6);
    str_to_val.insert("seven".to_string(), 7);
    str_to_val.insert("eight".to_string(), 8);
    str_to_val.insert("nine".to_string(), 9);

    let first = get_val(line, &str_to_val);

    // now revert all for getting the last
    let rev_line = revert_str(line);
    let mut rev_str_to_val: HashMap<String, i32> = HashMap::new();
    for (k, v) in str_to_val.iter() {
        let rev_k = revert_str(k.as_str());
        rev_str_to_val.insert(rev_k, *v);
    }
    let last = get_val(rev_line.as_str(), &rev_str_to_val);

    let result = first * 10 + last;
    result
}

fn get_val(line: &str, str_to_val: &HashMap<String, i32>) -> i32 {
    for idx in 0..line.len() {
        let search = &line[idx..];
        for (k, v) in str_to_val.iter() {
            if search.starts_with(k) {
                return *v;
            }
        }
    }
    0
}

fn revert_str(s: &str) -> String {
    let rev_s = s.chars().rev().collect::<String>();
    rev_s
}
