use std::fs;

pub fn part_a() {
    let mut sum = 0;
    if let Ok(data) = fs::read_to_string("day4.txt") {
        for line in data.lines() {
            let split = line.split(":");
            let nums = split.skip(1).next().unwrap();
            let (wins, haves) = nums.split_once("|").unwrap();
            let wins: Vec<&str> = wins.split(' ').collect();
            let haves: Vec<&str> = haves.split(' ').collect();

            let score_vec: Vec<&str> = wins
                .into_iter()
                .filter(|s| s.len() > 0 && haves.contains(s))
                .collect();
            if score_vec.len() > 0 {
                sum += 2u32.pow((score_vec.len() - 1) as u32);
            }
        }
    }
    println!("{}", sum);
}

pub fn part_b() {
    if let Ok(data) = fs::read_to_string("day4.txt") {
        let mut counts = Vec::new();
        for _ in 0..data.lines().count() {
            counts.push(1i128);
        }
        for (i, line) in data.lines().enumerate() {
            let split = line.split(":");
            let nums = split.skip(1).next().unwrap();
            let (wins, haves) = nums.split_once("|").unwrap();
            let wins: Vec<&str> = wins.split(' ').collect();
            let haves: Vec<&str> = haves.split(' ').collect();

            let score_vec: Vec<&str> = wins
                .into_iter()
                .filter(|s| s.len() > 0 && haves.contains(s))
                .collect();
            // println!("{} {}<<<", counts[i], i);
            if score_vec.len() > 0 {
                for idx in (i + 1)..=(i + score_vec.len()) {
                    // println!(" {}({})", counts[idx], idx);
                    counts[idx] += counts[i];
                }
            }
        }
        let score: i128 = counts.iter().sum();
        println!("{}", score);
    }
}
