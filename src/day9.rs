use std::fs;

pub fn extend(layer: &Vec<i64>) -> i64 {
    let mut dif = Vec::new();
    for i in 1..layer.len() {
        dif.push(layer[i] - layer[i-1]);
    }
    if dif.iter().all(|x| *x == 0) {
        return 0;
    } else {
        return dif.last().unwrap() + extend(&dif);
    }
}

pub fn rev_extend(layer: &Vec<i64>) -> i64 {
    let mut dif = Vec::new();
    for i in 1..layer.len() {
        dif.push(layer[i] - layer[i-1]);
    }
    if dif.iter().all(|x| *x == 0) {
        return 0;
    } else {
        return dif.first().unwrap() - rev_extend(&dif);
    }
}

pub fn part_a() {
    if let Ok(data) = fs::read_to_string("day9.txt") {
        let mut sum = 0;
        for line in data.lines() {
            let nums: Vec<i64> = line
                .split(' ')
                .filter(|s| s.len() > 0)
                .map(|s| i64::from_str_radix(s, 10).unwrap())
                .collect();
            sum += nums.last().unwrap() + extend(&nums);
        }
        println!("{}", sum);
    }
}

pub fn part_b() {
    if let Ok(data) = fs::read_to_string("day9.txt") {
        let mut sum = 0;
        for line in data.lines() {
            let nums: Vec<i64> = line
                .split(' ')
                .filter(|s| s.len() > 0)
                .map(|s| i64::from_str_radix(s, 10).unwrap())
                .collect();
            sum += nums.first().unwrap() - rev_extend(&nums);
        }
        println!("{}", sum);
    }
}