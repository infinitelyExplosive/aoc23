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
    if let Ok(data) = fs::read_to_string("day15.txt") {
        let mut boxes: Vec<Vec<(&str, i64)>> = vec![Vec::new(); 256];
        let parts = data.trim().split(',');

        for part in parts {
            if part.ends_with('-') {
                let label = &part[0..part.len() - 1];
                let idx = hash(label.as_bytes());
                let box_val = &mut boxes[idx as usize];
                if let Some(sub_idx) = box_val.iter().position(|s| s.0 == label) {
                    box_val.remove(sub_idx);
                }
            } else {
                let mut part_split = part.split('=');
                let label = part_split.next().unwrap();
                let val = i64::from_str_radix(part_split.next().unwrap(), 10).unwrap();
                let idx = hash(label.as_bytes());
                let box_val = &mut boxes[idx as usize];
                if let Some(sub_idx) = box_val.into_iter().position(|s| s.0 == label) {
                    box_val[sub_idx] = (label, val);
                } else {
                    box_val.push((label, val));
                };
            }
        }

        let mut sum = 0;
        for (i, box_val) in boxes.iter().enumerate() {
            for (j, label) in box_val.iter().enumerate() {
                sum += (i as i64 + 1) * (j as i64 + 1) * label.1;
            }
        }
        println!("{}", sum);
    }
}
