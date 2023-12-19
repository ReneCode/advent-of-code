// day19

use std::{collections::HashMap, str::FromStr};

use itertools::Itertools;

use crate::util::io;

#[derive(Debug, Clone)]
enum Compare {
    Nothing,
    LessThan,
    GreaterThan,
}

const ACTION_ACCEPT: &str = "A";
const ACTION_REJECT: &str = "R";

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

#[derive(Debug, Clone)]
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

        self.prove_value(part_val)
    }

    fn prove_value(&self, val: i32) -> bool {
        match self.compare {
            Compare::Nothing => return true,
            Compare::GreaterThan => return val > self.value,
            Compare::LessThan => return val < self.value,
        }
    }

    fn get_oposite(&self) -> Rule {
        match self.compare {
            Compare::Nothing => panic!("no oposite from compare-nothing"),
            Compare::GreaterThan => Rule {
                attribute: self.attribute.clone(),
                compare: Compare::LessThan,
                value: self.value + 1,
                action: "oposite-action".to_string(),
            },
            Compare::LessThan => Rule {
                attribute: self.attribute.clone(),
                compare: Compare::GreaterThan,
                value: self.value - 1,
                action: "oposite-action".to_string(),
            },
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
    fn new(x: i32, m: i32, a: i32, s: i32) -> Self {
        Part { x, m, a, s }
    }
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

    let result_a: i32 = part_a(&workflows, &parts);
    println!("Result A: {result_a}");

    let result_b = part_b(&workflows);
    println!("Result B: {result_b}");
}

fn part_b(workflows: &HashMap<String, Workflow>) -> usize {
    // traverse the rule-tree and collect all rule-paths
    // that result "A" result.

    let mut active_rules: Vec<Vec<Rule>> = Vec::new();
    let mut reject_rules: Vec<Vec<Rule>> = Vec::new();
    let rules: Vec<Rule> = Vec::new();
    get_rules(
        workflows,
        &mut active_rules,
        &mut reject_rules,
        &rules,
        "in",
    );

    let mut total_combinations = 0;

    for rules in active_rules {
        let mut x_rules: Vec<Rule> = Vec::new();
        let mut m_rules: Vec<Rule> = Vec::new();
        let mut a_rules: Vec<Rule> = Vec::new();
        let mut s_rules: Vec<Rule> = Vec::new();
        for rule in rules {
            match rule.attribute.as_str() {
                "x" => x_rules.push(rule.clone()),
                "m" => m_rules.push(rule.clone()),
                "a" => a_rules.push(rule.clone()),
                "s" => s_rules.push(rule.clone()),
                _ => panic!("bad attribute"),
            }
        }

        // collect all valid values for the 4 dimensions.
        // tiny brute-force to make it easier to deal with
        // multiple rules
        // a little bit hacky, but it works :-)
        let mut x_values: Vec<i32> = Vec::new();
        for x in 1..=4000 {
            if prove_rules(&x_rules, x) {
                x_values.push(x)
            }
        }

        let mut m_values: Vec<i32> = Vec::new();
        for m in 1..=4000 {
            if prove_rules(&m_rules, m) {
                m_values.push(m)
            }
        }

        let mut a_values: Vec<i32> = Vec::new();
        for a in 1..=4000 {
            if prove_rules(&a_rules, a) {
                a_values.push(a)
            }
        }

        let mut s_values: Vec<i32> = Vec::new();
        for s in 1..=4000 {
            if prove_rules(&s_rules, s) {
                s_values.push(s)
            }
        }

        let combinations = x_values.len() * m_values.len() * a_values.len() * s_values.len();
        total_combinations += combinations;
    }

    // for r in reject_rules {
    //     println!("{:?}", r);
    // }

    // for attribute in ["x", "s", "a", "m"] {
    //     println!("{:?}", attribute);
    // }

    total_combinations
}

fn prove_rules(rules: &Vec<Rule>, val: i32) -> bool {
    for rule in rules.iter() {
        if !rule.prove_value(val) {
            return false;
        }
    }
    return true;
}

fn get_rules<'a>(
    workflows: &HashMap<String, Workflow>,
    active_rules: &mut Vec<Vec<Rule>>,
    reject_rules: &mut Vec<Vec<Rule>>,
    rules: &Vec<Rule>,
    workflow_name: &str,
) {
    let workflow = workflows.get(workflow_name).unwrap();
    let mut workflow_rules = rules.clone();
    for rule in workflow.rules.iter() {
        let mut new_rules = workflow_rules.clone();
        match rule.compare {
            Compare::GreaterThan | Compare::LessThan => {
                new_rules.push(rule.clone());

                match rule.action.as_str() {
                    ACTION_ACCEPT => active_rules.push(new_rules),
                    ACTION_REJECT => reject_rules.push(new_rules),
                    _ => get_rules(
                        workflows,
                        active_rules,
                        reject_rules,
                        &new_rules,
                        &rule.action,
                    ),
                }

                let oposite_rule = rule.get_oposite();
                workflow_rules.push(oposite_rule);
            }
            Compare::Nothing => match rule.action.as_str() {
                ACTION_ACCEPT => active_rules.push(new_rules),
                ACTION_REJECT => reject_rules.push(new_rules),
                _ => get_rules(
                    workflows,
                    active_rules,
                    reject_rules,
                    &new_rules,
                    &rule.action,
                ),
            },
        }
    }
}

fn part_a(workflows: &HashMap<String, Workflow>, parts: &[Part]) -> i32 {
    let mut total_values = 0;

    for part in parts {
        total_values += calc_result(workflows, part)
    }

    total_values
}

fn calc_result(workflows: &HashMap<String, Workflow>, part: &Part) -> i32 {
    let mut workflow_name = "in";
    let mut total_value: i32 = 0;
    loop {
        let workflow = workflows.get(workflow_name).unwrap();
        let act = workflow.process(part);
        match act {
            ACTION_ACCEPT => {
                total_value += part.total_value();
                break;
            }
            ACTION_REJECT => break,
            _ => workflow_name = act,
        }
    }
    total_value
}
