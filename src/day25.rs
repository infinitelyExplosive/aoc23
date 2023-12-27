use rand::Rng;
use std::{
    collections::{HashSet, VecDeque},
    fs,
};

fn node_idx(name: &str, nodes: &Vec<String>) -> Option<usize> {
    // println!(" {}", name);
    nodes.iter().position(|x| x == name)
}

fn dfs(start: usize, end: usize, connections: &Vec<Vec<bool>>) -> Vec<usize> {
    let mut seen = HashSet::new();
    let mut queue = VecDeque::new();

    queue.push_back(vec![start]);

    while let Some(mut path) = queue.pop_front() {
        let pos = *path.last().unwrap();
        if pos == end {
            return path;
        }
        for (i, connected) in connections[pos].iter().enumerate() {
            if *connected && !seen.contains(&i) {
                if i == end {
                    path.push(i);
                    return path;
                }
                let mut new_path = path.clone();
                new_path.push(i);
                queue.push_back(new_path);
                seen.insert(i);
            }
        }
    }
    return Vec::new();
}

fn group_size(start: usize, connections: &Vec<Vec<bool>>) -> usize {
    let mut seen = HashSet::new();
    let mut queue = VecDeque::new();

    queue.push_back(start);
    seen.insert(start);
    while let Some(next) = queue.pop_front() {
        for (i, connected) in connections[next].iter().enumerate() {
            if *connected && !seen.contains(&i) {
                queue.push_back(i);
                seen.insert(i);
            }
        }
    }
    return seen.len();
}
pub fn part_a() {
    if let Ok(data) = fs::read_to_string("day25.txt") {
        let mut nodes = Vec::new();
        for line in data.lines() {
            let mut split = line.split(':');
            let name = split.next().unwrap();
            if !nodes.contains(&name.to_string()) {
                nodes.push(name.to_string());
            }
            let mut vals = split.next().unwrap().split(' ');
            while let Some(val) = vals.next() {
                if !nodes.contains(&val.to_string()) {
                    nodes.push(val.to_string());
                }
            }
        }
        let mut connections = vec![vec![false; nodes.len()]; nodes.len()];

        for line in data.lines() {
            let mut split = line.split(": ");
            let i = node_idx(split.next().unwrap(), &nodes).unwrap();
            let mut vals = split.next().unwrap().split(' ');
            while let Some(val) = vals.next() {
                if let Some(j) = node_idx(val, &nodes) {
                    connections[i][j] = true;
                    connections[j][i] = true;
                }
            }
        }

        let mut rng = rand::thread_rng();
        let mut candidates = Vec::new();
        while candidates.len() < 3 {
            let mut counts = vec![vec![0; nodes.len()]; nodes.len()];
            // let mut start: usize = 7;
            // let mut end: usize = 8;
            // for start in 0..(nodes.len() - 1) {
            //     for end in (start + 1)..nodes.len() {
            for sample in 0..3000 {
                let start = rng.gen_range(0..nodes.len());
                let end = rng.gen_range(0..nodes.len());
                // start = (start + 3) % nodes.len();
                // end = (end + 5) % nodes.len();
                // println!(" {} - {}", start, end);
                let path = dfs(start, end, &connections);
                // println!(" {:?}", path);
                if path.len() > 2 {
                    for i in 0..(path.len() - 1) {
                        let a = path[i];
                        let b = path[i + 1];
                        counts[a][b] += 1;
                        counts[b][a] += 1;
                    }
                }
            }
            //     }
            // }
            // for (i, row) in counts.iter().enumerate() {
            //     println!("{}: {:?}", nodes[i], row);
            // }
            // println!();
            let mut max = 0;
            let mut a = 0;
            let mut b = 0;
            for i in 0..(counts.len() - 1) {
                for j in (i + 1)..counts.len() {
                    if counts[i][j] > max {
                        max = counts[i][j];
                        a = i;
                        b = j;
                    }
                }
            }
            candidates.push((a, b));
            connections[a][b] = false;
            connections[b][a] = false;
        }
        // for (a, b) in &candidates {
        //     println!("{}-{}", nodes[*a], nodes[*b]);
        //     println!(" sizes {} {}", group_size(*a, &connections), group_size(*b, &connections));
        // }
        println!("{}", group_size(candidates[0].0, &connections) * group_size(candidates[0].1, &connections));
    }
}
