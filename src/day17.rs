use core::cmp::min;
use std::{collections::BinaryHeap, fs};

#[derive(Copy, Clone, PartialEq, Eq)]
struct Node {
    x: usize,
    y: usize,
    score: u64,
    dir: Dir,
    count: usize,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .score
            .cmp(&self.score)
            .then_with(|| self.x.cmp(&other.x))
            .then_with(|| self.y.cmp(&other.y))
            .then_with(|| self.dir.cmp(&other.dir))
            .then_with(|| self.count.cmp(&other.count))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Dir {
    UP = 0,
    RIGHT = 1,
    DOWN = 2,
    LEFT = 3,
}

fn try_node(
    x: usize,
    y: usize,
    score: u64,
    count: usize,
    dir: Dir,
    grid: &Vec<Vec<u64>>,
    scores: &mut Vec<Vec<Vec<Vec<u64>>>>,
    heap: &mut BinaryHeap<Node>,
) {
    let next = Node {
        x,
        y,
        score: score + grid[y][x],
        dir: dir,
        count: count,
    };
    if next.score < scores[y][x][dir as usize][count as usize - 1] {
        heap.push(next);
        scores[y][x][dir as usize][count as usize - 1] = next.score;
    }
}

pub fn part_a() {
    if let Ok(data) = fs::read_to_string("day17.txt") {
        let mut grid = Vec::new();
        for row in data.lines() {
            grid.push(
                row.as_bytes()
                    .iter()
                    .map(|val| (*val - b'0') as u64)
                    .collect::<Vec<u64>>(),
            );
        }
        let w = grid[0].len();
        let h = grid.len();

        let mut scores = vec![vec![vec![vec![u64::MAX; 3]; 4]; w]; h];

        let mut heap = BinaryHeap::new();

        scores[1][0][Dir::RIGHT as usize][0] = grid[1][0];
        scores[0][1][Dir::RIGHT as usize][0] = grid[0][1];

        heap.push(Node {
            x: 1,
            y: 0,
            score: grid[0][1],
            dir: Dir::RIGHT,
            count: 1,
        });
        heap.push(Node {
            x: 0,
            y: 1,
            score: grid[1][0],
            dir: Dir::DOWN,
            count: 1,
        });

        while let Some(Node {
            x,
            y,
            score,
            dir,
            count,
        }) = heap.pop()
        {
            if score > scores[y][x][dir as usize][count as usize - 1] {
                continue;
            }

            // println!(" at {},{} with score {:x}", x, y, score);
            match dir {
                Dir::UP => {
                    if x > 0 {
                        try_node(x - 1, y, score, 1, Dir::LEFT, &grid, &mut scores, &mut heap);
                    }
                    if x < w - 1 {
                        try_node(
                            x + 1,
                            y,
                            score,
                            1,
                            Dir::RIGHT,
                            &grid,
                            &mut scores,
                            &mut heap,
                        );
                    }
                    if y > 0 && count < 3 {
                        try_node(
                            x,
                            y - 1,
                            score,
                            count + 1,
                            Dir::UP,
                            &grid,
                            &mut scores,
                            &mut heap,
                        );
                    }
                }
                Dir::RIGHT => {
                    if y > 0 {
                        try_node(x, y - 1, score, 1, Dir::UP, &grid, &mut scores, &mut heap);
                    }
                    if y < h - 1 {
                        try_node(x, y + 1, score, 1, Dir::DOWN, &grid, &mut scores, &mut heap);
                    }
                    if x < w - 1 && count < 3 {
                        try_node(
                            x + 1,
                            y,
                            score,
                            count + 1,
                            Dir::RIGHT,
                            &grid,
                            &mut scores,
                            &mut heap,
                        );
                    }
                }
                Dir::DOWN => {
                    if x > 0 {
                        try_node(x - 1, y, score, 1, Dir::LEFT, &grid, &mut scores, &mut heap);
                    }
                    if x < w - 1 {
                        try_node(
                            x + 1,
                            y,
                            score,
                            1,
                            Dir::RIGHT,
                            &grid,
                            &mut scores,
                            &mut heap,
                        );
                    }
                    if y < h - 1 && count < 3 {
                        try_node(
                            x,
                            y + 1,
                            score,
                            count + 1,
                            Dir::DOWN,
                            &grid,
                            &mut scores,
                            &mut heap,
                        );
                    }
                }
                Dir::LEFT => {
                    if y > 0 {
                        try_node(x, y - 1, score, 1, Dir::UP, &grid, &mut scores, &mut heap);
                    }
                    if y < h - 1 {
                        try_node(x, y + 1, score, 1, Dir::DOWN, &grid, &mut scores, &mut heap);
                    }
                    if x > 0 && count < 3 {
                        try_node(
                            x - 1,
                            y,
                            score,
                            count + 1,
                            Dir::LEFT,
                            &grid,
                            &mut scores,
                            &mut heap,
                        );
                    }
                }
            }
        }

        let min = scores[h - 1][w - 1]
            .iter()
            .map(|dir| dir.iter().min().unwrap())
            .min()
            .unwrap();

        println!("{}", min);
    }
}

pub fn part_b() {
    if let Ok(data) = fs::read_to_string("day17.txt") {
        let mut grid = Vec::new();
        for row in data.lines() {
            grid.push(
                row.as_bytes()
                    .iter()
                    .map(|val| (*val - b'0') as u64)
                    .collect::<Vec<u64>>(),
            );
        }
        let w = grid[0].len();
        let h = grid.len();

        let mut scores = vec![vec![vec![vec![u64::MAX; 10]; 4]; w]; h];

        let mut heap = BinaryHeap::new();

        scores[1][0][Dir::RIGHT as usize][0] = grid[1][0];
        scores[0][1][Dir::RIGHT as usize][0] = grid[0][1];

        heap.push(Node {
            x: 1,
            y: 0,
            score: grid[0][1],
            dir: Dir::RIGHT,
            count: 1,
        });
        heap.push(Node {
            x: 0,
            y: 1,
            score: grid[1][0],
            dir: Dir::DOWN,
            count: 1,
        });

        while let Some(Node {
            x,
            y,
            score,
            dir,
            count,
        }) = heap.pop()
        {
            if score > scores[y][x][dir as usize][count as usize - 1] {
                continue;
            }

            // println!(" at {},{} with score {:x}", x, y, score);
            match dir {
                Dir::UP => {
                    if x > 0 && count > 3{
                        try_node(x - 1, y, score, 1, Dir::LEFT, &grid, &mut scores, &mut heap);
                    }
                    if x < w - 1 && count > 3 {
                        try_node(
                            x + 1,
                            y,
                            score,
                            1,
                            Dir::RIGHT,
                            &grid,
                            &mut scores,
                            &mut heap,
                        );
                    }
                    if y > 0 && count < 10 {
                        try_node(
                            x,
                            y - 1,
                            score,
                            count + 1,
                            Dir::UP,
                            &grid,
                            &mut scores,
                            &mut heap,
                        );
                    }
                }
                Dir::RIGHT => {
                    if y > 0 && count > 3{
                        try_node(x, y - 1, score, 1, Dir::UP, &grid, &mut scores, &mut heap);
                    }
                    if y < h - 1 && count > 3{
                        try_node(x, y + 1, score, 1, Dir::DOWN, &grid, &mut scores, &mut heap);
                    }
                    if x < w - 1 && count < 10 {
                        try_node(
                            x + 1,
                            y,
                            score,
                            count + 1,
                            Dir::RIGHT,
                            &grid,
                            &mut scores,
                            &mut heap,
                        );
                    }
                }
                Dir::DOWN => {
                    if x > 0 && count > 3 {
                        try_node(x - 1, y, score, 1, Dir::LEFT, &grid, &mut scores, &mut heap);
                    }
                    if x < w - 1 && count > 3{
                        try_node(
                            x + 1,
                            y,
                            score,
                            1,
                            Dir::RIGHT,
                            &grid,
                            &mut scores,
                            &mut heap,
                        );
                    }
                    if y < h - 1 && count < 10 {
                        try_node(
                            x,
                            y + 1,
                            score,
                            count + 1,
                            Dir::DOWN,
                            &grid,
                            &mut scores,
                            &mut heap,
                        );
                    }
                }
                Dir::LEFT => {
                    if y > 0 && count > 3{
                        try_node(x, y - 1, score, 1, Dir::UP, &grid, &mut scores, &mut heap);
                    }
                    if y < h - 1 && count > 3{
                        try_node(x, y + 1, score, 1, Dir::DOWN, &grid, &mut scores, &mut heap);
                    }
                    if x > 0 && count < 10 {
                        try_node(
                            x - 1,
                            y,
                            score,
                            count + 1,
                            Dir::LEFT,
                            &grid,
                            &mut scores,
                            &mut heap,
                        );
                    }
                }
            }
        }

        let min = scores[h - 1][w - 1]
            .iter()
            .map(|scores| scores[3..10].iter().min().unwrap())
            .min()
            .unwrap();

        println!("{}", min);
    }
}