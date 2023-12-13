// day13

use std::{iter, usize};

use itertools::Itertools;

const ON: char = '#';
const OFF: char = '.';

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
        .map(|group| get_mirror_value(&group, false))
        .sum::<usize>();
    println!("Result A {result_a}");

    let result_b = groups
        .iter()
        .map(|group| {
            let val = get_mirror_value(&group, true);
            // println!("{val}   {:?}", group);
            val
        })
        .sum::<usize>();
    println!("Result B {result_b}");
}

fn get_mirror_value(group: &[&str], look_smudge: bool) -> usize {
    // println!("{:?}", group);
    if let Some(row_idx) = find_horizontal_mirror(group, look_smudge) {
        return (row_idx + 1) * 100;
    }
    let turned_group = turn_pattern(group);
    let tg = turned_group.iter().map(|r| r.as_str()).collect_vec();
    if let Some(col_idx) = find_horizontal_mirror(&tg, look_smudge) {
        return col_idx + 1;
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

fn find_horizontal_mirror(group: &[&str], look_smudge: bool) -> Option<usize> {
    let len = group.len();
    for check_row in 0..group.len() - 1 {
        let mut found_smudge = false;
        let mut is_same = true;
        let mut start = check_row;
        let mut oposite = start + 1;
        while is_same {
            is_same = group[start] == group[oposite];
            if look_smudge {
                if !is_same && !found_smudge {
                    if same_with_sumdge(group[start], group[oposite]) {
                        is_same = true;
                        found_smudge = true;
                    }
                }
            }
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
            if !look_smudge {
                return Some(check_row);
            }
            if look_smudge && found_smudge {
                return Some(check_row);
            }
        }
    }
    None
}

fn same_with_sumdge(start: &str, oposite: &str) -> bool {
    let mut found_smudge = false;
    let mut is_same = true;
    for idx in 0..start.len() {
        let c1 = start.chars().nth(idx).unwrap();
        let c2 = oposite.chars().nth(idx).unwrap();
        is_same = c1 == c2;
        if !is_same && !found_smudge {
            if c1 == ON && c2 == OFF || c1 == OFF && c2 == ON {
                is_same = true;
                found_smudge = true;
            }
        }
        if !is_same {
            break;
        }
    }

    is_same
}

#[cfg(test)]
mod test_day13 {
    use crate::day13::{find_horizontal_mirror, same_with_sumdge, turn_pattern};

    #[test]
    fn test_find_horizontal_mirror() {
        assert_eq!(find_horizontal_mirror(&vec!["A", "B"], false), None);
        assert_eq!(find_horizontal_mirror(&vec!["A", "B", "B"], false), Some(1));
        assert_eq!(find_horizontal_mirror(&vec!["A", "A"], false), Some(0));
        assert_eq!(find_horizontal_mirror(&vec!["A", "B", "B"], false), Some(1));
        assert_eq!(
            find_horizontal_mirror(&vec!["A", "B", "B", "C"], false),
            None
        );
        assert_eq!(
            find_horizontal_mirror(&vec!["A", "B", "B", "A"], false),
            Some(1)
        );
        assert_eq!(
            find_horizontal_mirror(&vec!["A", "B", "C", "C", "B"], false),
            Some(2)
        );
        assert_eq!(
            find_horizontal_mirror(&vec!["A", "B", "B", "A", "B"], false),
            Some(1)
        );
    }

    #[test]
    fn test_find_horizontal_mirror_smudge() {
        assert_eq!(
            find_horizontal_mirror(&vec![".", "B", "B", "#", "C"], true),
            Some(1)
        );

        assert_eq!(find_horizontal_mirror(&vec!["##.", "#.."], true), Some(0));
        assert_eq!(find_horizontal_mirror(&vec!["##.", "##."], true), None);

        // reset check smudge each try-round
        assert_eq!(
            find_horizontal_mirror(&vec!["....", "#.#.", "###.", ".##.", ".##."], true),
            None
        );

        assert_eq!(
            find_horizontal_mirror(&vec!["#", "A", "B", "B", "A", ".", "C"], true),
            Some(2)
        );

        assert_eq!(
            find_horizontal_mirror(
                &vec![".", "X", "#", "A", "B", "B", "A", ".", "X", "#", "C"],
                true
            ),
            None
        );

        assert_eq!(
            find_horizontal_mirror(
                &vec![".", "X", "#", "A", "B", "B", "A", ".", "X", ".", "C"],
                true
            ),
            Some(4)
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

    #[test]
    fn test_same_with_sumdge() {
        assert_eq!(same_with_sumdge("abc", "abc"), true);
        assert_eq!(same_with_sumdge("abc", "xyz"), false);
        assert_eq!(same_with_sumdge(".abc", "#abc"), true);
        assert_eq!(same_with_sumdge("abc#", "abc."), true);
        assert_eq!(same_with_sumdge("abc#.", "abc.#"), false);
    }
}
