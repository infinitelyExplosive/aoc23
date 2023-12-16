use std::fs;

pub fn part_a() {
    let mut sum = 0;
    if let Ok(data) = fs::read_to_string("day1.txt") {
        let lines = data.lines();
        for line in lines {
            let mut first = None;
            let mut last = None;
            for letter in line.chars().into_iter() {
                if letter.is_numeric() {
                    if first.is_none() {
                        first = Some(letter);
                    } else {
                        last = Some(letter);
                    }
                }
            }
            if last.is_none() {
                last = first;
            }

            let mut val_str = String::from(first.unwrap());
            val_str.push(last.unwrap());
            sum += i64::from_str_radix(&val_str, 10).unwrap();
        }

        println!("{}", sum);
    }
}

pub fn part_b() {
    let mut sum = 0;
    if let Ok(data) = fs::read_to_string("day1.txt") {
        let lines = data.lines();
        for line in lines {
            let mut first = None;
            let mut last = None;
            for i in 0..line.len() {
                let mut parsed_val = None;
                let cur_char = line.chars().nth(i).unwrap();
                if cur_char.is_numeric() {
                    parsed_val = Some(cur_char);
                }
                if line.chars().skip(i).take(3).collect::<String>() == "one" {
                    parsed_val = Some('1');
                }
                if line.chars().skip(i).take(3).collect::<String>() == "two" {
                    parsed_val = Some('2');
                }
                if line.chars().skip(i).take(5).collect::<String>() == "three" {
                    parsed_val = Some('3');
                }
                if line.chars().skip(i).take(4).collect::<String>() == "four" {
                    parsed_val = Some('4');
                }
                if line.chars().skip(i).take(4).collect::<String>() == "five" {
                    parsed_val = Some('5');
                }
                if line.chars().skip(i).take(3).collect::<String>() == "six" {
                    parsed_val = Some('6');
                }
                if line.chars().skip(i).take(5).collect::<String>() == "seven" {
                    parsed_val = Some('7');
                }
                if line.chars().skip(i).take(5).collect::<String>() == "eight" {
                    parsed_val = Some('8');
                }
                if line.chars().skip(i).take(4).collect::<String>() == "nine" {
                    parsed_val = Some('9');
                }

                if parsed_val.is_some() {
                    if first.is_none() {
                        first = parsed_val;
                    } else {
                        last = parsed_val;
                    }
                }
            }
            if last.is_none() {
                last = first;
            }

            let mut val_str = String::from(first.unwrap());
            val_str.push(last.unwrap());
            // println!("{}", val_str);
            sum += i64::from_str_radix(&val_str, 10).unwrap();
        }

        println!("{}", sum);
    }
}
