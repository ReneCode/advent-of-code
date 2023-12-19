// day19

use std::{collections::HashMap, str::FromStr};

use itertools::Itertools;

use crate::util::io;

#[derive(Debug)]
enum Compare {
    Nothing,
    LessThan,
    GreaterThan,
}

#[derive(Debug)]
enum Action {
    Accept,
    Reject,
    Go(String),
    CompareAndGo(String),
}

#[derive(Debug)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

struct ParseWorkflowError;
impl FromStr for Workflow {
    type Err = ParseWorkflowError;

    // "px{a<2006:qkq,m>2090:A,rfg}"
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, rules) = s
            .strip_suffix('}')
            .and_then(|s| s.split_once('{'))
            .ok_or(ParseWorkflowError)?;
        // let rules = rest.strip_prefix('{').unwrap();
        let tok = rules.split(',').collect_vec();

        let mut rules: Vec<Rule> = Vec::new();
        for rule_str in tok {
            if let Ok(rule) = Rule::from_str(rule_str) {
                rules.push(rule);
            }
        }

        Ok(Workflow {
            name: name.to_string(),
            rules: rules,
        })
    }
}

#[derive(Debug)]
struct Rule {
    attribute: String,
    compare: Compare,
    value: i32,
    action: Action,
}

struct ParseRuleError;
impl FromStr for Rule {
    type Err = ParseRuleError;

    // "x>10:one"
    // "R", "A", "lnx"
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tok = s.split(':').collect_vec();
        if tok.len() == 1 {
            let name = tok[0];
            let mut action: Action = Action::Go(name.to_string());
            if name == "A" {
                action = Action::Accept
            } else if name == "R" {
                action = Action::Reject
            }
            return Ok(Rule {
                attribute: String::new(),
                compare: Compare::Nothing,
                value: 0,
                action: action,
            });
        } else {
            let action = tok[1];

            let mut compare = Compare::LessThan;
            if tok[0].contains('<') {
                tok = tok[0].split('<').collect_vec();
                compare = Compare::LessThan
            } else {
                tok = tok[0].split('>').collect_vec();
                compare = Compare::GreaterThan
            }
            let attribute = tok[0];
            let value: i32 = tok[1].parse().unwrap();
            return Ok(Rule {
                attribute: attribute.to_string(),
                compare: compare,
                value: value,
                action: Action::CompareAndGo(action.to_string()),
            });
        }
    }
}

#[derive(Debug)]
struct Part {
    x: i32,
    m: i32,
    a: i32,
    s: i32,
}

struct ParsePartError;
impl FromStr for Part {
    type Err = ParsePartError;

    // "{x=787,m=2655,a=1222,s=2876}"
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut attributes = s
            .strip_prefix('{')
            .and_then(|s| s.strip_suffix('}'))
            .ok_or(ParsePartError)?;
        let tok = attributes.split(',').collect_vec();
        let mut part = Part {
            x: 0,
            m: 0,
            a: 0,
            s: 0,
        };
        for t in tok {
            let kv = t.split('=').collect_vec();
            match kv[0] {
                "x" => part.x = kv[1].parse().unwrap(),
                "m" => part.m = kv[1].parse().unwrap(),
                "a" => part.a = kv[1].parse().unwrap(),
                "s" => part.s = kv[1].parse().unwrap(),
                _ => panic!("bad attribute {}", kv[0]),
            }
        }
        Ok(part)
    }
}

pub fn day19() {
    println!("hello day19");

    let lines = io::read_lines("./src/day19/19-example.data").unwrap();
    let mut workflows: HashMap<String, Workflow> = HashMap::new();
    let mut parts: Vec<Part> = Vec::new();
    let mut parse_workflow = true;
    for line in lines {
        if line.len() == 0 {
            parse_workflow = false;
            continue;
        }
        if parse_workflow {
            if let Ok(workflow) = Workflow::from_str(line.as_str()) {
                workflows.insert(workflow.name.clone(), workflow);
            } else {
                panic!("bad workflow")
            }
        } else {
            if let Ok(part) = Part::from_str(line.as_str()) {
                parts.push(part)
            }
        }
    }

    println!("{:?}", workflows);
    println!("{:?}", parts);
}
