// day15

use itertools::Itertools;

use crate::util::{io, parse};

struct Lens {
    label: String,
    length: u32,
}

impl Lens {
    fn new(label: String, length: u32) -> Self {
        Lens {
            label: label,
            length: length,
        }
    }
}

struct LensBox {
    nr: u32,
    lenses: Vec<Lens>,
}

impl LensBox {
    fn new(nr: u32) -> Self {
        LensBox {
            nr: nr,
            lenses: Vec::new(),
        }
    }

    fn remove_lense(&mut self, label: &str) {
        if let Some(idx) = self.lenses.iter().position(|l| l.label == label) {
            self.lenses.remove(idx);
        }
    }

    fn add_or_replace_lense(&mut self, label: &str, lens_len: u32) {
        if let Some(lense) = self.lenses.iter_mut().find(|l| l.label == label) {
            // update length
            lense.length = lens_len
        } else {
            self.lenses.push(Lens::new(label.to_string(), lens_len));
        }
    }

    fn calc_focusing_power(&self) -> u32 {
        let mut result: u32 = 0;
        for (i, lens) in self.lenses.iter().enumerate() {
            let lens_focusing_power = (self.nr + 1) * (i as u32 + 1) * lens.length;
            result += lens_focusing_power;
        }
        result
    }
}

pub fn day15() {
    println!("hello day15");

    let lines = io::read_lines("./src/day15/15.data").unwrap();
    let line = lines.get(0).unwrap();
    let commands = parse::to_str(line, ',');
    let result_a: u32 = commands.iter().map(|s| calc_hash(s)).sum();
    // let result_a = calc_hash("qp");
    println!("Result A {}", result_a);

    let result_b = part_b(&commands);
    println!("Result B {}", result_b);
}

fn part_b(commands: &[&str]) -> u32 {
    let mut lensboxes: Vec<LensBox> = Vec::new();
    for cmd in commands {
        let tok = cmd.split(['-', '=']).collect_vec();
        let mut cmd_type = '-';
        let label = tok[0];
        let mut lense_len: u32 = 0;
        if tok.len() == 2 && tok[1] != "" {
            cmd_type = '=';
            lense_len = tok[1].parse().unwrap();
        }
        let lensbox_nr = calc_hash(label);

        // get or create box
        let mut lensbox;
        if let Some(b) = lensboxes.iter_mut().find(|b| b.nr == lensbox_nr) {
            lensbox = b;
        } else {
            let b = LensBox::new(lensbox_nr);
            lensboxes.push(b);
            lensbox = lensboxes.iter_mut().last().unwrap();
        }

        match cmd_type {
            '-' => lensbox.remove_lense(label),
            '=' => lensbox.add_or_replace_lense(label, lense_len),
            _ => panic!("bad cmd_type"),
        }
    }

    let focusing_power: u32 = lensboxes.iter().map(|lb| lb.calc_focusing_power()).sum();

    focusing_power
}

fn calc_hash(s: &str) -> u32 {
    let mut hash = 0;

    for c in s.chars() {
        let ascii = c as u32;
        hash = hash + ascii;
        hash = hash * 17;
        hash = hash % 256;
    }
    hash
}
