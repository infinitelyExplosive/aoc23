use regex::Regex;
use std::{collections::HashMap, fs};

pub fn part_a() {
    if let Ok(data) = fs::read_to_string("day8.txt") {
        let mut lines = data.lines();
        let path = lines.next().unwrap().as_bytes();
        lines.next();

        let mut map: HashMap<&str, (&str, &str)> = HashMap::new();
        for line in lines {
            let re = Regex::new(r"(\w{3}) = \((\w{3}), (\w{3})\)").unwrap();
            let captures = re.captures(line).unwrap();
            let position = captures.get(1).unwrap().as_str();
            map.insert(
                position,
                (
                    captures.get(2).unwrap().as_str(),
                    captures.get(3).unwrap().as_str(),
                ),
            );
        }

        let mut position = "AAA";
        let mut count = 0;
        while position != "ZZZ" {
            let left = path[count % path.len()] == b'L';
            position = if left {
                map.get(&position).unwrap().0
            } else {
                map.get(&position).unwrap().1
            };
            count += 1;
        }
        println!("{}", count);
    }
}

fn gcd(a: i64, b:i64) -> i64 {
    if b == 0 {
        return a;
    } else {
        return gcd(b, a % b);
    }
}
fn lcm (a: i64, b:i64) -> i64 {
    return a * b / gcd(a, b);
}
pub fn part_b() {
    if let Ok(data) = fs::read_to_string("day8.txt") {
        let mut lines = data.lines();
        let path = lines.next().unwrap().as_bytes();
        lines.next();

        let mut map: HashMap<&str, (&str, &str)> = HashMap::new();
        let mut positions = Vec::new();
        for line in lines {
            let re = Regex::new(r"(\w{3}) = \((\w{3}), (\w{3})\)").unwrap();
            let captures = re.captures(line).unwrap();
            let position = captures.get(1).unwrap().as_str();
            map.insert(
                position,
                (
                    captures.get(2).unwrap().as_str(),
                    captures.get(3).unwrap().as_str(),
                ),
            );
            if position.ends_with("A") {
                positions.push(position);
            }
        }

        let mut cycles = Vec::new();

        for mut position in positions {
            let mut count = 0;
            while !position.ends_with("Z") {
                let left = path[count as usize % path.len()] == b'L';
                position = if left {
                    map.get(&position).unwrap().0
                } else {
                    map.get(&position).unwrap().1
                };
                count += 1;
            }
            cycles.push(count);
        }

        let mut result = 1;
        for cycle in cycles {
            result = lcm(result, cycle)
        }
        println!("{}", result);
    }
}
