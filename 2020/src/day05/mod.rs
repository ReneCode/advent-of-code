// day05

use itertools::Itertools;

use crate::util::io;

pub fn day05() {
    let lines = io::read_lines("05.data").unwrap();

    let mut seat_ids: Vec<i32> = lines.iter().map(|line| get_seat_id(line)).collect();
    let max_seat_id = seat_ids.iter().max().unwrap();
    println!("A: max seat ID: {}", max_seat_id);

    let mut invalid_seat_ids: Vec<i32> = Vec::new();
    for row in [0, 127] {
        for column in 0..8 {
            let seat_id = row * 8 + column;
            invalid_seat_ids.push(seat_id);
        }
    }

    let remaining_seat_ids: Vec<i32> = seat_ids
        .iter()
        .filter(|id| !invalid_seat_ids.contains(id))
        .map(|id| *id)
        .sorted()
        .collect();

    println!("remaining seat IDs: {:?}", remaining_seat_ids.len());

    for i in 1..remaining_seat_ids.len() {
        let id1 = remaining_seat_ids[i - 1];
        let id2 = remaining_seat_ids[i];
        if id1.abs_diff(id2) == 2 {
            println!("B: between seat IDs: {} and {} => {}", id1, id2, id1 + 1);
        }
    }
}

fn get_valid_seat_id(line: &str) -> Option<i32> {
    let row_line = &line[..7];
    let row = get_row(row_line);
    if row == 0 || row == 127 {
        return None;
    }
    let column_line = &line[7..];
    let column = get_column(column_line);
    Some(row * 8 + column)
}

fn get_seat_id(line: &str) -> i32 {
    let row_line = &line[..7];
    let row = get_row(row_line);
    let column_line = &line[7..];
    let column = get_column(column_line);
    row * 8 + column
}

fn get_row(row: &str) -> i32 {
    let mut min = 0;
    let mut max = 127;
    for c in row.chars() {
        let half = (max - min) / 2;
        match c {
            'F' => max -= half + 1,
            'B' => min += half + 1,
            _ => panic!("unexpected char in row: {}", c),
        }
    }
    min
}

fn get_column(column: &str) -> i32 {
    let mut min = 0;
    let mut max = 7;
    for c in column.chars() {
        let half = (max - min) / 2;
        match c {
            'L' => max -= half + 1,
            'R' => min += half + 1,
            _ => panic!("unexpected char in column: {}", c),
        }
    }
    min
}
