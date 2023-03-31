use util::io;

fn main() {
    println!("Hello, day16!");
    if let Some(input) = get_data("./16.data") {
        part_1(&input);
        part_2(&input);
    }
}

fn part_1(input: &String) {
    let result = &convert_multiple(input, input.len(), 100)[0..8];
    println!("part1 ater 100 phases: {result}");
}

fn part_2(input: &String) {
    let result = &solve_part2(input)[0..8];
    println!("part2 ater 10000 phases: {result}");
}

fn solve_part2(org_input: &str) -> String {
    let org_values = string_to_vector(org_input);

    let mut input: Vec<i32> = Vec::with_capacity(org_input.len() * 10000);
    for _ in 0..10000 {
        input.extend(&org_values);
    }

    let skip_len = vector_to_string(&input[0..7]).parse().unwrap();
    let input_len = input.len();

    println!(
        "skip {} from {} elements. That is much more than 1/4 so we can optimize.\nOnly use the '1' of the pattern 0,1,0,-1",
        skip_len, input_len
    );

    let mut right_part: Vec<i32> = input[skip_len..input_len].iter().map(|v| *v).collect();
    let mut out: Vec<i32> = Vec::new();
    for phase in 0..100 {
        out.clear();
        let mut sum_right: i32 = right_part.iter().sum();
        for val in right_part {
            out.push(sum_right % 10);
            // reduce the sum for the next loop
            sum_right -= val;
        }
        right_part = out.clone();
        // println!("phase: {}", phase + 1);
    }

    let result = vector_to_string(&right_part);
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
        .map(|c| c as i32 - '0' as i32)
        .collect()
}

fn get_pattern_val(idx: usize, out_idx: usize, pattern: &[i32]) -> i32 {
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

fn convert_digit(input: &Vec<i32>, out_idx: usize, repeat: usize) -> i32 {
    let mut result: i32 = 0;

    let (idx_plus, idx_minus) = get_relevant_indices(out_idx, input.len());
    let index_counts = compress_indices(&(idx_plus, idx_minus), repeat);

    for (idx, count) in index_counts.iter().enumerate() {
        if *count != 0 {
            result += *count * input[idx]
        }
    }

    result.abs() % 10
}

fn convert(input: &Vec<i32>, repeat: usize) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::with_capacity(input.len());
    for i in 0..input.len() {
        result.push(convert_digit(input, i, repeat));
        // println!("convert {i}");
    }
    result
}

fn convert_multiple(input: &str, repeat: usize, phases: usize) -> String {
    let mut values = string_to_vector(input);
    for phase in 0..phases {
        // println!("run phase: {phase} ");
        values = convert(&values, repeat);
    }

    vector_to_string(&values)
}

fn vector_to_string(values: &[i32]) -> String {
    let u8_values: Vec<u8> = values.iter().map(|v| *v as u8).collect();

    let mut result = String::new();
    for (i, v) in u8_values.iter().enumerate() {
        let x = b'0' + *v;
        result.push(x as char);
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
            end = end.min(len as i32 - 1) + 1;
            result_plus.extend(start as usize..end as usize);
        }
        if val == 3 {
            end = start + step - 1;
            end = end.min(len as i32 - 1) + 1;
            result_minus.extend(start as usize..end as usize);
        }

        if start >= len as i32 {
            break;
        }
        start += step;
        val = (val + 1) % 4;
    }

    (result_plus, result_minus)
}

fn compress_indices((plus, minus): &(Vec<usize>, Vec<usize>), repeat: usize) -> Vec<i32> {
    let mut result: Vec<i32> = vec![0; repeat];

    for v in plus {
        let val = *v % repeat;
        result[val] += 1;
    }
    for v in minus {
        let val = *v % repeat;
        result[val] -= 1;
    }
    result
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

        assert_eq!(
            get_relevant_indices(0, 40),
            (
                vec![0, 4, 8, 12, 16, 20, 24, 28, 32, 36],
                vec![2, 6, 10, 14, 18, 22, 26, 30, 34, 38]
            )
        );
        assert_eq!(
            get_relevant_indices(1, 40),
            (
                vec![1, 2, 9, 10, 17, 18, 25, 26, 33, 34],
                vec![5, 6, 13, 14, 21, 22, 29, 30, 37, 38]
            )
        );
        assert_eq!(
            get_relevant_indices(2, 40),
            (
                vec![2, 3, 4, 14, 15, 16, 26, 27, 28, 38, 39],
                vec![8, 9, 10, 20, 21, 22, 32, 33, 34]
            )
        );
        assert_eq!(
            get_relevant_indices(3, 40),
            (
                vec![3, 4, 5, 6, 19, 20, 21, 22, 35, 36, 37, 38],
                vec![11, 12, 13, 14, 27, 28, 29, 30]
            )
        );
        assert_eq!(
            get_relevant_indices(4, 40),
            (
                vec![4, 5, 6, 7, 8, 24, 25, 26, 27, 28],
                vec![14, 15, 16, 17, 18, 34, 35, 36, 37, 38]
            )
        );
        assert_eq!(
            get_relevant_indices(5, 40),
            (
                vec![5, 6, 7, 8, 9, 10, 29, 30, 31, 32, 33, 34],
                vec![17, 18, 19, 20, 21, 22]
            )
        );
        assert_eq!(
            get_relevant_indices(6, 40),
            (
                vec![6, 7, 8, 9, 10, 11, 12, 34, 35, 36, 37, 38, 39],
                vec![20, 21, 22, 23, 24, 25, 26,]
            )
        );

        assert_eq!(
            get_relevant_indices(7, 40),
            (
                vec![7, 8, 9, 10, 11, 12, 13, 14, 39],
                vec![23, 24, 25, 26, 27, 28, 29, 30]
            )
        );
    }

    #[test]
    fn test_compress_indices() {
        let plus = vec![4, 5, 6, 7, 8, 24, 25, 26, 27, 28];
        let minus = vec![14, 15, 16, 17, 18];
        assert_eq!(
            compress_indices(&(plus, minus), 8),
            vec![1, 0, 0, 1, 2, 1, 0, 0]
        )
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
        let input = vec![1, 2, 3, 4, 5, 6, 7, 8];
        assert_eq!(convert_digit(&input, 0, 8), 4);
        assert_eq!(convert_digit(&input, 1, 8), 8);
        assert_eq!(convert_digit(&input, 2, 8), 2);
        assert_eq!(convert_digit(&input, 3, 8), 2);
        assert_eq!(convert_digit(&input, 4, 8), 6);
        assert_eq!(convert_digit(&input, 5, 8), 1);
        assert_eq!(convert_digit(&input, 6, 8), 5);
        assert_eq!(convert_digit(&input, 7, 8), 8);
    }

    #[test]
    fn test_convert_digit_2() {
        let input = vec![4, 8, 2, 2, 6, 1, 5, 8];
        assert_eq!(convert_digit(&input, 0, 8), 3);
        assert_eq!(convert_digit(&input, 1, 8), 4);
        assert_eq!(convert_digit(&input, 2, 8), 0);
        assert_eq!(convert_digit(&input, 3, 8), 4);
        assert_eq!(convert_digit(&input, 4, 8), 0);
        assert_eq!(convert_digit(&input, 5, 8), 4);
        assert_eq!(convert_digit(&input, 6, 8), 3);
        assert_eq!(convert_digit(&input, 7, 8), 8);
    }

    #[test]
    fn test_convert() {
        assert_eq!(
            convert(&vec![1, 2, 3, 4, 5, 6, 7, 8], 8),
            [4, 8, 2, 2, 6, 1, 5, 8]
        );
        assert_eq!(
            convert(&vec![4, 8, 2, 2, 6, 1, 5, 8], 8),
            [3, 4, 0, 4, 0, 4, 3, 8]
        );
    }

    #[test]
    fn test_convert_multiple() {
        assert_eq!(convert_multiple("12345678", 8, 1), "48226158");
        assert_eq!(convert_multiple("12345678", 8, 2), "34040438");
        assert_eq!(convert_multiple("12345678", 8, 3), "03415518");
        assert_eq!(convert_multiple("12345678", 8, 4), "01029498");
    }

    #[test]
    fn test_convert_multiple_large() {
        assert_eq!(
            &convert_multiple("80871224585914546619083218645595", 32, 100)[0..8],
            "24176176"
        );
        assert_eq!(
            &convert_multiple("19617804207202209144916044189917", 32, 100)[0..8],
            "73745418"
        );
        assert_eq!(
            &convert_multiple("69317163492948606335995924319873", 32, 100)[0..8],
            "52432133"
        );
    }

    #[test]
    fn test_solve_part2_1() {
        let input = "03036732577212944063491565474664";
        let result = &solve_part2(input)[0..8];
        assert_eq!(result, "84462026");
    }

    #[test]
    fn test_solve_part2_2() {
        let input = "02935109699940807407585447034323";
        let result = &solve_part2(input)[0..8];
        assert_eq!(result, "78725270");
    }
    #[test]
    fn test_solve_part2_3() {
        let input = "03081770884921959731165446850517";
        let result = &solve_part2(input)[0..8];
        assert_eq!(result, "53553731");
    }
}
