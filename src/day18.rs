use std::{cmp, fs};

pub fn part_a() {
    if let Ok(data) = fs::read_to_string("day18.txt") {
        let mut s = 0;
        let mut t = 0;
        let mut w = 0;
        let mut h = 0;

        let mut x = 0;
        let mut y = 0;

        for line in data.lines() {
            let mut parts = line.split(' ');
            let dir = parts.next().unwrap();
            let dist = i64::from_str_radix(parts.next().unwrap(), 10).unwrap();
            match dir {
                "U" => y -= dist,
                "R" => x += dist,
                "D" => y += dist,
                "L" => x -= dist,
                _ => {
                    panic!("invalid dir");
                }
            }
            w = cmp::max(w, x + 1);
            h = cmp::max(h, y + 1);
            s = cmp::min(s, x);
            t = cmp::min(t, y);

            // assert!(x >= 0 && y >= 0);
        }

        let mut grid = vec![vec![0u32; (w - s) as usize]; (h - t) as usize];
        let mut x = -s as usize;
        let mut y = -t as usize;
        for line in data.lines() {
            let mut parts = line.split(' ');
            let dir = parts.next().unwrap();
            let dist = i64::from_str_radix(parts.next().unwrap(), 10).unwrap();
            match dir {
                "U" => {
                    for _ in 0..dist {
                        grid[y][x] |= 1;
                        y -= 1;
                    }
                    grid[y][x] |= 1;
                }
                "R" => {
                    for _ in 0..dist {
                        grid[y][x] |= 2;
                        x += 1;
                    }
                    grid[y][x] |= 2;
                }
                "D" => {
                    for _ in 0..dist {
                        grid[y][x] |= 4;
                        y += 1;
                    }
                    grid[y][x] |= 4;
                }
                "L" => {
                    for _ in 0..dist {
                        grid[y][x] |= 8;
                        x -= 1;
                    }
                    grid[y][x] |= 8;
                }
                _ => {
                    panic!("invalid dir");
                }
            }
        }

        let mut sum = 0;

        for row in grid.iter_mut() {
            let mut wall_dir = 0;
            let mut skip_dir = 0;
            for i in 0..(w - s) as usize {
                if (row[i] >> 0) & 1 == 1 {
                    if wall_dir == 0 && skip_dir != 1 {
                        wall_dir = 1;
                    } else if wall_dir == 4 {
                        wall_dir = 0;
                        skip_dir = 1;
                    } else if skip_dir == 1 {
                        skip_dir = 0;
                    }
                    // sum += 1;
                } else if (row[i] >> 2) & 1 == 1 {
                    if wall_dir == 0 && skip_dir != 4 {
                        wall_dir = 4;
                    } else if wall_dir == 1 {
                        wall_dir = 0;
                        skip_dir = 4;
                    } else if skip_dir == 4 {
                        skip_dir = 0;
                    }
                    // sum += 1;
                } else if row[i] == 0 {
                    if wall_dir != 0 {
                        row[i] = 1;
                    }
                    if skip_dir != 0 {
                        skip_dir = 0;
                    }
                    // sum += 1;
                }
                if row[i] != 0 {
                    sum += 1;
                }
            }
        }

        println!("{}", sum);

        // for row in grid {
        //     println!(
        //         "{}",
        //         String::from_iter(row.iter().map(|x| if *x == 0 {
        //             '.'
        //         } else {
        //             char::from_digit(*x, 16).unwrap()
        //         }))
        //     );
        // }
    }
}

pub fn part_b() {
    if let Ok(data) = fs::read_to_string("day18.txt") {
        let mut x_div = Vec::new();
        let mut y_div = Vec::new();
        let mut x_lines = Vec::new();
        let mut y_lines = Vec::new();

        let mut x = 0;
        let mut y = 0;

        for line in data.lines() {
            let mut parts = line.split('#');
            parts.next();
            let num = parts.next().unwrap();

            let dir = num.bytes().nth(5).unwrap();
            let dist = i64::from_str_radix(&num[0..5], 16).unwrap();
            match dir {
                b'3' => {
                    x_div.push(x);
                    y_lines.push((x, y - dist, x, y));
                    y -= dist;
                }
                b'0' => {
                    y_div.push(y);
                    x_lines.push((x, y, x + dist, y));
                    x += dist;
                }
                b'1' => {
                    x_div.push(x);
                    y_lines.push((x, y, x, y + dist));
                    y += dist;
                }
                b'2' => {
                    y_div.push(y);
                    x_lines.push((x - dist, y, x, y));
                    x -= dist;
                }
                _ => {
                    panic!("invalid dir");
                }
            }
        }

        x_div.sort();
        x_div.dedup();
        x_div.push(i64::MAX);
        y_div.sort();
        y_div.dedup();
        y_div.push(i64::MAX);
        x_lines.sort_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0)));
        y_lines.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));

        // println!("{:?}", x_div);
        // println!("{:?}", y_div);

        let mut sum = 0;
        let mut start_y = i64::MIN;

        for j in 0..y_div.len() {
            let mut inside = false;
            let end_y = y_div[j];
            let mut start_x = i64::MIN;
            for i in 0..x_div.len() {
                let end_x = x_div[i];
                // println!(
                //     " x:{}-{} y:{}-{}  inside:{}",
                //     start_x, end_x, start_y, end_y, inside
                // );
                if y_lines
                    .iter()
                    .any(|(x1, y1, _x2, y2)| *x1 == start_x && *y1 <= start_y && end_y <= *y2)
                {
                    if inside {
                        sum += end_y - start_y;
                        if x_lines.iter().any(|(x1, y1, x2, _y2)| {
                            *y1 == start_y && *x1 <= start_x && end_x <= *x2
                        }) {
                            sum += end_x - start_x - 1;
                        }
                        inside = false;
                        // println!("  unset inside");
                    } else {
                        sum += (end_y - start_y) * (end_x - start_x);
                        inside = true;
                        // println!("  set inside");
                    }
                } else if inside {
                    sum += (end_y - start_y) * (end_x - start_x);
                    // println!("  inside");
                } else if x_lines
                    .iter()
                    .any(|(x1, y1, x2, _y2)| *y1 == start_y && *x1 <= start_x && end_x <= *x2)
                {
                    sum += end_x - start_x;
                    // println!("  top line");
                } else if x_lines
                    .iter()
                    .any(|(_x1, y1, x2, _y2)| *y1 == start_y && *x2 == start_x)
                {
                    sum += 1;
                    // println!("  top square");
                }

                start_x = end_x;
            }

            start_y = end_y;
        }
        println!("{}", sum);
    }
}
