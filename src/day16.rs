use std::fs;

fn trace_path(
    mut x: usize,
    mut y: usize,
    dir: i8,
    grid: &Vec<Vec<char>>,
    travelled: &mut Vec<Vec<i8>>,
) {
    match dir {
        1 => {
            while y > 0 && (grid[y][x] == '.' || grid[y][x] == '|') {
                travelled[y][x] |= dir;
                y -= 1;
            }

            if travelled[y][x] & dir == 0 {
                travelled[y][x] |= dir;
                match grid[y][x] {
                    '\\' => {
                        if x > 0 {
                            trace_path(x - 1, y, 8, grid, travelled);
                        }
                    }
                    '/' => {
                        if x < grid.len() - 1 {
                            trace_path(x + 1, y, 2, grid, travelled);
                        }
                    }
                    '-' => {
                        if x < grid.len() - 1 {
                            trace_path(x + 1, y, 2, grid, travelled);
                        }
                        if x > 0 {
                            trace_path(x - 1, y, 8, grid, travelled);
                        }
                    }
                    _ => {}
                }
            }
        }
        2 => {
            while x < grid[0].len() - 1 && (grid[y][x] == '.' || grid[y][x] == '-') {
                travelled[y][x] |= dir;
                x += 1;
            }

            if travelled[y][x] & dir == 0 {
                travelled[y][x] |= dir;
                match grid[y][x] {
                    '\\' => {
                        if y < grid.len() - 1 {
                            trace_path(x, y + 1, 4, grid, travelled);
                        }
                    }
                    '/' => {
                        if y > 0 {
                            trace_path(x, y - 1, 1, grid, travelled);
                        }
                    }
                    '|' => {
                        if y < grid.len() - 1 {
                            trace_path(x, y + 1, 4, grid, travelled);
                        }
                        if y > 0 {
                            trace_path(x, y - 1, 1, grid, travelled);
                        }
                    }
                    _ => {}
                }
            }
        }
        4 => {
            while y < grid.len() - 1 && (grid[y][x] == '.' || grid[y][x] == '|') {
                travelled[y][x] |= dir;
                y += 1;
            }

            if travelled[y][x] & dir == 0 {
                travelled[y][x] |= dir;
                match grid[y][x] {
                    '\\' => {
                        if x < grid.len() - 1 {
                            trace_path(x + 1, y, 2, grid, travelled);
                        }
                    }
                    '/' => {
                        if x > 0 {
                            trace_path(x - 1, y, 8, grid, travelled);
                        }
                    }
                    '-' => {
                        if x < grid.len() - 1 {
                            trace_path(x + 1, y, 2, grid, travelled);
                        }
                        if x > 0 {
                            trace_path(x - 1, y, 8, grid, travelled);
                        }
                    }
                    _ => {}
                }
            }
        }
        8 => {
            while x > 0 && (grid[y][x] == '.' || grid[y][x] == '-') {
                travelled[y][x] |= dir;
                x -= 1;
            }

            if travelled[y][x] & dir == 0 {
                travelled[y][x] |= dir;
                match grid[y][x] {
                    '\\' => {
                        if y > 0 {
                            trace_path(x, y - 1, 1, grid, travelled);
                        }
                    }
                    '/' => {
                        if y < grid.len() - 1 {
                            trace_path(x, y + 1, 4, grid, travelled);
                        }
                    }
                    '|' => {
                        if y < grid.len() - 1 {
                            trace_path(x, y + 1, 4, grid, travelled);
                        }
                        if y > 0 {
                            trace_path(x, y - 1, 1, grid, travelled);
                        }
                    }
                    _ => {}
                }
            }
        }
        _ => {}
    }
    travelled[y][x] |= dir;
}

pub fn part_a() {
    if let Ok(data) = fs::read_to_string("day16.txt") {
        let mut grid: Vec<Vec<char>> = Vec::new();
        for line in data.lines() {
            grid.push(line.chars().collect())
        }
        let mut travelled = vec![vec![0i8; grid[0].len()]; grid.len()];

        trace_path(0, 0, 2, &grid, &mut travelled);

        let mut sum = 0;
        for row in &travelled {
            for val in row {
                if *val != 0 {
                    sum += 1;
                }
            }
        }
        println!("{}", sum);
    }
}

fn try_direction(x: usize, y: usize, dir: i8, grid: &Vec<Vec<char>>) -> i64 {
    let mut travelled = vec![vec![0i8; grid[0].len()]; grid.len()];
    trace_path(x, y, dir, &grid, &mut travelled);
    let mut sum = 0;
    for row in &travelled {
        for val in row {
            if *val != 0 {
                sum += 1;
            }
        }
    }
    return sum;
}
pub fn part_b() {
    if let Ok(data) = fs::read_to_string("day16.txt") {
        let mut grid: Vec<Vec<char>> = Vec::new();
        for line in data.lines() {
            grid.push(line.chars().collect())
        }

        let mut max_sum = 0;
        for i in 0..grid.len() {
            let sum = try_direction(0, i, 2, &grid);
            if sum > max_sum {
                max_sum = sum;
            }
            let sum = try_direction(grid[0].len() - 1, i, 4, &grid);
            if sum > max_sum {
                max_sum = sum;
            }
        }
        for i in 0..grid[0].len() {
            let sum = try_direction(i, 0, 4, &grid);
            if sum > max_sum {
                max_sum = sum;
            }
            let sum = try_direction(i, grid.len() - 1, 1, &grid);
            if sum > max_sum {
                max_sum = sum;
            }
        }

        println!("{}", max_sum);
    }
}
