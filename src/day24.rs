use std::{fs, process::exit};

#[derive(Debug)]
struct Stone {
    x: i64,
    y: i64,
    vx: i64,
    vy: i64,
}

fn intersection(a: &Stone, b: &Stone) -> (f64, f64) {
    if ((a.vx as f64 / a.vy as f64) - (b.vx as f64 / b.vy as f64)).abs() < 0.0000001f64 {
        return (f64::NAN, f64::NAN);
    }
    let t_top = (b.y * b.vx) + (b.vy * a.x) - (b.vy * b.x) - (a.y * b.vx);
    let t_bot = (b.vx * a.vy) - (b.vy * a.vx);
    let t = t_top as f64 / t_bot as f64;
    let s = (a.x as f64 + a.vx as f64 * t - b.x as f64) / b.vx as f64;

    if t > 0f64 && s > 0f64 {
        return (a.x as f64 + a.vx as f64 * t, a.y as f64 + a.vy as f64 * t);
    } else {
        return (f64::NAN, f64::NAN);
    }
}
pub fn part_a() {
    if let Ok(data) = fs::read_to_string("day24.txt") {
        let mut stones = Vec::new();
        for line in data.lines() {
            let mut parts = line.split(" @ ");
            let mut nums = parts.next().unwrap().split(", ");
            let x = i64::from_str_radix(nums.next().unwrap().trim(), 10).unwrap();
            let y = i64::from_str_radix(nums.next().unwrap().trim(), 10).unwrap();
            let mut vels = parts.next().unwrap().split(", ");
            let vx = i64::from_str_radix(vels.next().unwrap().trim(), 10).unwrap();
            let vy = i64::from_str_radix(vels.next().unwrap().trim(), 10).unwrap();
            stones.push(Stone { x, y, vx, vy });
        }

        let mut sum = 0;

        let min_coord = 200000000000000f64;
        let max_coord = 400000000000000f64;
        for i in 0..(stones.len() - 1) {
            for j in i + 1..stones.len() {
                let (inter_x, inter_y) = intersection(&stones[i], &stones[j]);
                if inter_x.is_finite()
                    && inter_y.is_finite()
                    && min_coord <= inter_x
                    && inter_x <= max_coord
                    && min_coord <= inter_y
                    && inter_y <= max_coord
                {
                    sum += 1;
                }
            }
        }
        println!("{}", sum);
    }
}

pub fn part_b() {
    if let Ok(data) = fs::read_to_string("day24.txt") {
        let mut i = 1;
        println!("var(\"a b c d e f t1 t2 t3\")");
        for line in data.lines() {
            let mut parts = line.split("@");
            let mut nums = parts.next().unwrap().split(",");
            let x = i64::from_str_radix(nums.next().unwrap().trim(), 10).unwrap();
            let y = i64::from_str_radix(nums.next().unwrap().trim(), 10).unwrap();
            let z = i64::from_str_radix(nums.next().unwrap().trim(), 10).unwrap();
            let mut vels = parts.next().unwrap().split(", ");
            let vx = i64::from_str_radix(vels.next().unwrap().trim(), 10).unwrap();
            let vy = i64::from_str_radix(vels.next().unwrap().trim(), 10).unwrap();
            let vz = i64::from_str_radix(vels.next().unwrap().trim(), 10).unwrap();
            let j = (i+2)/3;
            println!("e{} = a+b*t{} == {} + ({}) * t{}", i, j, x, vx, j);
            i += 1;
            println!("e{} = c+d*t{} == {} + ({}) * t{}", i, j, y, vy, j);
            i += 1;
            println!("e{} = e+f*t{} == {} + ({}) * t{}", i, j, z, vz, j);
            i += 1;
            if i > 9 {
                break;
            }
        }
        println!("result = solve([e1,e2,e3,e4,e5,e6,e7,e8,e9], a,b,c,d,e,f,t1,t2,t3)");
        println!("result[0][0].rhs() + result[0][2].rhs() + result[0][4].rhs()");

        
    }
}
