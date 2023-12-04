// day04

use core::num;
use std::collections::HashSet;

use itertools::Itertools;

use crate::util::{
    io,
    point2d::{self, Point2d},
};

#[derive(Debug)]
struct Card {
    id: i32,
    winning_numbers: Vec<i32>,
    my_numbers: Vec<i32>,
}

impl Card {
    fn new(id: i32, winning_numbers: Vec<i32>, my_numbers: Vec<i32>) -> Self {
        Card {
            id,
            winning_numbers,
            my_numbers,
        }
    }

    fn get_matching_numbers(&self) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        for wining_nr in self.winning_numbers.iter() {
            if self.my_numbers.contains(&wining_nr) {
                result.push(*wining_nr)
            }
        }
        result
    }
}
pub fn day04() {
    println!("hello day04");

    let lines = io::read_lines("./src/day04/04.data").unwrap();
    let cards = get_cards(&lines);
    // println!(">> {:?}", cards);

    let matching_cards = get_matching_numbers(&cards);
    // println!(">> {:?}", matching_cards);

    let result_a: i32 = matching_cards
        .iter()
        .map(|numbers| {
            let len = numbers.len() as u32;
            if len > 0 {
                return i32::pow(2i32, len - 1);
            } else {
                return 0;
            }
        })
        .sum();
    println!("Result A: {result_a}");
}

fn get_matching_numbers(cards: &Vec<Card>) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();
    for card in cards {
        let matching_numbers = card.get_matching_numbers();
        result.push(matching_numbers)
    }
    result
}

fn get_cards(lines: &[String]) -> Vec<Card> {
    let mut cards: Vec<Card> = Vec::new();
    for line in lines {
        let ta: Vec<&str> = split(&line, ':');
        let token: Vec<&str> = split(ta[0], ' ');
        let id: i32 = token[1].parse().unwrap();
        let tb: Vec<&str> = split(ta[1], '|');
        let winning_numbers = get_numbers(&tb[0]);
        let my_numbers = get_numbers(&tb[1]);
        cards.push(Card::new(id, winning_numbers, my_numbers))
    }
    cards
}

fn get_numbers(line: &str) -> Vec<i32> {
    let result: Vec<i32> = split(line, ' ')
        .iter()
        .map(|l| l.parse::<i32>().unwrap())
        .collect_vec();
    result
}

fn split(line: &str, delimiter: char) -> Vec<&str> {
    line.split(delimiter)
        .map(|l| l.trim())
        .filter(|l| l.len() > 0)
        .collect_vec()
}
