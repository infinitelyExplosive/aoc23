use std::{fs, ops::Index, vec, collections::VecDeque};

fn follow_path(grid: &Vec<&[u8]>, start_x: usize, start_y: usize) -> Vec<(usize, usize)> {
    enum Direction {
        Up,
        Down,
        Left,
        Right,
    }
    let h = grid.len();
    let w = grid[0].len();

    let mut starts = Vec::new();
    if start_y > 0 {
        starts.push((start_x, start_y - 1, Direction::Down));
    }
    if start_x > 0 {
        starts.push((start_x - 1, start_y, Direction::Right));
    }
    if start_y < h - 1 {
        starts.push((start_x, start_y + 1, Direction::Up));
    }
    if start_x < w - 1 {
        starts.push((start_x + 1, start_y, Direction::Left));
    }
    for (mut x, mut y, mut prev) in starts {
        let mut path = Vec::new();
        loop {
            if grid[y][x] == b'S' {
                path.push((x, y));
                return path;
            }
            match &prev {
                Direction::Down => {
                    if grid[y][x] == b'F' && x < w - 1 {
                        path.push((x, y));
                        prev = Direction::Left;
                        x += 1;
                    } else if grid[y][x] == b'|' && y > 0 {
                        path.push((x, y));
                        prev = Direction::Down;
                        y -= 1;
                    } else if grid[y][x] == b'7' && x > 0 {
                        path.push((x, y));
                        prev = Direction::Right;
                        x -= 1;
                    } else {
                        break;
                    }
                }
                Direction::Up => {
                    if grid[y][x] == b'L' && x < w - 1 {
                        path.push((x, y));
                        prev = Direction::Left;
                        x += 1;
                    } else if grid[y][x] == b'|' && y < h - 1 {
                        path.push((x, y));
                        prev = Direction::Up;
                        y += 1;
                    } else if grid[y][x] == b'J' && x > 0 {
                        path.push((x, y));
                        prev = Direction::Right;
                        x -= 1;
                    } else {
                        break;
                    }
                }
                Direction::Left => {
                    if grid[y][x] == b'J' && y > 0 {
                        path.push((x, y));
                        prev = Direction::Down;
                        y -= 1;
                    } else if grid[y][x] == b'-' && x < w - 1 {
                        path.push((x, y));
                        prev = Direction::Left;
                        x += 1;
                    } else if grid[y][x] == b'7' && y < h - 1 {
                        path.push((x, y));
                        prev = Direction::Up;
                        y += 1;
                    } else {
                        break;
                    }
                }
                Direction::Right => {
                    if grid[y][x] == b'L' && y > 0 {
                        path.push((x, y));
                        prev = Direction::Down;
                        y -= 1;
                    } else if grid[y][x] == b'-' && x > 0 {
                        path.push((x, y));
                        prev = Direction::Right;
                        x -= 1;
                    } else if grid[y][x] == b'F' && y < h - 1 {
                        path.push((x, y));
                        prev = Direction::Up;
                        y += 1;
                    } else {
                        break;
                    }
                }
            }
        }
    }
    return Vec::new();
}

fn find_path(grid: &Vec<&[u8]>) -> Vec<(usize, usize)> {
    for (y, row) in grid.iter().enumerate() {
        if let Some(x) = row.iter().position(|x| *x == b'S') {
            let path = follow_path(grid, x, y);
            return path;
        }
    }
    return Vec::new();
}

pub fn part_a() {
    if let Ok(data) = fs::read_to_string("day10.txt") {
        let mut grid = Vec::new();
        for line in data.lines() {
            let row = line.as_bytes();
            grid.push(row);
        }

        let path = find_path(&grid);
        println!("{:?}", path);
        println!("{}", path.len() / 2);
    }
}

fn fill(grid: &mut Vec<Vec<u8>>, x: usize, y: usize) {
    let mut queue = VecDeque::new();
    grid[y][x] = 2;
    queue.push_back((x, y));
    while !queue.is_empty() {
        let (x, y) = queue.pop_front().unwrap();

        if x > 0 && grid[y][x - 1] == 0 {
            grid[y][x-1] = 2;
            queue.push_back(( x - 1, y));
        }
        if x < grid[0].len() - 1 && grid[y][x+1] == 0 {
            grid[y][x+1] = 2;
            queue.push_back((x+1, y));
        }
        if y > 0 && grid[y-1][x] == 0 {
            grid[y-1][x] = 2;
            queue.push_back(( x, y-1));
        }
        if y < grid.len() - 1 && grid[y+1][x] == 0 {
            grid[y+1][x] = 2;
            queue.push_back((x, y+1));
        }
    }
}
pub fn part_b() {
    if let Ok(data) = fs::read_to_string("day10.txt") {
        let mut grid = Vec::new();
        for line in data.lines() {
            let row = line.as_bytes();
            grid.push(row);
        }

        let path = find_path(&grid);

        let mut expanded: Vec<Vec<u8>> = vec![vec![0u8; 2 * grid[0].len() + 1]; 2 * grid.len() + 1];
        for (x, y) in &path {
            let x = *x;
            let y = *y;
            expanded[2 * y + 1][2 * x + 1] = 1;
            if grid[y][x] == b'|' {
                expanded[2 * y][2 * x + 1] = 1;
                expanded[2 * y + 2][2 * x + 1] = 1;
            }
            if grid[y][x] == b'-' {
                expanded[2 * y + 1][2 * x] = 1;
                expanded[2 * y + 1][2 * x + 2] = 1;
            }
            if grid[y][x] == b'F' {
                expanded[2 * y + 1][2 * x + 2] = 1;
                expanded[2 * y + 2][2 * x + 1] = 1;
            }
            if grid[y][x] == b'L' {
                expanded[2 * y + 1][2 * x + 2] = 1;
                expanded[2 * y][2 * x + 1] = 1
            }
            if grid[y][x] == b'7' {
                expanded[2 * y + 1][2 * x] = 1;
                expanded[2 * y + 2][2 * x + 1] = 1;
            }
            if grid[y][x] == b'J' {
                expanded[2*y+1][2*x] = 1;
                expanded[2*y][2*x+1] = 1;
            }
        }

        fill(&mut expanded, 0, 0);
        
        let mut sum = 0;
        for row in expanded.iter().skip(1).step_by(2) {
            for val in row.iter().skip(1).step_by(2) {
                if *val == 0 {
                    sum += 1;
                }
            }
        }
        println!("{}", sum);
    }
}
