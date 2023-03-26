use std::collections::HashMap;

use util::io;

struct Pattern<'a> {
    org_pattern: &'a Vec<i32>,
    max_idx: usize,
    cache: HashMap<usize, Vec<i32>>,
}

impl<'a> Pattern<'a> {
    fn new(org_pattern: &'a Vec<i32>, max_idx: usize) -> Self {
        Pattern {
            org_pattern,
            max_idx,
            cache: HashMap::new(),
        }
    }

    fn get_value(&mut self, idx: usize, out_idx: usize) -> i32 {
        if let Some(pattern) = self.cache.get(&out_idx) {
            return pattern[idx + 1];
        } else {
            // println!("create pattern {out_idx}");
            let mut new_pattern = self.org_pattern.iter().fold(Vec::new(), |mut acc, val| {
                for _i in 0..=out_idx {
                    acc.push(*val);
                }
                acc
            });
            while (self.max_idx + 1) >= new_pattern.len() {
                new_pattern.extend(new_pattern.clone());
            }
            let val = new_pattern[idx + 1];
            self.cache.insert(out_idx, new_pattern);
            return val;
        }
    }
}

fn main() {
    println!("Hello, day16!");
    if let Some(input) = get_data("./16.data") {
        part_1(&input);
        // part_2(&input);
    }
}

fn part_1(input: &String) {
    let pat = vec![0, 1, 0, -1];
    let mut pattern = Pattern::new(&pat, input.len());

    let result = &convert_multiple(input, &mut pattern, 100)[0..8];
    println!("part1 ater 100 phases: {result}");
}

fn part_2(input: &String) {
    let pat = vec![0, 1, 0, -1];
    let mut pattern = Pattern::new(&pat, input.len());

    let result = convert_multiple(input, &mut pattern, 100);
    println!("part2 ater 10000 phases: {result}");
}

fn solve_part2(org_input: &str) -> String {
    let mut input = String::with_capacity(org_input.len() * 10000);
    for _ in 0..10000 {
        input.extend(org_input.chars());
    }
    let pat = vec![0, 1, 0, -1];
    let mut pattern = Pattern::new(&pat, input.len());

    let result = convert_multiple(input.as_str(), &mut pattern, 100);
    result
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

fn convert_digit_a(input: &Vec<i32>, out_idx: usize, pattern: &mut Pattern) -> i32 {
    let mut result: i32 = 0;

    let (idx_plus, idx_minus) = get_relevant_indices(out_idx, input.len());
    for idx in idx_plus {
        let value = pattern.get_value(idx, out_idx);
        result += input[idx];
    }

    for idx in idx_minus {
        let value = pattern.get_value(idx, out_idx);
        result -= input[idx];
    }

    result.abs() % 10
}

fn convert_digit(input: &Vec<i32>, out_idx: usize, pattern: &mut Pattern) -> i32 {
    let mut result: i32 = 0;

    let (idx_plus, idx_minus) = get_relevant_indices(out_idx, input.len());
    for idx in idx_plus {
        result += input[idx];
    }

    for idx in idx_minus {
        result -= input[idx];
    }

    result.abs() % 10
}

fn convert(input: &Vec<i32>, pattern: &mut Pattern) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::with_capacity(input.len());
    for i in 0..input.len() {
        result.push(convert_digit(input, i, pattern))
    }
    result
}

fn convert_multiple(input: &str, pattern: &mut Pattern, phases: usize) -> String {
    let mut values = string_to_vector(input);
    for phase in 0..phases {
        println!("run phase: {phase} ");
        values = convert(&values, pattern);
    }

    let result = list_to_string(&values);
    result
}

fn list_to_string(values: &Vec<i32>) -> String {
    let u8_values: Vec<u8> = values.iter().map(|v| *v as u8).collect();

    let mut result = String::new();
    for (i, v) in u8_values.iter().enumerate() {
        let x = '0' as u8 + *v;
        result.push(x as char);
        if i == 7 {
            // take only 8 chars
            // break;
        }
    }
    result
}

fn get_relevant_indices(out_idx: usize, len: usize) -> (Vec<usize>, Vec<usize>) {
    let mut result_plus: Vec<usize> = Vec::new();
    let mut result_minus: Vec<usize> = Vec::new();
    let step: i32 = out_idx as i32 + 1;

    // pattern 0  1  0 -1     relevant is 1 and -1
    // pattern 0  1  2  3
    let mut val = 0;

    let mut start = -1;
    let mut end = start + step;
    loop {
        if val == 1 {
            end = start + step - 1;
            result_plus.extend(start as usize..(end.min(len as i32 - 1) + 1) as usize);
        }
        if val == 3 {
            end = start + step - 1;
            result_minus.extend(start as usize..(end.min(len as i32 - 1) + 1) as usize);
        }

        if start >= len as i32 {
            break;
        }
        start += step;
        val = (val + 1) % 4;
    }

    (result_plus, result_minus)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_relevant_indices() {
        /*
        01020102  pat
         0123456789 val
         0 2 4 6 8 ->

        0011002200110022001100
         012345678901234567890
          12  56  90  34  78

        0001110002220001110002
         012345678901234567890
           234   890   456   0

          */
        assert_eq!(get_relevant_indices(0, 10), (vec![0, 4, 8], vec![2, 6]));
        assert_eq!(get_relevant_indices(1, 10), (vec![1, 2, 9], vec![5, 6]));
        assert_eq!(get_relevant_indices(2, 10), (vec![2, 3, 4], vec![8, 9]));
        assert_eq!(
            get_relevant_indices(3, 20),
            (vec![3, 4, 5, 6, 19], vec![11, 12, 13, 14])
        );
    }

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
        let pat = vec![0, 1, 0, -1];
        let input = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let mut pattern = Pattern::new(&pat, input.len());
        assert_eq!(convert_digit(&input, 0, &mut pattern), 4);
        assert_eq!(convert_digit(&input, 1, &mut pattern), 8);
        assert_eq!(convert_digit(&input, 2, &mut pattern), 2);
        assert_eq!(convert_digit(&input, 3, &mut pattern), 2);
        assert_eq!(convert_digit(&input, 4, &mut pattern), 6);
        assert_eq!(convert_digit(&input, 5, &mut pattern), 1);
        assert_eq!(convert_digit(&input, 6, &mut pattern), 5);
        assert_eq!(convert_digit(&input, 7, &mut pattern), 8);
    }

    #[test]
    fn test_convert_digit_2() {
        let pat = vec![0, 1, 0, -1];
        let input = vec![4, 8, 2, 2, 6, 1, 5, 8];
        let mut pattern = Pattern::new(&pat, input.len());
        assert_eq!(convert_digit(&input, 0, &mut pattern), 3);
        assert_eq!(convert_digit(&input, 1, &mut pattern), 4);
        assert_eq!(convert_digit(&input, 2, &mut pattern), 0);
        assert_eq!(convert_digit(&input, 3, &mut pattern), 4);
        assert_eq!(convert_digit(&input, 4, &mut pattern), 0);
        assert_eq!(convert_digit(&input, 5, &mut pattern), 4);
        assert_eq!(convert_digit(&input, 6, &mut pattern), 3);
        assert_eq!(convert_digit(&input, 7, &mut pattern), 8);
    }

    #[test]
    fn test_convert() {
        let pat = vec![0, 1, 0, -1];
        let mut pattern = Pattern::new(&pat, 8);

        assert_eq!(
            convert(&vec![1, 2, 3, 4, 5, 6, 7, 8], &mut pattern),
            [4, 8, 2, 2, 6, 1, 5, 8]
        );
        assert_eq!(
            convert(&vec![4, 8, 2, 2, 6, 1, 5, 8], &mut pattern),
            [3, 4, 0, 4, 0, 4, 3, 8]
        );
    }

    #[test]
    fn test_convert_multiple() {
        let pat = vec![0, 1, 0, -1];
        let mut pattern = Pattern::new(&pat, 8);

        assert_eq!(convert_multiple("12345678", &mut pattern, 1), "48226158");
        assert_eq!(convert_multiple("12345678", &mut pattern, 2), "34040438");
        assert_eq!(convert_multiple("12345678", &mut pattern, 3), "03415518");
        assert_eq!(convert_multiple("12345678", &mut pattern, 4), "01029498");
    }

    #[test]
    fn test_convert_multiple_large() {
        let pat = vec![0, 1, 0, -1];
        let mut pattern = Pattern::new(&pat, 32);

        assert_eq!(
            &convert_multiple("80871224585914546619083218645595", &mut pattern, 100)[0..8],
            "24176176"
        );
        assert_eq!(
            &convert_multiple("19617804207202209144916044189917", &mut pattern, 100)[0..8],
            "73745418"
        );
        assert_eq!(
            &convert_multiple("69317163492948606335995924319873", &mut pattern, 100)[0..8],
            "52432133"
        );
    }

    #[test]
    #[ignore = "not ready yet"]
    fn test_solve_part2() {
        let pat = vec![0, 1, 0, -1];
        let mut pattern = Pattern::new(&pat, 32);

        let input = "03036732577212944063491565474664";
        let result = solve_part2(input);
        assert_eq!(result, "");
    }
}
