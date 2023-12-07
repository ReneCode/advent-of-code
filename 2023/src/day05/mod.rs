// day05

use itertools::Itertools;

use crate::util::{io, parse};

type Id = u64;

#[derive(PartialEq)]
enum State {
    ReadDestination,
    ReadMappingHeader,
    ReadMapping,
}

#[derive(Debug)]
struct Mapping {
    source_start_nr: Id,
    destination_start_nr: Id,
    length: Id,
}

#[derive(Debug)]
struct Mapper {
    source: String,
    destination: String,
    mappings: Vec<Mapping>,
}

impl Mapper {
    fn new(source: &str, destination: &str) -> Self {
        Mapper {
            source: source.to_string(),
            destination: destination.to_string(),
            mappings: Vec::new(),
        }
    }

    fn addd_mapping(&mut self, source_start_nr: Id, destination_start_nr: Id, length: Id) {
        let mapping = Mapping {
            source_start_nr,
            destination_start_nr,
            length,
        };
        self.mappings.push(mapping);
    }

    fn calc_nr(&self, source_nr: Id) -> Id {
        for mapping in self.mappings.iter() {
            if mapping.source_start_nr <= source_nr
                && source_nr <= mapping.source_start_nr + mapping.length
            {
                return source_nr - mapping.source_start_nr + mapping.destination_start_nr;
            }
        }
        return source_nr;
    }
}

pub fn day05() {
    println!("hello day05");

    let lines = io::read_lines("./src/day05/05.data").unwrap();

    let groups = lines.join("\n");
    let groups = groups.split("\n\n").collect_vec();

    let mut destination_numbers: Vec<u64> = Vec::new();
    let mut all_mapper: Vec<Mapper> = Vec::new();
    let mut state: State = State::ReadDestination;
    for group in groups {
        let lines = group.split('\n').collect_vec();
        // src-to-dest map:
        // dest-start src-start length
        let mut one_mapper: Mapper = Mapper::new("", "");

        for line in lines {
            // println!("{line}");
            if line.len() == 0 {
                state = State::ReadMappingHeader;
                continue;
            }
            let source: String;
            let destination: String;
            match state {
                State::ReadDestination => {
                    let tok = parse::to_str(line, ':');
                    destination_numbers = parse::to_numbers::<Id>(tok[1], ' ');
                    state = State::ReadMappingHeader
                }
                State::ReadMappingHeader => {
                    let tok = parse::to_str(line, ' ');
                    let tok = parse::to_str(tok[0], '-');
                    source = tok[0].to_string();
                    destination = tok[2].to_string();
                    one_mapper = Mapper::new(&source, &destination);
                    state = State::ReadMapping
                }
                State::ReadMapping => {
                    let tok = parse::to_numbers::<Id>(line, ' ');
                    one_mapper.addd_mapping(tok[1], tok[0], tok[2])
                }
            }
        }
        if state == State::ReadMapping {
            all_mapper.push(one_mapper);
            state = State::ReadMappingHeader
        }
    }

    // println!(">>> all_mapper {:?}", all_mapper)

    let result_numbers = destination_numbers
        .iter()
        .map(|nr| get_nr("seed", *nr, "location", &all_mapper))
        .collect_vec();
    let min_number = result_numbers.iter().min().unwrap();
    println!("Result A: {}", *min_number);

    let mut all_dest_nr: Vec<Id> = Vec::new();
    let mut idx = 0;
    while idx < destination_numbers.len() {
        for nr in destination_numbers[idx]..destination_numbers[idx] + destination_numbers[idx + 1]
        {
            all_dest_nr.push(nr);
        }
        idx += 2;
    }
    println!("start mapping");
    let result_numbers = all_dest_nr
        .iter()
        .map(|nr| {
            let result_nr = get_nr("seed", *nr, "location", &all_mapper);
            // println!("{nr} -> {result_nr}");
            result_nr
        })
        .collect_vec();
    let min_number = result_numbers.iter().min().unwrap();
    println!("Result B: {}", *min_number);
}

fn get_nr(source: &str, source_nr: Id, destination: &str, mappers: &Vec<Mapper>) -> Id {
    if let Some(mapper) = mappers.iter().find(|m| m.source == source) {
        let dest_nr: Id = mapper.calc_nr(source_nr);
        if mapper.destination == destination {
            return dest_nr;
        } else {
            return get_nr(&mapper.destination, dest_nr, destination, mappers);
        }
    }
    0
}
