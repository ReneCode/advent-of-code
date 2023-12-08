// day07

use std::{cmp::Ordering, collections::HashMap};

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
    fn new(cards: &str, bid: i32, with_joker: bool) -> Self {
        let typ = get_type(cards, with_joker);
        Game {
            cards: cards.to_string(),
            bid: bid,
            rank: 0,
            typ: typ,
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

fn get_type(cards: &str, with_joker: bool) -> Type {
    let counts = count_chars(cards, with_joker);
    let result = match counts.as_slice() {
        [5] => Type::FiveOfKind,
        [4, 1] => Type::FourOfKind,
        [3, 2] => Type::FullHouse,
        [3, 1, 1] => Type::ThreeOfKind,
        [2, 2, 1] => Type::TwoPair,
        [2, 1, 1, 1] => Type::OnePair,
        [1, 1, 1, 1, 1] => Type::HighCard,
        _ => {
            panic!("bad counts")
        }
    };
    result
}

fn count_chars(cards: &str, with_joker: bool) -> Vec<i32> {
    let mut char_counts: HashMap<char, i32> = HashMap::new();
    let mut count_joker = 0;
    for c in cards.chars() {
        if with_joker && c == 'J' {
            count_joker += 1;
            continue;
        }
        if let Some(val) = char_counts.get_mut(&c) {
            *val += 1;
        } else {
            char_counts.insert(c, 1);
        }
    }

    let mut counts = char_counts.iter().map(|(_k, v)| *v).collect_vec();
    counts.sort_by(|a, b| b.cmp(a)); // sort from high to low

    if with_joker {
        if counts.len() > 0 {
            counts[0] += count_joker;
        } else {
            counts.push(count_joker)
        }
    }

    counts
}

fn replace_cards(s: &str, with_joker: bool) -> String {
    let mut result = s
        .replace('T', "V")
        .replace('Q', "X")
        .replace('K', "Y")
        .replace('A', "Z");

    if with_joker {
        result = result.as_str().replace('J', "1");
    } else {
        result = result.as_str().replace('J', "W");
    }
    result
}

fn cmp_stronger(s1_org: &str, s2_org: &str, with_joker: bool) -> Ordering {
    let s1 = replace_cards(s1_org, with_joker);
    let s2 = replace_cards(s2_org, with_joker);

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

    let result_a = part_a(&lines);
    println!("Result A: {} ", result_a);

    let result_b = part_b(&lines);
    println!("Result B: {} ", result_b);
}

fn part_a(lines: &Vec<String>) -> i32 {
    let mut games: Vec<Game> = Vec::new();
    for line in lines {
        let tok = parse::to_str(line.as_str(), ' ');
        let bid: i32 = tok[1].parse().unwrap();
        let game = Game::new(tok[0], bid, false);
        games.push(game);
    }

    games.sort_by(|a, b| {
        let cmp = a.typ.cmp(&b.typ);
        if cmp != Ordering::Equal {
            return cmp;
        } else {
            return cmp_stronger(a.cards.as_str(), b.cards.as_str(), false);
        }
    });
    let mut rank = 0;
    let mut result_a = 0;
    for game in games.iter_mut() {
        rank += 1;
        game.rank = rank;
        result_a += game.rank * game.bid;
    }
    result_a
}

fn part_b(lines: &Vec<String>) -> i32 {
    let mut games: Vec<Game> = Vec::new();
    for line in lines {
        let tok = parse::to_str(line.as_str(), ' ');
        let bid: i32 = tok[1].parse().unwrap();
        let game = Game::new(tok[0], bid, true);
        games.push(game);
    }

    games.sort_by(|a, b| {
        let cmp = a.typ.cmp(&b.typ);
        if cmp != Ordering::Equal {
            return cmp;
        } else {
            return cmp_stronger(a.cards.as_str(), b.cards.as_str(), true);
        }
    });
    let mut rank = 0;
    let mut result_a = 0;
    for game in games.iter_mut() {
        rank += 1;
        game.rank = rank;
        result_a += game.rank * game.bid;
    }
    result_a
}

#[cfg(test)]
mod test {

    use std::cmp::Ordering;

    use crate::day07::{self, Type};
    #[test]
    fn test_cmp_rank() {
        assert_eq!(day07::cmp_stronger("T55J5", "QQQJA", false), Ordering::Less);
        assert_eq!(day07::cmp_stronger("T55J5", "KTJJT", false), Ordering::Less);
        assert_eq!(day07::cmp_stronger("QQQJA", "KTJJT", false), Ordering::Less);
    }

    #[test]
    fn test_count_chars() {
        assert_eq!(day07::count_chars("T55J5", false), vec![3, 1, 1]);
        assert_eq!(day07::count_chars("T55J5", true), vec![4, 1]);
        assert_eq!(day07::count_chars("JJJJJ", true), vec![5]);
        assert_eq!(day07::count_chars("2JJJJ", true), vec![5]);
        assert_eq!(day07::count_chars("21JJJ", true), vec![4, 1]);
        assert_eq!(day07::count_chars("234JJ", true), vec![3, 1, 1]);
        assert_eq!(day07::count_chars("2345J", true), vec![2, 1, 1, 1]);
    }

    #[test]
    fn test_get_type() {
        assert_eq!(day07::get_type("JJJJJ", true), Type::FiveOfKind);
        assert_eq!(day07::get_type("5JJJJ", true), Type::FiveOfKind);
        assert_eq!(day07::get_type("55JJJ", true), Type::FiveOfKind);
        assert_eq!(day07::get_type("56JJJ", true), Type::FourOfKind);
    }
}
