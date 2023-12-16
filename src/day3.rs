use std::fs;

const NON_SYMBOL: [char; 11] = ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '.'];
fn is_part(x: usize, y: usize, board: &Vec<Vec<char>>) -> bool {
    if y > 0 {
        if x > 0 {
            if !NON_SYMBOL.contains(&board[y - 1][x - 1]) {
                return true;
            }
        }
        if !NON_SYMBOL.contains(&board[y - 1][x]) {
            return true;
        }
        if x < board.get(0).unwrap().len() - 1 {
            if !NON_SYMBOL.contains(&board[y - 1][x + 1]) {
                return true;
            }
        }
    }
    if x > 0 {
        if !NON_SYMBOL.contains(&board[y][x - 1]) {
            return true;
        }
    }
    if x < board.get(0).unwrap().len() - 1 {
        if !NON_SYMBOL.contains(&board[y][x + 1]) {
            return true;
        }
    }
    if y < board.len() - 1 {
        if x > 0 {
            if !NON_SYMBOL.contains(&board[y + 1][x - 1]) {
                return true;
            }
        }
        if !NON_SYMBOL.contains(&board[y + 1][x]) {
            return true;
        }
        if x < board.get(0).unwrap().len() - 1 {
            if !NON_SYMBOL.contains(&board[y + 1][x + 1]) {
                return true;
            }
        }
    }
    return false;
}

fn get_num_indices(board: &Vec<Vec<char>>) -> Vec<(usize, usize, usize, i64)> {
    let mut result = Vec::new();

    for y in 0..board.len() {
        let mut is_num = false;
        let mut len = 0;
        for x in 0..board.get(0).unwrap().len() {
            if board[y][x].is_numeric() {
                if !is_num {
                    is_num = true;
                    len = 1;
                } else {
                    len += 1;
                }
            } else if is_num {
                let val: String = board[y][x - len..x].iter().collect();
                let val = i64::from_str_radix(&val, 10).unwrap();
                result.push((x - len, y, len, val));
                is_num = false;
            }
        }
        if is_num {
            let x = board.get(0).unwrap().len();
            let val: String = board[y][x - len..x].iter().collect();
            let val = i64::from_str_radix(&val, 10).unwrap();
            result.push((x - len, y, len, val));
        }
    }
    return result;
}
pub fn part_a() {
    let mut sum = 0;
    if let Ok(data) = fs::read_to_string("day3.txt") {
        let mut board: Vec<Vec<char>> = Vec::new();
        for line in data.lines() {
            board.push(line.chars().collect());
        }

        let nums = get_num_indices(&board);
        for (x, y, len, val) in nums {
            let mut valid = false;
            for offset in 0..len {
                valid |= is_part(x + offset, y, &board);
            }
            if valid {
                sum += val;
            }
        }
    }
    println!("{}", sum);
}

fn find_start(mut x: usize, y: usize, board: &Vec<Vec<char>>) -> (usize, usize, i64) {
    let mut end_x = x;
    while x > 0 && board[y][x].is_numeric() {
        x -= 1;
    }
    if !board[y][x].is_numeric() {
        x += 1;
    }

    while end_x < board.get(0).unwrap().len() - 1 && board[y][end_x].is_numeric() {
        end_x += 1;
    }
    if !board[y][end_x].is_numeric() {
        end_x -= 1;
    }
    let val: String = board[y][x..=end_x].iter().collect();
    let val = i64::from_str_radix(&val, 10).unwrap();
    return (x, y, val);
}
macro_rules! update_point {
    ($pt1: expr, $pt2: expr, $found: expr) => {
        if $pt1.is_none() {
            $pt1 = Some($found);
        } else if $pt1.unwrap() != $found {
            if $pt2.is_none() {
                $pt2 = Some($found);
            } else if $pt2.unwrap() != $found {
                return None;
            }
        }
    };
}
fn get_gear_ratio(x: usize, y: usize, board: &Vec<Vec<char>>) -> Option<i64> {
    let mut pt1 = None;
    let mut pt2 = None;

    if y > 0 {
        if x > 0 {
            if board[y - 1][x - 1].is_numeric() {
                let found = find_start(x - 1, y - 1, board);
                update_point!(pt1, pt2, found);
            }
        }
        if board[y - 1][x].is_numeric() {
            let found = find_start(x, y - 1, board);
            update_point!(pt1, pt2, found);
        }
        if x < board.get(0).unwrap().len() - 1 {
            if board[y - 1][x + 1].is_numeric() {
                let found = find_start(x + 1, y - 1, board);
                update_point!(pt1, pt2, found);
            }
        }
    }
    if x > 0 {
        if board[y][x - 1].is_numeric() {
            let found = find_start(x - 1, y, board);
            update_point!(pt1, pt2, found);
        }
    }
    if x < board.get(0).unwrap().len() - 1 {
        if board[y][x + 1].is_numeric() {
            let found = find_start(x + 1, y, board);
            update_point!(pt1, pt2, found);
        }
    }
    if y < board.len() - 1 {
        if x > 0 {
            if board[y + 1][x - 1].is_numeric() {
                let found = find_start(x - 1, y + 1, board);
                update_point!(pt1, pt2, found);
            }
        }
        if board[y + 1][x].is_numeric() {
            let found = find_start(x, y + 1, board);
            update_point!(pt1, pt2, found);
        }
        if x < board.get(0).unwrap().len() - 1 {
            if board[y + 1][x + 1].is_numeric() {
                let found = find_start(x + 1, y + 1, board);
                update_point!(pt1, pt2, found);
            }
        }
    }
    if pt1.is_some() && pt2.is_some() {
        return Some(pt1.unwrap().2 * pt2.unwrap().2);
    }
    return None;
}

pub fn part_b() {
    let mut sum = 0;
    if let Ok(data) = fs::read_to_string("day3.txt") {
        let mut board: Vec<Vec<char>> = Vec::new();
        for line in data.lines() {
            board.push(line.chars().collect());
        }

        for y in 0..board.len() {
            for x in 0..board.get(0).unwrap().len() {
                if board[y][x] == '*' {
                    if let Some(ratio) = get_gear_ratio(x, y, &board) {
                        sum += ratio;
                    }
                }
            }
        }
    }
    println!("{}", sum);
}
