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

const ACTION_ACCEPT: &str = "A";
const ACTION_REJECT: &str = "R";

// #[derive(Debug, Clone)]
// enum Action {
//     Accept,
//     Reject,
//     Go(String),
//     CompareAndGo(String),
// }

// struct ParseActionError;
// impl FromStr for Action {
//     type Err = ParseActionError;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         let act = match s {
//             "A" => Action::Accept,
//             "R" => Action::Reject,
//             _ => Action::Go(s.to_string()),
//         };
//         Ok(act)
//     }
// }

#[derive(Debug)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}
impl Workflow {
    fn process(&self, part: &Part) -> &str {
        for rule in self.rules.iter() {
            match rule.compare {
                Compare::Nothing => return &rule.action,
                Compare::GreaterThan | Compare::LessThan => {
                    if rule.prove(part) {
                        return &rule.action;
                    }
                }
            }
        }
        panic!("process does not find action")
    }
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
    action: String,
}
impl Rule {
    fn prove(&self, part: &Part) -> bool {
        let part_val = match self.attribute.as_str() {
            "x" => part.x,
            "m" => part.m,
            "a" => part.a,
            "s" => part.s,
            _ => panic!("bad rule attribute {}", self.attribute),
        };

        match self.compare {
            Compare::Nothing => return true,
            Compare::GreaterThan => return part_val > self.value,
            Compare::LessThan => return part_val < self.value,
        }
    }
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
            return Ok(Rule {
                attribute: String::new(),
                compare: Compare::Nothing,
                value: 0,
                action: name.to_string(),
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
                action: action.to_string(),
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
impl Part {
    fn total_value(&self) -> i32 {
        self.x + self.m + self.a + self.s
    }
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

    let lines = io::read_lines("./src/day19/19.data").unwrap();
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

    // println!("{:?}", workflows);
    // println!("{:?}", parts);

    let result_a: i32 = part_a(&workflows, &parts);
    println!("Result A: {result_a}")
}

fn part_a(workflows: &HashMap<String, Workflow>, parts: &[Part]) -> i32 {
    let mut total_values = 0;

    for part in parts {
        let mut workflow_name = "in";

        loop {
            let workflow = workflows.get(workflow_name).unwrap();
            let act = workflow.process(part);
            match act {
                ACTION_ACCEPT => {
                    total_values += part.total_value();
                    break;
                }
                ACTION_REJECT => break,
                _ => workflow_name = act,
            }
        }
    }

    total_values
}
