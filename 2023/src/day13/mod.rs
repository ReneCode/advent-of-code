// day13

use std::{iter, usize};

use itertools::Itertools;

use crate::util::{io, parse};

pub fn day13() {
    println!("hello day13");

    let lines = io::read_lines("./src/day13/13.data").unwrap();
    let one_line = lines.join("\n");
    let groups = one_line
        .split("\n\n")
        .map(|group_line| group_line.split('\n').collect_vec())
        .collect_vec();

    let result_a = groups
        .iter()
        .map(|group| get_mirror_value(&group))
        .sum::<usize>();
    println!("Result A {result_a}");
}

fn get_mirror_value(group: &[&str]) -> usize {
    // println!("{:?}", group);
    if let Some(row_idx) = find_horizontal_mirror(group) {
        return (row_idx + 1) * 100;
    } else {
        let turned_group = turn_pattern(group);
        let tg = turned_group.iter().map(|r| r.as_str()).collect_vec();
        if let Some(col_idx) = find_horizontal_mirror(&tg) {
            return col_idx + 1;
        }
    }
    0
}

fn turn_pattern(group: &[&str]) -> Vec<String> {
    let len_row = group[0].len();
    let mut result: Vec<String> = iter::repeat(String::new()).take(len_row).collect_vec();
    for idx in 0..len_row {
        for row in group {
            let c = row.chars().nth(idx).unwrap();
            result.get_mut(idx).unwrap().push(c);
        }
    }

    result
}

fn find_horizontal_mirror(group: &[&str]) -> Option<usize> {
    let len = group.len();
    for check_row in 0..group.len() - 1 {
        let mut is_same = true;
        let mut start = check_row;
        let mut oposite = start + 1;
        while is_same {
            is_same = group[start] == group[oposite];
            if is_same {
                if start > 0 && oposite + 1 < len {
                    // continue check next line
                    start -= 1;
                    oposite += 1;
                } else {
                    break;
                }
            }
        }
        if is_same {
            return Some(check_row);
        }
    }
    None
}

#[cfg(test)]
mod test_day13 {
    use crate::day13::{find_horizontal_mirror, turn_pattern};

    #[test]
    fn test_find_horizontal_mirror() {
        assert_eq!(find_horizontal_mirror(&vec!["A", "B"]), None);
        assert_eq!(find_horizontal_mirror(&vec!["A", "A"]), Some(0));
        assert_eq!(find_horizontal_mirror(&vec!["A", "B", "B"]), Some(1));
        assert_eq!(find_horizontal_mirror(&vec!["A", "B", "B", "C"]), None);
        assert_eq!(find_horizontal_mirror(&vec!["A", "B", "B", "A"]), Some(1));
        assert_eq!(
            find_horizontal_mirror(&vec!["A", "B", "C", "C", "B"]),
            Some(2)
        );
        assert_eq!(
            find_horizontal_mirror(&vec!["A", "B", "B", "A", "B"]),
            Some(1)
        );
    }

    #[test]
    fn test_turn_pattern() {
        assert_eq!(
            turn_pattern(&vec!["ABC", "XYZ"]),
            vec!["AX".to_string(), "BY".to_string(), "CZ".to_string()]
        );

        assert_eq!(
            turn_pattern(&vec!["ABC", "XYZ", "UVW"]),
            vec!["AXU".to_string(), "BYV".to_string(), "CZW".to_string()]
        );
    }
}
