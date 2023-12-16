use regex::Regex;
use std::fs;

pub fn part_a() {
    let mut sum = 0;
    if let Ok(data) = fs::read_to_string("day2.txt") {
        let lines = data.lines();
        for line in lines {
            let mut works = true;
            let mut i = 0;
            let re = Regex::new(r"Game (\d+)").unwrap();
            let caps = re.captures(line).unwrap();
            let index = i64::from_str_radix(&caps[1], 10).unwrap();
            i += caps[0].len();
            let line: String = line.chars().skip(i).collect();
            
            let re = Regex::new(r"(\d+) (red|blue|green)").unwrap();
            for (_, [count, color]) in re.captures_iter(&line).map(|c|c.extract()) {
                let count = i64::from_str_radix(count, 10).unwrap();
                if color == "red" {
                    if count > 12 {
                        works &= false;
                    }
                } else if color == "green" {
                    if count > 13 {
                        works &= false;
                    }
                } else if color == "blue" {
                    if count > 14 {
                        works &= false;
                    }
                }
            }
            if works {
                sum += index;
            }



        }
        println!("{}", sum);
    }
}

pub fn part_b() {
    let mut sum = 0;
    if let Ok(data) = fs::read_to_string("day2.txt") {
        let lines = data.lines();
        for line in lines {
            let mut red = 0;
            let mut blue = 0;
            let mut green = 0;

            let mut i = 0;
            let re = Regex::new(r"Game (\d+)").unwrap();
            let caps = re.captures(line).unwrap();
            i += caps[0].len();
            let line: String = line.chars().skip(i).collect();
            
            let re = Regex::new(r"(\d+) (red|blue|green)").unwrap();
            for (_, [count, color]) in re.captures_iter(&line).map(|c|c.extract()) {
                let count = i64::from_str_radix(count, 10).unwrap();
                if color == "red" {
                    red = std::cmp::max(red, count);
                } else if color == "green" {
                    green = std::cmp::max(green, count);
                } else if color == "blue" {
                    blue = std::cmp::max(blue, count);
                }
            }
            
            sum += red*green*blue;



        }
        println!("{}", sum);
    }
}
