// day04

use itertools::Itertools;

use crate::util::io;

#[derive(Debug)]
struct Card {
    id: i32,
    winning_numbers: Vec<i32>,
    my_numbers: Vec<i32>,
    count_matching_numbers: i32,
}

impl Card {
    fn new(id: i32, winning_numbers: Vec<i32>, my_numbers: Vec<i32>) -> Self {
        let mut count_matching_numbers = 0;

        for wining_nr in winning_numbers.iter() {
            if my_numbers.contains(&wining_nr) {
                count_matching_numbers += 1
            }
        }
        Card {
            id,
            winning_numbers,
            my_numbers,
            count_matching_numbers,
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

    let mut result_b = 0;
    for card in cards.iter() {
        let count = count_resulting_cards(&cards, card);
        result_b += count;
        // println!(">> {}  {}", card.id, count);
    }
    println!("Result B: {result_b}");
}

fn count_resulting_cards(cards: &Vec<Card>, card: &Card) -> i32 {
    let mut count = 1;
    for id in card.id + 1..card.id + 1 + card.count_matching_numbers {
        let other_card = get_card(&cards, id);
        let count_other = count_resulting_cards(cards, other_card);
        count += count_other;
    }
    count
}

fn get_card(cards: &Vec<Card>, id: i32) -> &Card {
    cards.iter().find(|c| c.id == id).unwrap()
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
