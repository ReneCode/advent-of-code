// use util::io;

fn main() {
    println!("Hello, day16!");

    println!("{}", 4 % 4);
}

// fn convert(input: &Vec<i32>, pattern: &Vec<i32>) -> Vec<i32> {
//     let result: Vec<i32> = Vec::new();

//     result
// }

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
}
