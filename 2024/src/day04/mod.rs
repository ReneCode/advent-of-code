use crate::util::io;

pub fn day04() {
    let lines = io::read_lines("./src/day04/04.data").unwrap();

    let line_len = lines[0].len() as i64;
    let line_count = lines.len() as i64;

    let mut total_found = 0;
    let left_right_lines = lines.clone();
    total_found += find_all(left_right_lines);

    let mut top_down_lines: Vec<String> = Vec::new();
    for i in 0..line_len {
        let mut top_down_line = String::new();
        for j in 0..line_count {
            let c = lines[j as usize].chars().nth(i as usize).unwrap();
            top_down_line.push(c);
        }
        top_down_lines.push(top_down_line);
    }
    total_found += find_all(top_down_lines);

    // diagonal from top left to bottom right
    let mut right_down_lines: Vec<String> = Vec::new();
    let size = line_len.max(line_count);
    for x_start in 0 - (size - 1)..size as i64 {
        let mut diagonal_line = String::new();
        let mut y = 0;
        for x in x_start..x_start + size {
            if x >= 0 && x < line_len && y >= 0 && y < line_count {
                let c = lines[y as usize].chars().nth(x as usize).unwrap();
                diagonal_line.push(c);
            }
            y += 1;
        }
        if !diagonal_line.is_empty() {
            right_down_lines.push(diagonal_line);
        }
    }
    total_found += find_all(right_down_lines);

    // diagonal from top right to bottom left
    let mut right_up_lines: Vec<String> = Vec::new();
    for x_start in 0 - (size - 1)..size as i64 {
        let mut diagonal_line = String::new();
        let mut y = size - 1;
        for x in x_start..x_start + size {
            if x >= 0 && x < line_len && y >= 0 && y < line_count {
                let c = lines[y as usize].chars().nth(x as usize).unwrap();
                diagonal_line.push(c);
            }
            y -= 1;
        }
        if !diagonal_line.is_empty() {
            right_up_lines.push(diagonal_line);
        }
    }
    total_found += find_all(right_up_lines);

    println!("Day 04: {}", total_found);
}

fn find_all(lines: Vec<String>) -> usize {
    let re = regex::Regex::new(r"XMAS").unwrap();
    let re_rev = regex::Regex::new(r"SAMX").unwrap();

    let mut count = 0;
    for line in lines.iter() {
        count += re.find_iter(line).count();
        count += re_rev.find_iter(line).count();
    }
    count
}
