use std::{clone, collections::HashSet, fs};

fn print_location(x: usize, y: usize, grid: &Vec<Vec<char>>) {
    for (i, row) in grid.iter().enumerate() {
        if i == y {
            let mut print_row = row.clone();
            print_row[x] = 'O';
            println!(" {}", String::from_iter(print_row.iter()));
        } else {
            println!(" {}", String::from_iter(row.iter()));
        }
    }
    println!();
}

fn print_taken(count: usize, grid: &Vec<Vec<char>>, taken: HashSet<(usize, usize)>) {
    let mut print_grid = grid.clone();
    for (x, y) in taken {
        print_grid[y][x] = 'O';
    }
    for row in print_grid {
        println!(" {}", String::from_iter(row.iter()));
    }
    println!("{}", count);
}

fn follow_path(
    mut x: usize,
    mut y: usize,
    mut prev_dir: usize,
    mut taken: HashSet<(usize, usize)>,
    mut count: usize,
    grid: &Vec<Vec<char>>,
) -> usize {
    loop {
        if count > 20000 {
            println!("count too large");
            return 0;
        }
        if x == grid[0].len() - 2 && y == grid.len() - 2 {
            // print_taken(count + 1, grid, taken);
            return count + 1;
        }
        let can_up = prev_dir != 2
            && y > 1
            && (grid[y - 1][x] == '.' || grid[y - 1][x] == '^')
            && !taken.contains(&(x, y - 1));
        let can_right = prev_dir != 3
            && x < grid[0].len() - 2
            && (grid[y][x + 1] == '.' || grid[y][x + 1] == '>')
            && !taken.contains(&(x + 1, y));
        let can_down = prev_dir != 0
            && y < grid.len() - 2
            && (grid[y + 1][x] == '.' || grid[y + 1][x] == 'v')
            && !taken.contains(&(x, y + 1));
        let can_left = prev_dir != 1
            && x > 1
            && (grid[y][x - 1] == '.' || grid[y][x - 1] == '<')
            && !taken.contains(&(x - 1, y));

        // println!("  {},{}  u:{} r:{} d:{} l:{}  taken:{:?}", x, y, can_up, can_right, can_down, can_left, taken);

        match (can_up, can_right, can_down, can_left) {
            (true, false, false, false) => {
                y -= 1;
                prev_dir = 0;
                count += 1;
            }
            (false, true, false, false) => {
                x += 1;
                prev_dir = 1;
                count += 1;
            }
            (false, false, true, false) => {
                y += 1;
                prev_dir = 2;
                count += 1;
            }
            (false, false, false, true) => {
                x -= 1;
                prev_dir = 3;
                count += 1;
            }
            (true, true, false, false) => {
                taken.insert((x, y));
                count += 1;
                return std::cmp::max(
                    follow_path(x, y - 1, 0, taken.clone(), count, grid),
                    follow_path(x + 1, y, 1, taken, count, grid),
                );
            }
            (true, false, true, false) => {
                taken.insert((x, y));
                count += 1;
                return std::cmp::max(
                    follow_path(x, y - 1, 0, taken.clone(), count, grid),
                    follow_path(x, y + 1, 2, taken, count, grid),
                );
            }
            (true, false, false, true) => {
                taken.insert((x, y));
                count += 1;
                return std::cmp::max(
                    follow_path(x, y - 1, 0, taken.clone(), count, grid),
                    follow_path(x - 1, y, 3, taken, count, grid),
                );
            }
            (false, true, true, false) => {
                taken.insert((x, y));
                count += 1;
                return std::cmp::max(
                    follow_path(x + 1, y, 1, taken.clone(), count, grid),
                    follow_path(x, y + 1, 2, taken, count, grid),
                );
            }
            (false, true, false, true) => {
                taken.insert((x, y));
                count += 1;
                return std::cmp::max(
                    follow_path(x + 1, y, 1, taken.clone(), count, grid),
                    follow_path(x - 1, y, 3, taken, count, grid),
                );
            }
            (false, false, true, true) => {
                taken.insert((x, y));
                count += 1;
                return std::cmp::max(
                    follow_path(x, y + 1, 2, taken.clone(), count, grid),
                    follow_path(x - 1, y, 3, taken, count, grid),
                );
            }
            (true, true, true, false) => {
                taken.insert((x, y));
                count += 1;
                return std::cmp::max(
                    follow_path(x, y - 1, 0, taken.clone(), count, grid),
                    std::cmp::max(
                        follow_path(x + 1, y, 1, taken.clone(), count, grid),
                        follow_path(x, y + 1, 2, taken, count, grid),
                    ),
                );
            }
            (true, true, false, true) => {
                taken.insert((x, y));
                count += 1;
                return std::cmp::max(
                    follow_path(x, y - 1, 0, taken.clone(), count, grid),
                    std::cmp::max(
                        follow_path(x + 1, y, 1, taken.clone(), count, grid),
                        follow_path(x - 1, y, 3, taken, count, grid),
                    ),
                );
            }
            (true, false, true, true) => {
                taken.insert((x, y));
                count += 1;
                return std::cmp::max(
                    follow_path(x, y - 1, 0, taken.clone(), count, grid),
                    std::cmp::max(
                        follow_path(x, y + 1, 2, taken.clone(), count, grid),
                        follow_path(x - 1, y, 3, taken, count, grid),
                    ),
                );
            }
            (false, true, true, true) => {
                taken.insert((x, y));
                count += 1;
                return std::cmp::max(
                    follow_path(x + 1, y, 1, taken.clone(), count, grid),
                    std::cmp::max(
                        follow_path(x, y + 1, 2, taken.clone(), count, grid),
                        follow_path(x - 1, y, 3, taken, count, grid),
                    ),
                );
            }

            _ => {
                // print_location(x, y, grid);
                return 0;
            }
        }
    }
}

pub fn part_a() {
    if let Ok(data) = fs::read_to_string("day23.txt") {
        let mut grid: Vec<Vec<char>> = Vec::new();
        for line in data.lines() {
            grid.push(line.chars().collect());
        }
        let result = follow_path(1, 1, 2, HashSet::new(), 1, &grid);
        println!("{}", result);
    }
}

pub fn part_b() {
    if let Ok(data) = fs::read_to_string("day23.txt") {
        let mut grid: Vec<Vec<char>> = Vec::new();
        for line in data.lines() {
            grid.push(
                line.replace('^', ".")
                    .replace('>', ".")
                    .replace('v', ".")
                    .replace('<', ".")
                    .chars()
                    .collect(),
            );
        }
        let result = follow_path(1, 1, 2, HashSet::new(), 1, &grid);
        println!("{}", result);
    }
}
