// day12

use std::usize;

use crate::util::{io, parse};

const EMPTY: char = '.';
const DAMAGED: char = '#';
const NOTKNOWN: char = '?';

pub fn day12() {
    println!("hello day12");

    let lines = io::read_lines("./src/day12/12-example.data").unwrap();

    // let result_a: usize = lines.iter().map(|line| part_a(line)).sum();
    // println!("Result A: {result_a}");

    let result_b: usize = lines
        .iter()
        .map(|line| {
            let tok = parse::to_str(line, ' ');
            let mut exp_line = tok[0].to_string();
            for _ in 1..5 {
                exp_line.push('?');
                exp_line.push_str(tok[0])
            }
            exp_line.push(' ');
            exp_line.push_str(tok[1]);
            for _ in 1..5 {
                exp_line.push(',');
                exp_line.push_str(tok[1])
            }

            return exp_line;
        })
        .map(|line| part_a(&line))
        .sum();
    println!("Result B: {result_b}");

    // for line in lines {
    //     let result = part_a(&line);
    //     println!("{result}    {}", line);
    // }
}

fn part_a(line: &str) -> usize {
    let tok: Vec<&str> = parse::to_str(line, ' ');
    let org_pattern = tok[0];
    let counts = parse::to_numbers::<usize>(tok[1], ',');

    let patterns = build_patterns(org_pattern, &counts);
    // .iter()
    // .filter(|pattern| is_valid_pattern(org_pattern, *pattern))
    // .cloned()
    // .collect_vec();

    println!("{line} {}", patterns.len());
    patterns.len()
    // println!("============ {:?} / {:?}", org_pattern, counts);
}

fn build_patterns(org_pattern: &str, counts: &[usize]) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();

    let counts_sum: usize = counts.iter().sum();
    let len = org_pattern.len();
    let count_gap = counts.len() + 1;
    let max_gap_len = len - counts_sum + 1 - (count_gap - 2);

    // create 0,1,1,1,0 vector.
    // first and last gap could be 0
    // but inside it has to be at least 1
    let mut gap_lengths = Vec::new();
    gap_lengths.push(0);
    for _ in 1..count_gap - 1 {
        gap_lengths.push(1);
    }
    gap_lengths.push(0);

    loop {
        let mut str = String::new();
        let mut ok = true;
        for idx in 0..count_gap {
            let gap_len = gap_lengths[idx];
            append_string(&mut str, EMPTY, gap_len);
            if idx < counts.len() {
                let fill_len = counts[idx];
                append_string(&mut str, DAMAGED, fill_len);
            }
            if str.len() > len {
                // || !is_valid_pattern(org_pattern, str.as_str()) {
                ok = false;
                break;
            }
        }
        if ok && str.len() == len && is_valid_pattern(org_pattern, str.as_str()) {
            result.push(str);
        }

        let mut finished = false;
        loop {
            if next_gaps(&mut gap_lengths, max_gap_len) {
                finished = true;
                break;
            }
            let complete_sum: usize = gap_lengths.iter().sum::<usize>() + counts_sum;
            if complete_sum == len {
                break;
            }
        }

        if finished {
            break;
        }
    }

    result
}

fn next_gaps(gap_lengths: &mut Vec<usize>, max_gap_len: usize) -> bool {
    let count_gap = gap_lengths.len();
    let mut finished = true;
    let right_idx = count_gap - 1;
    for idx in (0..count_gap).rev() {
        let gap_len = gap_lengths.get_mut(idx).unwrap();
        if *gap_len < max_gap_len {
            *gap_len += 1;
            finished = false;
            break;
        } else if *gap_len == max_gap_len {
            // the middle gaps will start at length 1
            // the outer gaps start at length 0
            if idx == 0 || idx == right_idx {
                *gap_len = 0;
            } else {
                *gap_len = 1;
            }
        }
    }
    finished
}

fn append_string(str: &mut String, c: char, count: usize) {
    // let app: String = iter::repeat(c).take(count).collect();
    // str.push_str(app.as_str())
    for _ in 0..count {
        str.push(c);
    }
}

fn is_valid_pattern(org_pattern: &str, check_pattern: &str) -> bool {
    for (i, c) in check_pattern.chars().enumerate() {
        let org_c = org_pattern.chars().nth(i).unwrap();
        if org_c != c && org_c != NOTKNOWN {
            return false;
        }
    }
    true
}

fn get_possible_patterns(count: usize, max_len: usize) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();

    for i in 0..max_len - count + 1 {
        let mut str = String::new();
        for _ in 0..i {
            str.push(EMPTY)
        }
        for _ in 0..count {
            str.push(DAMAGED)
        }
        result.push(str);
    }
    result
}

#[cfg(test)]
mod test {
    use crate::day12::{build_patterns, get_possible_patterns, is_valid_pattern};

    #[test]
    fn test_build_patterns() {
        assert_eq!(
            build_patterns("???", &vec![1]),
            vec!["#..".to_string(), ".#.".to_string(), "..#".to_string()]
        );

        assert_eq!(
            build_patterns("?????", &vec![1, 2]),
            vec![
                "#.##.".to_string(),
                "#..##".to_string(),
                ".#.##".to_string()
            ]
        );

        assert_eq!(
            build_patterns("??????", &vec![3, 1]),
            vec![
                "###.#.".to_string(),
                "###..#".to_string(),
                ".###.#".to_string()
            ]
        );
    }

    #[test]
    fn test_get_possible_patterns() {
        assert_eq!(
            get_possible_patterns(3, 4),
            vec!["###".to_string(), ".###".to_string()]
        );

        assert_eq!(
            get_possible_patterns(2, 5),
            vec![
                "##".to_string(),
                ".##".to_string(),
                "..##".to_string(),
                "...##".to_string()
            ]
        );
    }

    #[test]
    fn test_is_valid_pattern() {
        assert_eq!(is_valid_pattern("..##", "..##"), true);
        assert_eq!(is_valid_pattern(".??#", ".###"), true);
        assert_eq!(is_valid_pattern(".??#", "...#"), true);
        assert_eq!(is_valid_pattern(".??#", "#..#"), false);
        assert_eq!(is_valid_pattern(".??#", "...."), false);
    }
}
