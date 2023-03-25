use util::io;

fn main() {
    println!("Hello, day16!");
    if let Some(input) = get_data("./16.data") {
        part_1(&input);
    }
}

fn part_1(input: &String) {
    let pattern = vec![0, 1, 0, -1];

    let result = convert_multiple(input, &pattern, 100);
    println!("part1 ater 100 phases: {result}");
}

fn get_data(filename: &str) -> Option<String> {
    if let Some(lines) = io::get_lines(filename) {
        Some(lines[0].clone())
    } else {
        None
    }
}

fn string_to_vector(input: &str) -> Vec<i32> {
    input
        .chars()
        .into_iter()
        .map(|c| {
            let val = c as i32 - '0' as i32;
            val
        })
        .collect()
}

fn get_pattern_val(idx: usize, out_idx: usize, pattern: &Vec<i32>) -> i32 {
    let mut new_pattern = pattern.iter().fold(Vec::new(), |mut acc, val| {
        for _i in 0..=out_idx {
            acc.push(*val);
        }
        acc
    });
    while (idx + 1) >= new_pattern.len() {
        new_pattern.extend(new_pattern.clone());
    }

    new_pattern[idx + 1]
}

fn convert_digit(input: &Vec<i32>, out_idx: usize, pattern: &Vec<i32>) -> i32 {
    let mut new_pattern = pattern.iter().fold(Vec::new(), |mut acc, val| {
        for _i in 0..=out_idx {
            acc.push(*val);
        }
        acc
    });
    while input.len() >= new_pattern.len() {
        new_pattern.extend(new_pattern.clone());
    }

    let mut result = 0;
    for (i, v) in input.iter().enumerate() {
        let pattern_val = new_pattern[i + 1];
        result += *v * pattern_val;
    }

    result.abs() % 10
}

fn convert(input: &Vec<i32>, pattern: &Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    for i in 0..input.len() {
        result.push(convert_digit(input, i, pattern))
    }
    result
}

fn convert_multiple(input: &str, pattern: &Vec<i32>, phases: usize) -> String {
    let mut values = string_to_vector(input);
    for _ in 0..phases {
        values = convert(&values, pattern);
    }

    let u8_values: Vec<u8> = values.iter().map(|v| *v as u8).collect();

    let mut result = String::new();
    for (i, v) in u8_values.iter().enumerate() {
        let x = '0' as u8 + *v;
        result.push(x as char);
        if i == 7 {
            // take only 8 chars
            break;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_to_vector() {
        let input = string_to_vector("12345678");
        assert_eq!(input, vec![1, 2, 3, 4, 5, 6, 7, 8]);
    }
    #[test]
    fn test_get_pattern_val() {
        let pattern = vec![0, 1, 0, -1];
        // output 0
        assert_eq!(get_pattern_val(0, 0, &pattern), 1);
        assert_eq!(get_pattern_val(1, 0, &pattern), 0);
        assert_eq!(get_pattern_val(2, 0, &pattern), -1);
        assert_eq!(get_pattern_val(3, 0, &pattern), 0);
        assert_eq!(get_pattern_val(4, 0, &pattern), 1);
        assert_eq!(get_pattern_val(5, 0, &pattern), 0);
        assert_eq!(get_pattern_val(6, 0, &pattern), -1);
        assert_eq!(get_pattern_val(7, 0, &pattern), 0);

        // output 1
        assert_eq!(get_pattern_val(0, 1, &pattern), 0);
        assert_eq!(get_pattern_val(1, 1, &pattern), 1);
        assert_eq!(get_pattern_val(2, 1, &pattern), 1);
        assert_eq!(get_pattern_val(3, 1, &pattern), 0);
        assert_eq!(get_pattern_val(4, 1, &pattern), 0);
        assert_eq!(get_pattern_val(5, 1, &pattern), -1);
        assert_eq!(get_pattern_val(6, 1, &pattern), -1);
        assert_eq!(get_pattern_val(7, 1, &pattern), 0);
    }

    #[test]
    fn test_convert_digit_1() {
        let pattern = vec![0, 1, 0, -1];
        let input = vec![1, 2, 3, 4, 5, 6, 7, 8];
        assert_eq!(convert_digit(&input, 0, &pattern), 4);
        assert_eq!(convert_digit(&input, 1, &pattern), 8);
        assert_eq!(convert_digit(&input, 2, &pattern), 2);
        assert_eq!(convert_digit(&input, 3, &pattern), 2);
        assert_eq!(convert_digit(&input, 4, &pattern), 6);
        assert_eq!(convert_digit(&input, 5, &pattern), 1);
        assert_eq!(convert_digit(&input, 6, &pattern), 5);
        assert_eq!(convert_digit(&input, 7, &pattern), 8);
    }

    #[test]
    fn test_convert_digit_2() {
        let pattern = vec![0, 1, 0, -1];
        let input = vec![4, 8, 2, 2, 6, 1, 5, 8];
        assert_eq!(convert_digit(&input, 0, &pattern), 3);
        assert_eq!(convert_digit(&input, 1, &pattern), 4);
        assert_eq!(convert_digit(&input, 2, &pattern), 0);
        assert_eq!(convert_digit(&input, 3, &pattern), 4);
        assert_eq!(convert_digit(&input, 4, &pattern), 0);
        assert_eq!(convert_digit(&input, 5, &pattern), 4);
        assert_eq!(convert_digit(&input, 6, &pattern), 3);
        assert_eq!(convert_digit(&input, 7, &pattern), 8);
    }

    #[test]
    fn test_convert() {
        let pattern = vec![0, 1, 0, -1];
        assert_eq!(
            convert(&vec![1, 2, 3, 4, 5, 6, 7, 8], &pattern),
            [4, 8, 2, 2, 6, 1, 5, 8]
        );
        assert_eq!(
            convert(&vec![4, 8, 2, 2, 6, 1, 5, 8], &pattern),
            [3, 4, 0, 4, 0, 4, 3, 8]
        );
    }

    #[test]
    fn test_convert_multiple() {
        let pattern = vec![0, 1, 0, -1];
        assert_eq!(convert_multiple("12345678", &pattern, 1), "48226158");
        assert_eq!(convert_multiple("12345678", &pattern, 2), "34040438");
        assert_eq!(convert_multiple("12345678", &pattern, 3), "03415518");
        assert_eq!(convert_multiple("12345678", &pattern, 4), "01029498");
    }

    #[test]
    fn test_convert_multiple_large() {
        let pattern = vec![0, 1, 0, -1];
        assert_eq!(
            convert_multiple("80871224585914546619083218645595", &pattern, 100),
            "24176176"
        );
        assert_eq!(
            convert_multiple("19617804207202209144916044189917", &pattern, 100),
            "73745418"
        );
        assert_eq!(
            convert_multiple("69317163492948606335995924319873", &pattern, 100),
            "52432133"
        );
    }
}
