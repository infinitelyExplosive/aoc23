use std::fs;

fn hash(data: &[u8]) -> u8 {
    let mut acc: u8 = 0;
    for val in data {
        acc = acc.wrapping_add(*val);
        acc = acc.wrapping_mul(17);
    }
    return acc;
}

pub fn part_a() {
    if let Ok(data) = fs::read_to_string("day15.txt") {
        let parts = data.trim().split(',');
        let mut sum = 0;
        for part in parts {
            sum += hash(part.as_bytes()) as i64;
        }
        println!("{}", sum)
    }
}

pub fn part_b() {
    
}