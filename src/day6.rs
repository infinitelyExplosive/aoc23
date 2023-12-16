use std::{fs, iter};

fn quadratic_solns(a: i64, b: i64) -> i64 {
    let a = a as f64;
    let b = b as f64;

    let sqrt = (a.powi(2) - 4f64 * b).sqrt();
    let min = (a - sqrt) / 2f64;
    let max = (a + sqrt) / 2f64;
    println!(" {} {}", min, max);
    let min_int = min as i64 + 1;
    let max_int = if max.fract() == 0.0 {
        max as i64 - 1
    } else {
        max as i64
    };
    println!(" {} {}", min_int, max_int);
    return max_int - min_int + 1;
}
pub fn part_a() {
    if let Ok(data) = fs::read_to_string("day6.txt") {
        let mut lines = data.lines();
        let times: Vec<i64> = lines
            .next()
            .unwrap()
            .split(':')
            .skip(1)
            .next()
            .unwrap()
            .split(' ')
            .filter(|s| s.len() > 0)
            .map(|s| i64::from_str_radix(s, 10).unwrap())
            .collect();

        let distances: Vec<i64> = lines
            .next()
            .unwrap()
            .split(':')
            .skip(1)
            .next()
            .unwrap()
            .split(' ')
            .filter(|s| s.len() > 0)
            .map(|s| i64::from_str_radix(s, 10).unwrap())
            .collect();

        let mut prod = 1;

        for (a, b) in iter::zip(times, distances) {
            let size = quadratic_solns(a, b);
            println!(" {}", size);
            prod *= size;
        }
        println!("{}", prod);
    }
}

pub fn part_b() {
    if let Ok(data) = fs::read_to_string("day6.txt") {
        let mut lines = data.lines();
        let time = i64::from_str_radix(
            lines
                .next()
                .unwrap()
                .replace(" ", "")
                .split(':')
                .skip(1)
                .next()
                .unwrap(),
            10,
        ).unwrap();
        let distance= i64::from_str_radix(
            lines
                .next()
                .unwrap()
                .replace(" ", "")
                .split(':')
                .skip(1)
                .next()
                .unwrap(),
            10,
        ).unwrap();
        let result = quadratic_solns(time, distance);
        println!("{}", result);
    }
}
