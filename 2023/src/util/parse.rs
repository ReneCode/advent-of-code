use std::{
    fmt::{self, Debug},
    str::FromStr,
};

use itertools::Itertools;

pub fn to_numbers<T: FromStr>(line: &str, delimiter: char) -> Vec<T>
where
    <T as FromStr>::Err: Debug,
{
    let result: Vec<T> = line
        .split(delimiter)
        .map(|l| l.trim())
        .filter(|l| l.len() > 0)
        .map(|l| l.parse().unwrap())
        .collect_vec();
    result
}

pub fn to_str(line: &str, delimiter: char) -> Vec<&str> {
    let result: Vec<&str> = line
        .split(delimiter)
        .map(|l| l.trim())
        .filter(|l| l.len() > 0)
        .collect_vec();
    result
}
