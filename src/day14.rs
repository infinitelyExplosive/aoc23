use std::{collections::HashMap, fs};

fn eval_load(grid: &Vec<Vec<char>>) -> i64 {
    let mut sum = 0;
    for i in 0..grid.len() {
        sum += (grid[i].iter().filter(|c| **c == 'O').count()) * (grid.len() - i)
    }
    return sum as i64;
}

fn move_north(grid: &mut Vec<Vec<char>>) {
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'O' {
                grid[i][j] = '.';
                let mut probe = i;
                while probe > 0 && grid[probe][j] == '.' {
                    probe -= 1;
                }
                if grid[probe][j] != '.' {
                    probe += 1;
                }
                grid[probe][j] = 'O';
            }
        }
    }
}

fn move_east(grid: &mut Vec<Vec<char>>) {
    for i in 0..grid.len() {
        for j in (0..grid[0].len()).rev() {
            if grid[i][j] == 'O' {
                grid[i][j] = '.';
                let mut probe = j;
                while probe < grid[0].len() - 1 && grid[i][probe] == '.' {
                    probe += 1;
                }
                if grid[i][probe] != '.' {
                    probe -= 1;
                }
                grid[i][probe] = 'O';
            }
        }
    }
}

fn move_south(grid: &mut Vec<Vec<char>>) {
    for i in (0..grid.len()).rev() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'O' {
                grid[i][j] = '.';
                let mut probe = i;
                while probe < grid.len() - 1 && grid[probe][j] == '.' {
                    probe += 1;
                }
                if grid[probe][j] != '.' {
                    probe -= 1;
                }
                grid[probe][j] = 'O';
            }
        }
    }
}

fn move_west(grid: &mut Vec<Vec<char>>) {
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'O' {
                grid[i][j] = '.';
                let mut probe = j;
                while probe > 0 && grid[i][probe] == '.' {
                    probe -= 1;
                }
                if grid[i][probe] != '.' {
                    probe += 1;
                }
                grid[i][probe] = 'O';
            }
        }
    }
}

pub fn part_a() {
    if let Ok(data) = fs::read_to_string("day14.txt") {
        let mut grid = Vec::new();
        for line in data.lines() {
            grid.push(line.chars().collect::<Vec<char>>());
        }

        move_north(&mut grid);
        let result = eval_load(&grid);
        println!("{}", result);
    }
}

pub fn part_b() {
    if let Ok(data) = fs::read_to_string("day14.txt") {
        let mut grid = Vec::new();
        for line in data.lines() {
            grid.push(line.chars().collect::<Vec<char>>());
        }

        let mut results = HashMap::new();
        let mut needed = None;
        for i in 1..25000 {
            move_north(&mut grid);
            move_west(&mut grid);
            move_south(&mut grid);
            move_east(&mut grid);

            // for row in &grid {
            //     println!("{}", String::from_iter(row.iter()));
            // }
            // println!();

            if i > 21149 {
                let str_form = String::from_iter(grid.iter().flatten());
                if results.contains_key(&str_form) {
                    let val: &mut Vec<i64> = results.get_mut(&str_form).unwrap();
                    val.push(i);
                    if val.len() > 6 {
                        let mut diffs = Vec::new();
                        for i in 1..val.len() {
                            let diff = val[i] - val[i - 1];
                            diffs.push(diff);
                        }
                        if diffs.iter().all(|v| *v == diffs[0]) {
                            println!("on {} found loop len {}", i, diffs[0]);
                            let to_go = 1000000000i64 - i;

                            needed = Some(to_go % diffs[0]);
                            break;
                        }
                    }
                } else {
                    results.insert(str_form, vec![i]);
                }
            }
        }
        if let Some(needed) = needed {
            for _ in 0..needed {
                move_north(&mut grid);
                move_west(&mut grid);
                move_south(&mut grid);
                move_east(&mut grid);
            }

            let result = eval_load(&grid);
            println!("{}", result);
        } else {
            println!("no loop found");
        }
    }
}
