use std::fs;

struct MapRange {
    from: i64,
    to: i64,
    len: i64,
}

impl<T> From<T> for MapRange
where
    T: Iterator<Item = i64>,
{
    fn from(mut value: T) -> Self {
        MapRange {
            to: value.next().unwrap(),
            from: value.next().unwrap(),
            len: value.next().unwrap(),
        }
    }
}

fn map_index(index: i64, map: &Vec<MapRange>) -> i64 {
    for range in map {
        if range.from <= index && index < range.from + range.len {
            return index - range.from + range.to;
        }
    }
    return index;
}

fn apply_maps(mut index: i64, maps: &Vec<Vec<MapRange>>) -> i64 {
    for map in maps {
        print!(" {}", index);
        index = map_index(index, map);
    }
    print!(" {}\n", index);
    return index;
}

fn inv_map_index(i: i64, map: &Vec<MapRange>) -> i64 {
    for range in map {
        if range.to <= i && i < range.to + range.len {
            return i - range.to + range.from;
        }
    }
    return i;
}

fn inv_apply_maps(mut i: i64, maps: &Vec<Vec<MapRange>>) -> i64 {
    for map in maps.iter().rev() {
        i = inv_map_index(i, map);
    }
    return i;
}

pub fn part_a() {
    if let Ok(data) = fs::read_to_string("day5.txt") {
        let mut line_iter = data.lines();
        let targets: Vec<i64> = line_iter
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

        let mut mappings: Vec<Vec<MapRange>> = Vec::new();
        let mut mapping = Vec::new();

        line_iter.next();
        line_iter.next();
        loop {

            let line = line_iter.next();
            if line.is_none() {
                break;
            }
            let line = line.unwrap();

            if line.len() > 0 {
                let parts = line
                    .split(' ')
                    .map(|s| i64::from_str_radix(s, 10).unwrap())
                    .into();
                mapping.push(parts);
            } else {
                mappings.push(mapping);
                mapping = Vec::new();
                line_iter.next();
            }
        }
        mappings.push(mapping);

        let min = targets
            .iter()
            .map(|i| apply_maps(*i, &mappings))
            .min()
            .unwrap();
        println!("{}", min);
    }
}

fn in_range(i: i64, start: i64, len:i64) -> bool {
    return i >= start && i < start + len;
}
pub fn part_b() {
    if let Ok(data) = fs::read_to_string("day5.txt") {
        let mut line_iter = data.lines();
        let targets: Vec<i64> = line_iter
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
        
        let mut target_ranges = Vec::new();
        for i in (0..targets.len()).step_by(2) {
            target_ranges.push((targets[i], targets[i+1]));
        }

        let mut mappings: Vec<Vec<MapRange>> = Vec::new();
        let mut mapping = Vec::new();

        line_iter.next();
        line_iter.next();
        loop {

            let line = line_iter.next();
            if line.is_none() {
                break;
            }
            let line = line.unwrap();

            if line.len() > 0 {
                let parts = line
                    .split(' ')
                    .map(|s| i64::from_str_radix(s, 10).unwrap())
                    .into();
                mapping.push(parts);
            } else {
                mappings.push(mapping);
                mapping = Vec::new();
                line_iter.next();
            }
        }
        mappings.push(mapping);

        let mut i = 1;
        loop {
            let result = inv_apply_maps(i, &mappings);
            if target_ranges.iter().map(|r| r.0 <= result && result < r.0 + r.1 ).any(|x| x) {
                println!("{}", i);
                break;
            }
            i += 1;
        }

        
    }

}
