use std::{fs, process::exit};

fn do_step(current: Vec<Vec<bool>>, grid: &Vec<Vec<char>>) -> Vec<Vec<bool>> {
    let mut result = vec![vec![false; grid[0].len()]; grid.len()];
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if current[i][j] {
                if i > 0 && grid[i - 1][j] == '.' {
                    result[i - 1][j] = true;
                }
                if i < grid.len() - 1 && grid[i + 1][j] == '.' {
                    result[i + 1][j] = true;
                }
                if j > 0 && grid[i][j - 1] == '.' {
                    result[i][j - 1] = true;
                }
                if j < grid[0].len() - 1 && grid[i][j + 1] == '.' {
                    result[i][j + 1] = true;
                }
            }
        }
    }
    return result;
}

pub fn part_a() {
    if let Ok(data) = fs::read_to_string("day21.txt") {
        let mut grid: Vec<Vec<char>> = Vec::new();

        let mut start_x = 0;
        let mut start_y = 0;
        for line in data.lines() {
            if let Some(j) = line.chars().position(|c| c == 'S') {
                start_x = j;
                start_y = grid.len();
            }
            let row = line.replace('S', ".").chars().collect();
            grid.push(row);
        }

        let mut reachable = vec![vec![false; grid[0].len()]; grid.len()];
        reachable[start_y][start_x] = true;

        for _step in 0..64 {
            reachable = do_step(reachable, &grid);
        }

        let mut sum = 0;
        let mut printable = grid.clone();
        for (i, row) in reachable.iter().enumerate() {
            for (j, c) in row.iter().enumerate() {
                if *c {
                    assert!(printable[i][j] == '.');
                    printable[i][j] = 'O';
                    sum += 1;
                }
            }
            let row_str = String::from_iter(printable[i].iter());
            println!("{}", row_str);
        }
        println!("{}", sum);
    }
}

enum Dir {
    Top,
    TopRight,
    Right,
    BottomRight,
    Bottom,
    BottomLeft,
    Left,
    TopLeft,
}

fn fill_cell(grid: &Vec<Vec<char>>, direction: Dir, steps: usize, output: bool) -> usize {
    let mut reachable = vec![vec![false; grid[0].len()]; grid.len()];

    let height = grid.len();
    let width = grid[0].len();
    let center_y = (grid.len() - 1) / 2;
    let center_x = (grid[0].len() - 1) / 2;

    match direction {
        Dir::Top => reachable[0][center_x] = true,
        Dir::TopRight => reachable[0][width - 1] = true,
        Dir::Right => reachable[center_y][grid[0].len() - 1] = true,
        Dir::BottomRight => reachable[height - 1][width - 1] = true,
        Dir::Bottom => reachable[grid.len() - 1][center_x] = true,
        Dir::BottomLeft => reachable[height - 1][0] = true,
        Dir::Left => reachable[center_y][0] = true,
        Dir::TopLeft => reachable[0][0] = true,
    }
    for _step in 0..steps {
        reachable = do_step(reachable, &grid);
    }

    let sum = reachable
        .iter()
        .map(|col| col.iter().filter(|x| **x == true).count())
        .sum();

    if output {
        print_grid(&reachable, grid, true);
        println!();
    }
    return sum;
}

fn full_cell_sum(
    grid: &Vec<Vec<char>>,
    steps: usize,
    even: bool,
) -> usize {
        let mut reachable = vec![vec![false; grid[0].len()]; grid.len()];
        let offset = if even ^ (steps % 2 == 0) {0} else {1};
        reachable[offset][0] = true;
        // old, known good
        for _step in 0..350 {
            reachable = do_step(reachable, &grid);
        }
        let sum = print_grid(&reachable, &grid, false);

        return sum;
    // let mut sum = 0;
    // let offset = if even ^ (steps % 2 == 0) { 0 } else { 1 };
    // // let target_parity = (start_x + start_y + steps + offset) % 2;
    // for i in 0..grid.len() {
    //     for j in 0..grid[0].len() {
    //         if (i + j) % 2 == offset {
    //             if grid[i][j] == '.' {
    //                 sum += 1;
    //             }
    //         }
    //     }
    // }
    // return sum;
}
pub fn part_b() {
    let prints = 0;
    if let Ok(data) = fs::read_to_string("day21.txt") {
        let mut grid: Vec<Vec<char>> = Vec::new();

        let mut start_x = 0;
        let mut start_y = 0;
        for line in data.lines() {
            if let Some(j) = line.chars().position(|c| c == 'S') {
                start_x = j;
                start_y = grid.len();
            }
            let row = line.replace('S', ".").chars().collect();
            grid.push(row);
        }

        let height = grid.len();
        let width = grid[0].len();
        // println!("{} x {}", width, height);

        let factors = 5;
        let left_factor = factors;
        let top_factor = factors;
        let right_factor = factors;
        let bottom_factor = factors;
        let mut fullsize = grid.clone();
        for _ in 0..(left_factor + right_factor) {
            for i in 0..grid.len() {
                fullsize[i].append(&mut grid[i].clone());
            }
        }
        for _ in 0..(top_factor + bottom_factor) {
            fullsize.append(&mut fullsize[0..height].to_vec());
        }

        let steps = 26501365;
        // let steps = 501;

        let grids_above = (steps - 1 - (height - 1) / 2) / height;
        let grids_side = (steps - 1 - (width - 1) / 2) / width;
        // println!(
        //     "full grids: {} x {}",
        //     2 * grids_side + 1,
        //     2 * grids_above + 1
        // );

        let leftmost_steps = ((steps - (width - 1) / 2) - 1) % height;
        // println!("left {}", leftmost_steps);
        let leftmost_sum = fill_cell(&grid, Dir::Right, leftmost_steps, false);
        let topmost_sum = fill_cell(&grid, Dir::Bottom, leftmost_steps, false);
        let rightmost_sum = fill_cell(&grid, Dir::Left, leftmost_steps, false);
        let bottommost_sum = fill_cell(&grid, Dir::Top, leftmost_steps, false);

        let second_left_steps = leftmost_steps + width;
        // println!("2nd left {}", second_left_steps);
        let second_left_sum = fill_cell(&grid, Dir::Right, second_left_steps, false);
        let second_top_sum = fill_cell(&grid, Dir::Bottom, second_left_steps, false);
        let second_right_sum = fill_cell(&grid, Dir::Left, second_left_steps, false);
        let second_bottom_sum = fill_cell(&grid, Dir::Top, second_left_steps, false);

        let full_grid_sum_odd = full_cell_sum(&grid,  steps, true);
        let full_grid_sum_even = full_cell_sum(&grid,  steps, false);

        let outer_top_left_steps = if leftmost_steps >= (height + 1) / 2 {
            leftmost_steps - (height + 1) / 2
        } else {
            leftmost_steps + (height - 1) / 2
        };
        // println!("outer top left");
        let outer_top_left_sum = fill_cell(&grid, Dir::BottomRight, outer_top_left_steps, false);
        let outer_top_right_sum = fill_cell(&grid, Dir::BottomLeft, outer_top_left_steps, false);
        let outer_bottom_right_sum = fill_cell(&grid, Dir::TopLeft, outer_top_left_steps, false);
        let outer_bottom_left_sum = fill_cell(&grid, Dir::TopRight, outer_top_left_steps, false);

        let inner_top_left_steps = if leftmost_steps >= (height + 1) / 2 {
            leftmost_steps + (height - 1) / 2
        } else {
            second_left_steps + (height - 1) / 2
        };
        // println!("inner top left");
        let inner_top_left_sum = fill_cell(&grid, Dir::BottomRight, inner_top_left_steps, false);
        let inner_top_right_sum = fill_cell(&grid, Dir::BottomLeft, inner_top_left_steps, false);
        let inner_bottom_right_sum = fill_cell(&grid, Dir::TopLeft, inner_top_left_steps, false);
        let inner_bottom_left_sum = fill_cell(&grid, Dir::TopRight, inner_top_left_steps, false);

        let outer_times = if leftmost_steps >= (height + 1) / 2 {
            grids_side + 1
        } else {
            grids_side
        };
        let inner_times = outer_times - 1;

        let full_odd_times_left = grids_side;
        let full_even_times_left = if leftmost_steps >= (height + 1) / 2 {
            grids_side + 1
        } else {
            grids_side - 1
        };

        let full_odd_times = if full_odd_times_left % 2 == 0 {
            ((2 * full_odd_times_left.pow(2) + 2) / 8) * 4
        } else {
            (2 * full_odd_times_left.pow(2) / 8) * 4
        };

        let full_even_times = if leftmost_steps >= (height + 1) / 2 {
            if full_even_times_left % 2 == 0 {
                (((2 * full_even_times_left.pow(2) + 2) / 8) * 4) - 4
            } else {
                ((2 * full_even_times_left.pow(2) / 8) * 4) - 4
            }
        } else {
            if full_even_times_left % 2 == 0 {
                ((2 * full_even_times_left.pow(2) + 2) / 8) * 4
            } else {
                (2 * full_even_times_left.pow(2) / 8) * 4
            }
        };

        let (full_even_times, full_odd_times) = if full_odd_times_left % 2 == 1 {
            (full_odd_times, full_even_times)
        } else {
            (full_even_times, full_odd_times)
        };

        let corner_sum = leftmost_sum + topmost_sum + rightmost_sum + bottommost_sum;
        let second_sum = second_left_sum + second_top_sum + second_right_sum + second_bottom_sum;
        let outer_sum = outer_top_left_sum
            + outer_top_right_sum
            + outer_bottom_right_sum
            + outer_bottom_left_sum;
        let inner_sum = inner_top_left_sum
            + inner_top_right_sum
            + inner_bottom_right_sum
            + inner_bottom_left_sum;
        let even_prod = full_grid_sum_even * full_even_times;
        let odd_prod = full_grid_sum_odd * full_odd_times;
        let sum = corner_sum
            + second_sum
            + (outer_sum * outer_times)
            + (inner_sum * inner_times)
            + even_prod
            + odd_prod
            + full_grid_sum_even;

        if prints > 0 {
            println!(
                "\nfull odd times left: {} full even times left: {}",
                full_odd_times_left, full_even_times_left
            );
            println!("left:{}   2nd left:{}", leftmost_sum, second_left_sum);
            println!("right:{}   2nd right:{}", rightmost_sum, second_right_sum);
            println!("top:{}   2nd top:{}", topmost_sum, second_top_sum);
            println!("bot:{}   2nd bot:{}", bottommost_sum, second_bottom_sum);
            println!(
                "top left out:{}    top left in:{}",
                outer_top_left_sum, inner_top_left_sum
            );
            println!(
                "top right out:{}    top right in:{}",
                outer_top_right_sum, inner_top_right_sum
            );
            println!(
                "bottom left out:{}    bottom left in:{}",
                outer_bottom_left_sum, inner_bottom_left_sum
            );
            println!(
                "bottom right out:{}    bottom right in:{}",
                outer_bottom_right_sum, inner_bottom_right_sum
            );
            println!(
                "calculated outer:{}*{} inner:{}*{} even:{}*{} odd:{}*{} corner:{} second:{}",
                outer_sum,
                outer_times,
                inner_sum,
                inner_times,
                full_grid_sum_even,
                full_even_times,
                full_grid_sum_odd,
                full_odd_times,
                corner_sum,
                second_sum,
            );
        }

        let sum1 = sum;
        println!("{}", sum);

        // let mut reachable = vec![vec![false; fullsize[0].len()]; fullsize.len()];
        // reachable[start_y + height * top_factor][start_x + width * left_factor] = true;
        // // old, known good
        // for _step in 0..steps {
        //     reachable = do_step(reachable, &fullsize);
        // }
        // let sum = print_grid(&reachable, &fullsize, false);
        // // if prints > 1 {
        // //     print_grid_spaced(&reachable, &fullsize, width, height);
        // //     for i in 0..=left_factor {
        // //         print!("{: ^width$}", left_factor - i, width = width);
        // //     }

        // //     println!("");
        // // }

        // let known_good = sum;
        // println!("{} {}    {}", sum1, known_good, sum1 == known_good);

        // println!("{}\n-------------", known_good);
        // println!();

        // if prints > 0 {
        //     print!("even:");
        //     get_subgrid(&reachable, &grid, height, width, factors, factors, false);
        //     print!("odd:");
        //     get_subgrid(
        //         &reachable,
        //         &grid,
        //         height,
        //         width,
        //         factors - 1,
        //         factors,
        //         false,
        //     );
        //     print!("left:");
        //     let known_l = get_subgrid(
        //         &reachable,
        //         &grid,
        //         height,
        //         width,
        //         factors - grids_side - 1,
        //         factors,
        //         false,
        //     );
        //     print!("2nd left:");
        //     let known_2l = get_subgrid(
        //         &reachable,
        //         &grid,
        //         height,
        //         width,
        //         factors - grids_side,
        //         factors,
        //         false,
        //     );
        //     print!("right:");
        //     let known_r = get_subgrid(
        //         &reachable,
        //         &grid,
        //         height,
        //         width,
        //         factors + grids_side + 1,
        //         factors,
        //         false,
        //     );
        //     print!("2nd right:");
        //     let known_2r = get_subgrid(
        //         &reachable,
        //         &grid,
        //         height,
        //         width,
        //         factors + grids_side,
        //         factors,
        //         false,
        //     );
        //     print!("top:");
        //     let known_t = get_subgrid(
        //         &reachable,
        //         &grid,
        //         height,
        //         width,
        //         factors,
        //         factors - grids_above - 1,
        //         false,
        //     );
        //     print!("2nd top:");
        //     let known_2t = get_subgrid(
        //         &reachable,
        //         &grid,
        //         height,
        //         width,
        //         factors,
        //         factors - grids_above,
        //         false,
        //     );
        //     print!("bot:");
        //     let known_b = get_subgrid(
        //         &reachable,
        //         &grid,
        //         height,
        //         width,
        //         factors,
        //         factors + grids_above + 1,
        //         false,
        //     );
        //     print!("2nd bot:");
        //     let known_2b = get_subgrid(
        //         &reachable,
        //         &grid,
        //         height,
        //         width,
        //         factors,
        //         factors + grids_above,
        //         false,
        //     );

        //     println!();

        //     let index_offset = if leftmost_steps >= (height + 1) / 2 {
        //         0
        //     } else {
        //         1
        //     };
        //     print!("top left out:");
        //     let known_tlo = get_subgrid(
        //         &reachable,
        //         &grid,
        //         height,
        //         width,
        //         factors - grids_side - 1 + index_offset,
        //         factors - 1,
        //         false,
        //     );
        //     print!("top left in:");
        //     let known_tli = get_subgrid(
        //         &reachable,
        //         &grid,
        //         height,
        //         width,
        //         factors - grids_side + index_offset,
        //         factors - 1,
        //         false,
        //     );
        //     print!("top right out:");
        //     let known_tro = get_subgrid(
        //         &reachable,
        //         &grid,
        //         height,
        //         width,
        //         factors + grids_side + 1 - index_offset,
        //         factors - 1,
        //         false,
        //     );
        //     print!("top right in:");
        //     let known_tri = get_subgrid(
        //         &reachable,
        //         &grid,
        //         height,
        //         width,
        //         factors + grids_side - index_offset,
        //         factors - 1,
        //         false,
        //     );
        //     print!("bot left out:");
        //     let known_blo = get_subgrid(
        //         &reachable,
        //         &grid,
        //         height,
        //         width,
        //         factors - grids_side - 1 + index_offset,
        //         factors + 1,
        //         false,
        //     );
        //     print!("bot left in:");
        //     let known_bli = get_subgrid(
        //         &reachable,
        //         &grid,
        //         height,
        //         width,
        //         factors - grids_side + index_offset,
        //         factors + 1,
        //         false,
        //     );
        //     print!("bot right out:");
        //     let known_bro = get_subgrid(
        //         &reachable,
        //         &grid,
        //         height,
        //         width,
        //         factors + grids_side + 1 - index_offset,
        //         factors + 1,
        //         false,
        //     );
        //     print!("bot right in:");
        //     let known_bri = get_subgrid(
        //         &reachable,
        //         &grid,
        //         height,
        //         width,
        //         factors + grids_side - index_offset,
        //         factors + 1,
        //         false,
        //     );

        //     println!();
        //     let known_corner = known_l + known_r + known_t + known_b;
        //     let known_second = known_2l + known_2r + known_2t + known_2b;
        //     let known_outer = known_tlo + known_tro + known_blo + known_bro;
        //     let known_inner = known_tli + known_tri + known_bri + known_bli;
        //     println!(
        //         "outer:{} inner:{} corner:{}, second:{}",
        //         known_outer, known_inner, known_corner, known_second
        //     );
        // }

        // let mut buf = String::new();
        // std::io::stdin().read_line(&mut buf);
        // println!("{}: {}", step + 1, sum);
        // }

        // let left_knowngood = reachable[0..height]
        //     .iter()
        //     .map(|row| row[width * 3..width * 4].to_vec())
        //     .collect();
        // println!("\nleftmost");
        // let sum = print_grid(&left_knowngood, &grid, false);
        // println!("{}\n", sum);
        // let second_left_knowngood = reachable[0..height]
        //     .iter()
        //     .map(|row| row[width * 4..width * 5].to_vec())
        //     .collect();
        // println!("second left");
        // let sum = print_grid(&second_left_knowngood, &grid, true);
        // println!("{}", sum);
    }
}

fn get_subgrid(
    reachable: &Vec<Vec<bool>>,
    grid: &Vec<Vec<char>>,
    height: usize,
    width: usize,
    x: usize,
    y: usize,
    output: bool,
) -> usize {
    let left_knowngood = reachable[y * height..(y + 1) * height]
        .iter()
        .map(|row| row[width * x..width * (x + 1)].to_vec())
        .collect();
    let sum = print_grid(&left_knowngood, grid, output);
    println!("{}", sum);
    return sum;
}

fn print_grid(reachable: &Vec<Vec<bool>>, grid: &Vec<Vec<char>>, output: bool) -> usize {
    let mut sum = 0;
    let mut printable = grid.clone();
    for (i, row) in reachable.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c {
                assert!(printable[i][j] == '.');
                printable[i][j] = 'O';
                sum += 1;
            }
        }
        if output {
            let row_str = String::from_iter(printable[i].iter());
            println!("{}", row_str);
        }
    }
    return sum;
}

fn print_grid_spaced(
    reachable: &Vec<Vec<bool>>,
    grid: &Vec<Vec<char>>,
    width: usize,
    height: usize,
) {
    let mut printable = grid.clone();
    for (i, row) in reachable.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c {
                assert!(printable[i][j] == '.');
                printable[i][j] = 'O';
            }
        }
        for k in (1..(printable[0].len() / width)).rev() {
            printable[i].insert(k * width, ' ');
        }
        let row_str = String::from_iter(printable[i].iter());
        println!("{}", row_str);
        if i % height == height - 1 {
            println!();
        }
    }
}
