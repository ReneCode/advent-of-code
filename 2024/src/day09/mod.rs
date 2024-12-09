use crate::util::io;

type Number = i64;

const FREE: Number = -1;

pub fn day09() {
    let lines = io::read_lines("./src/day09/09.data").unwrap();

    let line = lines.first().unwrap();

    let mut disk: Vec<Number> = Vec::new();

    let mut id: Number = 0;
    for (pos, c) in line.chars().enumerate() {
        let count = c.to_digit(10).unwrap() as Number;
        let block;
        if pos % 2 == 0 {
            block = id;
            id += 1;
        } else {
            block = FREE;
        }
        for _i in 0..count {
            disk.push(block);
        }
    }

    part1(&disk);
}

fn part1(disk: &[Number]) {
    let mut result_ids: Vec<Number> = Vec::new();
    let mut last_pos = disk.len() - 1;
    for (pos, block) in disk.iter().enumerate() {
        if pos > last_pos {
            break;
        }

        if *block != FREE {
            result_ids.push(*block);
        } else {
            // from the end, find the last file sector
            let mut last_block = disk.get(last_pos).unwrap();
            while pos <= last_pos && *last_block == FREE {
                last_pos -= 1;
                last_block = disk.get(last_pos).unwrap();
            }

            if pos > last_pos {
                break;
            }
            last_pos -= 1;
            result_ids.push(*last_block);
        }
    }

    let result = calc_checksum(&result_ids);

    println!("Day09 part 1: {:?}", result);
}

fn calc_checksum(ids: &[Number]) -> Number {
    let mut result = 0;
    for (idx, id) in ids.iter().enumerate() {
        result += idx as i64 * *id;
    }
    result
}
