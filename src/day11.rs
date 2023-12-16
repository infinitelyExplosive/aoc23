use std::{fs, vec};

fn expand(space: &mut Vec<Vec<u8>>) {
    let expand_rows: Vec<usize> = space
        .iter()
        .enumerate()
        .filter(|(_, row)| row.iter().all(|x| *x == b'.'))
        .map(|(i, _)| i)
        .collect();

    // println!(" {:?} rows", expand_rows);
    let expand_cols: Vec<usize> = (0..space[0].len())
        .filter(|i| space.iter().all(|row| row[*i] == b'.'))
        .collect();
    // println!(" {:?} cols", expand_cols);

    for i in expand_rows.iter().rev() {
        space.insert(*i, vec![b'.'; space[0].len()]);
    }
    for i in expand_cols.iter().rev() {
        for row in space.iter_mut() {
            row.insert(*i, b'.');
        }
    }
}

pub fn part_a() {
    if let Ok(data) = fs::read_to_string("day11.txt") {
        let mut space: Vec<Vec<u8>> = Vec::new();
        for line in data.lines() {
            space.push(line.bytes().collect());
        }

        expand(&mut space);

        let mut galaxies = Vec::new();
        for y in 0..space.len() {
            for x in 0..space[0].len() {
                if space[y][x] == b'#' {
                    galaxies.push((x, y));
                }
            }
        }

        // println!("{:?}", galaxies);
        let mut sum = 0;
        for i in 0..galaxies.len() {
            for j in (i + 1)..galaxies.len() {
                let dist =
                    galaxies[i].0.abs_diff(galaxies[j].0) + galaxies[i].1.abs_diff(galaxies[j].1);
                sum += dist;
                // println!(" {} {} {}", i + 1, j + 1, dist);
            }
        }

        println!("{}", sum);
    }
}

pub fn part_b() {
    if let Ok(data) = fs::read_to_string("day11.txt") {
        let mut galaxies = Vec::new();
        let mut max_x = 0;
        let mut max_y = 0;
        for (i, line) in data.lines().enumerate() {
            for (j, val) in line.bytes().enumerate() {
                if val == b'#' {
                    galaxies.push((j, i));
                    if j > max_x {
                        max_x = j;
                    }
                    if i > max_y {
                        max_y = i;
                    }
                }
            }
        }

        let expand_rows: Vec<usize> = (0..max_y).filter(|y| galaxies.iter().map(|(_, gal_y)| gal_y).all(|gal_y| y != gal_y)).collect();
        let expand_cols: Vec<usize> = (0..max_x).filter(|x| galaxies.iter().map(|(gal_x, _)| gal_x).all(|gal_x| x != gal_x)).collect();

        let factor = 999999;

        for row in expand_rows.iter().rev() {
            for galaxy in galaxies.iter_mut() {
                if galaxy.1 > *row {
                    *galaxy = (galaxy.0, galaxy.1 + factor);
                }
            }
        }
        for col in expand_cols.iter().rev() {
            for galaxy in galaxies.iter_mut() {
                if galaxy.0 > *col {
                    *galaxy = (galaxy.0 + factor, galaxy.1);
                }
            }
        }

        let mut sum = 0;
        for i in 0..galaxies.len() {
            for j in (i + 1)..galaxies.len() {
                let dist =
                    galaxies[i].0.abs_diff(galaxies[j].0) + galaxies[i].1.abs_diff(galaxies[j].1);
                sum += dist;
            }
        }

        println!("{}", sum);

    }
}
