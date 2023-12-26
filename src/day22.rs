use std::cmp::max;
use std::cmp::min;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::remove_dir;
use std::{fs, vec};

#[derive(Debug, Clone)]
struct Brick {
    x1: usize,
    y1: usize,
    z1: usize,
    x2: usize,
    y2: usize,
    z2: usize,
}

pub fn part_a() {
    if let Ok(data) = fs::read_to_string("day22.txt") {
        let mut max_x = 0;
        let mut max_y = 0;
        let mut max_z = 0;
        for line in data.lines() {
            let mut parts = line.split('~');
            let mut first = parts.next().unwrap().split(',');
            max_x = std::cmp::max(
                max_x,
                usize::from_str_radix(first.next().unwrap(), 10).unwrap(),
            );
            max_y = std::cmp::max(
                max_y,
                usize::from_str_radix(first.next().unwrap(), 10).unwrap(),
            );
            max_z = std::cmp::max(
                max_z,
                usize::from_str_radix(first.next().unwrap(), 10).unwrap(),
            );

            let mut second = parts.next().unwrap().split(',');
            max_x = std::cmp::max(
                max_x,
                usize::from_str_radix(second.next().unwrap(), 10).unwrap(),
            );
            max_y = std::cmp::max(
                max_y,
                usize::from_str_radix(second.next().unwrap(), 10).unwrap(),
            );
            max_z = std::cmp::max(
                max_z,
                usize::from_str_radix(second.next().unwrap(), 10).unwrap(),
            );
        }

        max_x += 1;
        max_y += 1;
        max_z += 1;

        let mut volume = vec![vec![vec![0; max_x]; max_y]; max_z];
        let mut bricks = Vec::new();
        for line in data.lines() {
            let mut parts = line.split('~');
            let mut first = parts.next().unwrap().split(',');
            let x1 = usize::from_str_radix(first.next().unwrap(), 10).unwrap();
            let y1 = usize::from_str_radix(first.next().unwrap(), 10).unwrap();
            let z1 = usize::from_str_radix(first.next().unwrap(), 10).unwrap();

            let mut second = parts.next().unwrap().split(',');
            let x2 = usize::from_str_radix(second.next().unwrap(), 10).unwrap();
            let y2 = usize::from_str_radix(second.next().unwrap(), 10).unwrap();
            let z2 = usize::from_str_radix(second.next().unwrap(), 10).unwrap();

            let brick = Brick {
                x1,
                y1,
                z1,
                x2,
                y2,
                z2,
            };
            bricks.push(brick);
            for x in x1..=x2 {
                for y in y1..=y2 {
                    for z in z1..=z2 {
                        volume[z][y][x] = bricks.len();
                    }
                }
            }
        }

        let mut moved = true;
        while moved {
            moved = false;
            for (i, brick) in bricks.iter_mut().enumerate() {
                let mut space_below = usize::MAX;
                for x in brick.x1..=brick.x2 {
                    for y in brick.y1..=brick.y2 {
                        let mut probe = brick.z1;
                        while probe > 0 && volume[probe - 1][y][x] == 0 {
                            probe -= 1;
                        }
                        space_below = min(space_below, brick.z1 - probe);
                    }
                }
                if space_below > 0 {
                    moved = true;
                    for x in brick.x1..=brick.x2 {
                        for y in brick.y1..=brick.y2 {
                            for z in brick.z1..=brick.z2 {
                                volume[z][y][x] = 0;
                            }
                        }
                    }
                    for x in brick.x1..=brick.x2 {
                        for y in brick.y1..=brick.y2 {
                            for z in brick.z1..=brick.z2 {
                                volume[z - space_below][y][x] = i + 1;
                            }
                        }
                    }
                    *brick = Brick {
                        z1: brick.z1 - space_below,
                        z2: brick.z2 - space_below,
                        ..*brick
                    };
                }
            }
        }

        let mut supports = HashMap::new();
        for (i, brick) in bricks.iter().enumerate() {
            let mut connections = HashSet::new();
            if brick.z1 > 0 {
                for x in brick.x1..=brick.x2 {
                    for y in brick.y1..=brick.y2 {
                        if volume[brick.z1 - 1][y][x] > 0 {
                            connections.insert(volume[brick.z1 - 1][y][x]);
                        }
                    }
                }
            }
            supports.insert(i + 1, connections);
        }

        let mut removable = (1..=bricks.len()).collect::<HashSet<usize>>();
        for (_i, supporters) in supports {
            if supporters.len() == 1 {
                removable.remove(supporters.iter().next().unwrap());
            }
        }

        println!("{}", removable.len());
    }
}

pub fn part_b() {
    if let Ok(data) = fs::read_to_string("day22.txt") {
        let mut max_x = 0;
        let mut max_y = 0;
        let mut max_z = 0;
        for line in data.lines() {
            let mut parts = line.split('~');
            let mut first = parts.next().unwrap().split(',');
            max_x = std::cmp::max(
                max_x,
                usize::from_str_radix(first.next().unwrap(), 10).unwrap(),
            );
            max_y = std::cmp::max(
                max_y,
                usize::from_str_radix(first.next().unwrap(), 10).unwrap(),
            );
            max_z = std::cmp::max(
                max_z,
                usize::from_str_radix(first.next().unwrap(), 10).unwrap(),
            );

            let mut second = parts.next().unwrap().split(',');
            max_x = std::cmp::max(
                max_x,
                usize::from_str_radix(second.next().unwrap(), 10).unwrap(),
            );
            max_y = std::cmp::max(
                max_y,
                usize::from_str_radix(second.next().unwrap(), 10).unwrap(),
            );
            max_z = std::cmp::max(
                max_z,
                usize::from_str_radix(second.next().unwrap(), 10).unwrap(),
            );
        }

        max_x += 2;
        max_y += 2;
        max_z += 2;

        let mut volume = vec![vec![vec![0; max_x]; max_y]; max_z];
        let mut bricks = Vec::new();
        for line in data.lines() {
            let mut parts = line.split('~');
            let mut first = parts.next().unwrap().split(',');
            let x1 = usize::from_str_radix(first.next().unwrap(), 10).unwrap();
            let y1 = usize::from_str_radix(first.next().unwrap(), 10).unwrap();
            let z1 = usize::from_str_radix(first.next().unwrap(), 10).unwrap();

            let mut second = parts.next().unwrap().split(',');
            let x2 = usize::from_str_radix(second.next().unwrap(), 10).unwrap();
            let y2 = usize::from_str_radix(second.next().unwrap(), 10).unwrap();
            let z2 = usize::from_str_radix(second.next().unwrap(), 10).unwrap();

            let brick = Brick {
                x1,
                y1,
                z1,
                x2,
                y2,
                z2,
            };
            bricks.push(brick);
            for x in x1..=x2 {
                for y in y1..=y2 {
                    for z in z1..=z2 {
                        volume[z][y][x] = bricks.len();
                    }
                }
            }
        }

        let mut moved = true;
        while moved {
            moved = false;
            for (i, brick) in bricks.iter_mut().enumerate() {
                let mut space_below = usize::MAX;
                for x in brick.x1..=brick.x2 {
                    for y in brick.y1..=brick.y2 {
                        let mut probe = brick.z1;
                        while probe > 0 && volume[probe - 1][y][x] == 0 {
                            probe -= 1;
                        }
                        space_below = min(space_below, brick.z1 - probe);
                    }
                }
                if space_below > 0 {
                    moved = true;
                    for x in brick.x1..=brick.x2 {
                        for y in brick.y1..=brick.y2 {
                            for z in brick.z1..=brick.z2 {
                                volume[z][y][x] = 0;
                            }
                        }
                    }
                    for x in brick.x1..=brick.x2 {
                        for y in brick.y1..=brick.y2 {
                            for z in brick.z1..=brick.z2 {
                                volume[z - space_below][y][x] = i + 1;
                            }
                        }
                    }
                    *brick = Brick {
                        z1: brick.z1 - space_below,
                        z2: brick.z2 - space_below,
                        ..*brick
                    };
                }
            }
        }

        let mut sum = 0;
        for (removed_idx, removed_brick) in bricks.iter().enumerate() {
            let mut changed = HashSet::new();

            let mut moved = true;
            let mut new_bricks = bricks.clone();
            let mut new_volume = volume.clone();
            for x in removed_brick.x1..=removed_brick.x2 {
                for y in removed_brick.y1..=removed_brick.y2 {
                    for z in removed_brick.z1..=removed_brick.z2 {
                        new_volume[z][y][x] = 0;
                    }
                }
            }
            new_bricks[removed_idx] = Brick{x1: max_x-1, x2:max_x-1, y1:max_y-1, y2:max_y-1, z1: 0, z2: 0};
            while moved {
                moved = false;
                for (i, brick) in new_bricks.iter_mut().enumerate() {
                    let mut space_below = usize::MAX;
                    for x in brick.x1..=brick.x2 {
                        for y in brick.y1..=brick.y2 {
                            let mut probe = brick.z1;
                            while probe > 0 && new_volume[probe - 1][y][x] == 0 {
                                probe -= 1;
                            }
                            space_below = min(space_below, brick.z1 - probe);
                        }
                    }
                    if space_below > 0 {
                        changed.insert(i + 1);
                        moved = true;
                        for x in brick.x1..=brick.x2 {
                            for y in brick.y1..=brick.y2 {
                                for z in brick.z1..=brick.z2 {
                                    new_volume[z][y][x] = 0;
                                }
                            }
                        }
                        for x in brick.x1..=brick.x2 {
                            for y in brick.y1..=brick.y2 {
                                for z in brick.z1..=brick.z2 {
                                    new_volume[z - space_below][y][x] = i + 1;
                                }
                            }
                        }
                        *brick = Brick {
                            z1: brick.z1 - space_below,
                            z2: brick.z2 - space_below,
                            ..*brick
                        };
                    }
                }
            }
            println!(" {}: {:?}", removed_idx, changed);
            sum += changed.len();
        }
        println!("{}", sum);
    }
}
