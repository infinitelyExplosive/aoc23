use std::fs;

fn find_reflections(grid: &Vec<Vec<char>>) -> usize {
    let mut reflect_col = 0;

    while reflect_col < grid[0].len() - 1 {
        let mut good = true;
        'col_lbl: for row in grid {
            for i in 0..=reflect_col {
                if reflect_col + i + 1 < row.len()
                    && row[reflect_col - i] != row[reflect_col + i + 1]
                {
                    // println!(
                    //     "col{} failed on {}{} ({} {}) in {:?}",
                    //     reflect_col,
                    //     row[reflect_col - i],
                    //     row[reflect_col + i + 1],
                    //     reflect_col - i,
                    //     reflect_col + i + 1,
                    //     row
                    // );
                    good = false;
                    break 'col_lbl;
                }
            }
        }
        if good {
            return reflect_col + 1;
        } else {
            reflect_col += 1;
        }
    }

    let mut reflect_row = 0;
    while reflect_row < grid.len() - 1 {
        let mut good = true;
        'row_lbl: for col_i in 0..grid[0].len() {
            for i in 0..=reflect_row {
                if reflect_row + i + 1 < grid.len()
                    && grid[reflect_row - i][col_i] != grid[reflect_row + i + 1][col_i]
                {
                    // println!(
                    //     "row{} failed on {}{} ({} {}) in {:?}",
                    //     reflect_row,
                    //     grid[reflect_row - i][col_i],
                    //     grid[reflect_row + i + 1][col_i],
                    //     reflect_row - i,
                    //     reflect_row + i + 1,
                    //     grid.iter().map(|row| row[col_i]).collect::<Vec<char>>()
                    // );
                    good = false;
                    break 'row_lbl;
                }
            }
        }
        if good {
            return (reflect_row + 1) * 100
        } else {
            reflect_row += 1;
        }
    }
    return 0;
}

pub fn part_a() {
    if let Ok(data) = fs::read_to_string("day13.txt") {
        let mut grids = Vec::new();
        let mut grid = Vec::new();
        for line in data.lines() {
            if line.len() == 0 {
                grids.push(grid);
                grid = Vec::new();
            } else {
                grid.push(line.chars().collect::<Vec<char>>());
            }
        }
        grids.push(grid);

        let mut result = 0;
        for grid in grids {
            let partial = find_reflections(&grid);
            // println!("{}", partial);
            if partial == 0 {
                println!("grid starting {:?} failed", grid[0]);
            }
            result += partial;
        }
        println!("{}", result);
    }
}

fn find_almost_reflections(grid: &Vec<Vec<char>>) -> usize {
    let mut reflect_col = 0;

    while reflect_col < grid[0].len() - 1 {
        let mut good = 0;
        for row in grid {
            for i in 0..=reflect_col {
                if !(reflect_col + i + 1 < row.len()
                    && row[reflect_col - i] != row[reflect_col + i + 1])
                {
                    // println!(
                    //     "col{} failed on {}{} ({} {}) in {:?}",
                    //     reflect_col,
                    //     row[reflect_col - i],
                    //     row[reflect_col + i + 1],
                    //     reflect_col - i,
                    //     reflect_col + i + 1,
                    //     row
                    // );
                    good += 1;
                }
            }
        }
        if good == grid.len() * (reflect_col+1) - 1{
            return reflect_col + 1;
        } else {
            reflect_col += 1;
        }
    }

    let mut reflect_row = 0;
    while reflect_row < grid.len() - 1 {
        let mut good = 0;
        for col_i in 0..grid[0].len() {
            for i in 0..=reflect_row {
                if !(reflect_row + i + 1 < grid.len()
                    && grid[reflect_row - i][col_i] != grid[reflect_row + i + 1][col_i])
                {
                    // println!(
                    //     "row{} failed on {}{} ({} {}) in {:?}",
                    //     reflect_row,
                    //     grid[reflect_row - i][col_i],
                    //     grid[reflect_row + i + 1][col_i],
                    //     reflect_row - i,
                    //     reflect_row + i + 1,
                    //     grid.iter().map(|row| row[col_i]).collect::<Vec<char>>()
                    // );
                    good += 1;
                }
            }
        }
        if good == grid[0].len() * (reflect_row+1) - 1 {
            return (reflect_row + 1) * 100
        } else {
            reflect_row += 1;
        }
    }
    return 0;
}
pub fn part_b() {
    if let Ok(data) = fs::read_to_string("day13.txt") {
        let mut grids = Vec::new();
        let mut grid = Vec::new();
        for line in data.lines() {
            if line.len() == 0 {
                grids.push(grid);
                grid = Vec::new();
            } else {
                grid.push(line.chars().collect::<Vec<char>>());
            }
        }
        grids.push(grid);

        let mut result = 0;
        for grid in grids {
            let partial = find_almost_reflections(&grid);
            // println!("{}", partial);
            if partial == 0 {
                println!("grid starting {:?} failed", grid[0]);
            }
            result += partial;
        }
        println!("{}", result);
    }
}
