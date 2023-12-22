use std::{collections::HashMap, fs};

struct Part {
    x: i64,
    m: i64,
    a: i64,
    s: i64,
}

struct Rule {
    category: char,
    is_greater: bool,
    thresh: i64,
    dest: String,
}

fn eval_rule(
    part: &Part,
    rule: &Rule,
    chains: &HashMap<String, (Vec<Rule>, String)>,
) -> Option<bool> {
    let val = match rule.category {
        'x' => part.x,
        'm' => part.m,
        'a' => part.a,
        's' => part.s,
        _ => 0,
    };
    if rule.is_greater {
        if val > rule.thresh {
            if rule.dest == "A" {
                return Some(true);
            } else if rule.dest == "R" {
                return Some(false);
            } else {
                return eval_chain(part, &rule.dest, chains);
            }
        };
    } else if val < rule.thresh {
        if rule.dest == "A" {
            return Some(true);
        } else if rule.dest == "R" {
            return Some(false);
        } else {
            return eval_chain(part, &rule.dest, chains);
        };
    };
    return None;
}

fn eval_chain(
    part: &Part,
    chain_name: &String,
    chains: &HashMap<String, (Vec<Rule>, String)>,
) -> Option<bool> {
    let chain = chains.get(chain_name).unwrap();
    for rule in &chain.0 {
        if let Some(result) = eval_rule(part, rule, chains) {
            return Some(result);
        }
    }
    if chain.1 == "A" {
        return Some(true);
    } else if chain.1 == "R" {
        return Some(false);
    } else {
        return eval_chain(part, &chain.1, chains);
    }
}

pub fn part_a() {
    if let Ok(data) = fs::read_to_string("day19.txt") {
        let mut chains = HashMap::new();
        let mut parts = Vec::new();

        let mut lines = data.lines();
        while let Some(entry) = lines.next() {
            if entry.len() == 0 {
                break;
            }
            let mut split = entry.split('{');
            let name = split.next().unwrap().to_string();
            let mut split = split.next().unwrap().split('}').next().unwrap().split(',');
            let mut chain = Vec::new();
            let mut default = "R".to_string();
            while let Some(rule_txt) = split.next() {
                if rule_txt.contains('<') {
                    let mut subsplit = rule_txt.split('<');
                    let category = subsplit.next().unwrap().chars().next().unwrap();
                    let mut subsplit = subsplit.next().unwrap().split(':');
                    let thresh = i64::from_str_radix(subsplit.next().unwrap(), 10).unwrap();
                    let dest = subsplit.next().unwrap().to_string();
                    chain.push(Rule {
                        category: category,
                        is_greater: false,
                        thresh: thresh,
                        dest: dest,
                    });
                } else if rule_txt.contains('>') {
                    let mut subsplit = rule_txt.split('>');
                    let category = subsplit.next().unwrap().chars().next().unwrap();
                    let mut subsplit = subsplit.next().unwrap().split(':');
                    let thresh = i64::from_str_radix(subsplit.next().unwrap(), 10).unwrap();
                    let dest = subsplit.next().unwrap().to_string();
                    chain.push(Rule {
                        category: category,
                        is_greater: true,
                        thresh: thresh,
                        dest: dest,
                    });
                } else {
                    default = rule_txt.to_string();
                }
            }
            chains.insert(name, (chain, default));
        }

        while let Some(part) = lines.next() {
            let mut split = part
                .split('{')
                .skip(1)
                .next()
                .unwrap()
                .split('}')
                .next()
                .unwrap()
                .split(',');

            let x =
                i64::from_str_radix(split.next().unwrap().split('=').nth(1).unwrap(), 10).unwrap();
            let m =
                i64::from_str_radix(split.next().unwrap().split('=').nth(1).unwrap(), 10).unwrap();
            let a =
                i64::from_str_radix(split.next().unwrap().split('=').nth(1).unwrap(), 10).unwrap();
            let s =
                i64::from_str_radix(split.next().unwrap().split('=').nth(1).unwrap(), 10).unwrap();
            parts.push(Part {
                x: x,
                m: m,
                a: a,
                s: s,
            });
        }

        let mut sum = 0;
        for part in parts {
            if let Some(result) = eval_chain(&part, &"in".to_string(), &chains) {
                if result {
                    sum += part.x + part.m + part.a + part.s;
                }
            }
        }
        println!("{}", sum);
    }
}

#[derive(Clone, Copy, Default)]
struct PartRange {
    xmin: i64,
    xmax: i64,
    mmin: i64,
    mmax: i64,
    amin: i64,
    amax: i64,
    smin: i64,
    smax: i64,
}

fn new_ranges(range: PartRange, rule: &Rule) -> (PartRange, PartRange) {
    let (min, max) = match rule.category {
        'x' => (range.xmin, range.xmax),
        'm' => (range.mmin, range.mmax),
        'a' => (range.amin, range.amax),
        's' => (range.smin, range.smax),
        _ => {
            panic!("invalid category")
        }
    };
    let thresh = rule.thresh;
    if thresh <= min {
        if rule.is_greater {
            return (Default::default(), range);
        } else {
            return (range, Default::default());
        }
    } else if thresh > max {
        return (range, Default::default());
    } else {
        if rule.is_greater {
            return match rule.category {
                'x' => (
                    PartRange {
                        xmax: thresh - 1,
                        ..range
                    },
                    PartRange {
                        xmin: thresh,
                        ..range
                    },
                ),
                'm' => (
                    PartRange {
                        mmax: thresh - 1,
                        ..range
                    },
                    PartRange {
                        mmin: thresh,
                        ..range
                    },
                ),
                'a' => (
                    PartRange {
                        amax: thresh - 1,
                        ..range
                    },
                    PartRange {
                        amin: thresh,
                        ..range
                    },
                ),
                's' => (
                    PartRange {
                        smax: thresh - 1,
                        ..range
                    },
                    PartRange {
                        smin: thresh,
                        ..range
                    },
                ),
                _ => {
                    panic!("invalid category")
                }
            };
        } else {
            return match rule.category {
                'x' => (
                    PartRange {
                        xmax: thresh - 1,
                        ..range
                    },
                    PartRange {
                        xmin: thresh,
                        ..range
                    },
                ),
                'm' => (
                    PartRange {
                        mmax: thresh - 1,
                        ..range
                    },
                    PartRange {
                        mmin: thresh,
                        ..range
                    },
                ),
                'a' => (
                    PartRange {
                        amax: thresh - 1,
                        ..range
                    },
                    PartRange {
                        amin: thresh,
                        ..range
                    },
                ),
                's' => (
                    PartRange {
                        smax: thresh - 1,
                        ..range
                    },
                    PartRange {
                        smin: thresh,
                        ..range
                    },
                ),
                _ => {
                    panic!("invalid category")
                }
            };
        }
    }
}

fn range_size(range: &PartRange) -> i64 {
    return (range.xmax - range.xmin)
        * (range.mmax - range.mmin)
        * (range.amax - range.amin)
        * (range.smax - range.smin);
}

fn trace_rule(
    range: PartRange,
    rule: &Rule,
    chains: &HashMap<String, (Vec<Rule>, String)>,
    count: &mut i64,
) {
    if rule.is_greater {
        let (r1, r2) = new_ranges(range, rule);
        if rule.dest == "A" {
            *count += range_size(&r1);
        } else if rule.dest == "R" {
            Some(false);
        } else {
            trace_chain(range, &rule.dest, chains, count);
        }
    } else {
        if rule.dest == "A" {
            Some(true);
        } else if rule.dest == "R" {
            Some(false);
        } else {
            trace_chain(range, &rule.dest, chains, count);
        };
    };
}

fn trace_chain(
    range: PartRange,
    chain_name: &String,
    chains: &HashMap<String, (Vec<Rule>, String)>,
    count: &mut i64,
) {
    let chain = chains.get(chain_name).unwrap();
    for rule in &chain.0 {
        trace_rule(range, rule, chains, count);
    }
    if chain.1 == "A" {
        Some(true);
    } else if chain.1 == "R" {
        Some(false);
    } else {
        trace_chain(range, &chain.1, chains, count);
    }
}
pub fn part_b() {
    if let Ok(data) = fs::read_to_string("day19.txt") {
        let mut chains = HashMap::new();
        let mut parts = Vec::new();

        let mut lines = data.lines();
        while let Some(entry) = lines.next() {
            if entry.len() == 0 {
                break;
            }
            let mut split = entry.split('{');
            let name = split.next().unwrap().to_string();
            let mut split = split.next().unwrap().split('}').next().unwrap().split(',');
            let mut chain = Vec::new();
            let mut default = "R".to_string();
            while let Some(rule_txt) = split.next() {
                if rule_txt.contains('<') {
                    let mut subsplit = rule_txt.split('<');
                    let category = subsplit.next().unwrap().chars().next().unwrap();
                    let mut subsplit = subsplit.next().unwrap().split(':');
                    let thresh = i64::from_str_radix(subsplit.next().unwrap(), 10).unwrap();
                    let dest = subsplit.next().unwrap().to_string();
                    chain.push(Rule {
                        category: category,
                        is_greater: false,
                        thresh: thresh,
                        dest: dest,
                    });
                } else if rule_txt.contains('>') {
                    let mut subsplit = rule_txt.split('>');
                    let category = subsplit.next().unwrap().chars().next().unwrap();
                    let mut subsplit = subsplit.next().unwrap().split(':');
                    let thresh = i64::from_str_radix(subsplit.next().unwrap(), 10).unwrap();
                    let dest = subsplit.next().unwrap().to_string();
                    chain.push(Rule {
                        category: category,
                        is_greater: true,
                        thresh: thresh,
                        dest: dest,
                    });
                } else {
                    default = rule_txt.to_string();
                }
            }
            chains.insert(name, (chain, default));
        }

        while let Some(part) = lines.next() {
            let mut split = part
                .split('{')
                .skip(1)
                .next()
                .unwrap()
                .split('}')
                .next()
                .unwrap()
                .split(',');

            let x =
                i64::from_str_radix(split.next().unwrap().split('=').nth(1).unwrap(), 10).unwrap();
            let m =
                i64::from_str_radix(split.next().unwrap().split('=').nth(1).unwrap(), 10).unwrap();
            let a =
                i64::from_str_radix(split.next().unwrap().split('=').nth(1).unwrap(), 10).unwrap();
            let s =
                i64::from_str_radix(split.next().unwrap().split('=').nth(1).unwrap(), 10).unwrap();
            parts.push(Part {
                x: x,
                m: m,
                a: a,
                s: s,
            });
        }

        let mut sum = 0;
        for part in parts {
            if let Some(result) = eval_chain(&part, &"in".to_string(), &chains) {
                if result {
                    sum += part.x + part.m + part.a + part.s;
                }
            }
        }
        println!("{}", sum);
    }
}
