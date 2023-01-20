// using modules
// https://levelup.gitconnected.com/easiest-way-to-understand-rust-modules-across-multiple-files-234b5018cbfd
mod util;

#[cfg(test)]
#[test]
fn test_calc_required() {
    assert_eq!(calc_required(12), 2);
    assert_eq!(calc_required(14), 2);
    assert_eq!(calc_required(1969), 654);
    assert_eq!(calc_required(100756), 33583);
}

#[test]
fn test_calc_required_part_2() {
    assert_eq!(calc_required_part_2(14), 2);
    assert_eq!(calc_required_part_2(1969), 966);
    assert_eq!(calc_required_part_2(100756), 50346);
}

fn main() {
    println!("Hello, day01");
    part_1();
    part_2();
}

fn part_1() {
    let mut total_fuel_value: i32 = 0;

    if let Some(lines) = util::get_lines("./01.data") {
        for line in lines {
            match line.parse::<i32>() {
                Ok(mass) => {
                    let fuel_value = calc_required(mass);
                    total_fuel_value = total_fuel_value + fuel_value;
                    // println!("{} / {}", mass, fuel_value);
                }
                Err(_) => {}
            }
        }
    }
    println!("part-1 total fuel value {}", total_fuel_value);
}

// fn parse_f32_or_0(str: &str) -> f32 {
//     match str.parse::<f32>() {
//         Ok(m) => m,
//         Err(_) => 0.0,
//     }
// }

fn parse_i32_or_0(str: &str) -> i32 {
    match str.parse::<i32>() {
        Ok(m) => m,
        Err(_) => 0,
    }
}

fn part_2() {
    if let Some(lines) = util::get_lines("./01.data") {
        let total = lines
            .iter()
            .map(|line| parse_i32_or_0(line))
            .map(|mass| calc_required_part_2(mass))
            .reduce(|a, b| a + b);
        if let Some(result) = total {
            println!("part-2 total {}", result);
        };
    }
}

fn calc_required(mass: i32) -> i32 {
    let fuel_value = (mass as f32 / 3.0).floor() - 2.0;
    fuel_value as i32
}

fn calc_required_part_2(mass: i32) -> i32 {
    let mut result = 0;
    let mut val = mass;
    loop {
        let req = calc_required(val);
        if req > 0 {
            result += req;
            val = req;
        } else {
            break;
        }
    }
    // println!("requ {} / {}", mass, result);
    result
}
