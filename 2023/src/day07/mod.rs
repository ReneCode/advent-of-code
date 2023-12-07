// day07

use core::num;
use std::{cmp::Ordering, collections::HashSet, string};

use itertools::Itertools;

use crate::util::{io, parse};

#[derive(Debug)]
struct Game {
    cards: String,
    bid: i32,
    rank: i32,
    typ: Type,
}

impl Game {
    fn new(cards: &str, bid: i32) -> Self {
        let typ = get_type(cards);
        Game {
            cards: replace_cards(cards),
            bid: bid,
            rank: 0,
            typ: typ,
        }
    }
}

#[derive(Debug)]

struct CharCount {
    char: char,
    count: i32,
}

impl CharCount {
    fn new(c: char, count: i32) -> Self {
        CharCount {
            char: c,
            count: count,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Type {
    FiveOfKind = 7,
    FourOfKind = 6,
    FullHouse = 5,
    ThreeOfKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

fn get_type(cards: &str) -> Type {
    let dups = get_duplicates(cards);
    let len = dups.len();
    if len == 0 {
        return Type::HighCard;
    }
    if len == 1 && dups[0].count == 5 {
        return Type::FiveOfKind;
    }
    if len == 1 && dups[0].count == 4 {
        return Type::FourOfKind;
    }
    if len == 2 && (dups[0].count == 3 || dups[1].count == 3) {
        return Type::FullHouse;
    }
    if len == 1 && dups[0].count == 3 {
        return Type::ThreeOfKind;
    }
    if len == 2 {
        return Type::TwoPair;
    }
    if len == 1 {
        return Type::OnePair;
    }

    panic!("bad cards")
}

fn get_duplicates(cards: &str) -> Vec<(CharCount)> {
    let mut checked_chars: HashSet<char> = HashSet::new();
    let mut dups: Vec<(CharCount)> = Vec::new();
    for i in 0..cards.len() {
        let mut count = 1;
        let c1 = cards.chars().nth(i).unwrap();
        if checked_chars.contains(&c1) {
            continue;
        } else {
            checked_chars.insert(c1);
        }
        for j in i + 1..cards.len() {
            let c2 = cards.chars().nth(j).unwrap();
            if c1 == c2 {
                count += 1;
            }
        }
        if count > 1 {
            dups.push(CharCount::new(c1, count));
        }
    }
    dups
}

fn replace_cards(s: &str) -> String {
    s.replace('T', "V")
        .replace('J', "W")
        .replace('Q', "X")
        .replace('K', "Y")
        .replace('A', "Z")
}

fn cmp_stronger(s1: &str, s2: &str) -> Ordering {
    for i in 0..s1.len() {
        let c1 = s1.chars().nth(i).unwrap();
        let c2 = s2.chars().nth(i).unwrap();
        let cmp = c1.cmp(&c2);
        if cmp != Ordering::Equal {
            return cmp;
        }
    }
    Ordering::Equal
}

pub fn day07() {
    println!("hello day07");

    let lines = io::read_lines("./src/day07/07.data").unwrap();

    let mut games: Vec<Game> = Vec::new();
    for line in lines {
        let tok = parse::to_str(line.as_str(), ' ');
        let bid: i32 = tok[1].parse().unwrap();
        let game = Game::new(tok[0], bid);
        games.push(game);
    }

    games.sort_by(|a, b| {
        let cmp = a.typ.cmp(&b.typ);
        if cmp != Ordering::Equal {
            return cmp;
        } else {
            return cmp_stronger(a.cards.as_str(), b.cards.as_str());
        }
    });
    let mut rank = 0;
    let mut result_a = 0;
    for game in games.iter_mut() {
        rank += 1;
        game.rank = rank;
        result_a += (game.rank * game.bid);
    }
    println!("Result A: {} ", result_a);
}
