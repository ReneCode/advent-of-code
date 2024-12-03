use crate::util::io;

pub fn day02() {
    let lines = io::read_lines("./src/day02/02.data").unwrap();

    let levels: Vec<Vec<i32>> = lines
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    let count_safe = levels.iter().filter(|level| is_safe(level)).count();
    println!("Safe levels: {}", count_safe);

    let count_damped_safe = levels.iter().filter(|level| is_damped_safe(level)).count();
    println!("Damped safe levels: {}", count_damped_safe);
}

fn is_safe(level: &[i32]) -> bool {
    let down = level[0] > level[1];
    for i in 0..level.len() - 1 {
        if (down && level[i] <= level[i + 1]) || (!down && level[i] >= level[i + 1]) {
            // turns direction
            return false;
        }
    }

    for i in 0..level.len() - 1 {
        let diff = (level[i] - level[i + 1]).abs();
        match diff {
            1..=3 => (),
            _ => return false,
        }
    }

    true
}

fn is_damped_safe(level: &[i32]) -> bool {
    if is_safe(level) {
        return true;
    }

    for i in 0..level.len() {
        let one_removed: Vec<i32> = level
            .iter()
            .enumerate()
            .filter(|&(idx, _)| idx != i)
            .map(|(_idx, nr)| *nr)
            .collect();
        if is_safe(&one_removed) {
            return true;
        }
    }

    false
}
