use std::fs;

fn is_valid(sequence: &[char], runs: &[i64]) -> bool {
    let mut observed: Vec<i64> = Vec::new();
    let mut count = 0;
    for character in sequence {
        if *character == '#' {
            count += 1;
        } else {
            if count > 0 {
                observed.push(count);
            }
            count = 0;
        }
    }
    if count > 0 {
        observed.push(count);
    }
    // println!("{:?} {:?}", sequence, observed);
    return observed == runs;
}
fn find_valid_permutations(sequence: &Vec<char>, runs: &[i64]) -> i64 {
    // println!(" {:?}", sequence);
    let mut permutations = 0;

    let num_broken: i64 = runs.iter().sum();

    let positions: Vec<usize> = sequence
        .iter()
        .enumerate()
        .filter(|(_, c)| **c == '?')
        .map(|(i, _)| i)
        .collect();
    let mut filled_seq = sequence.clone();
    for i in 0..(2i64.pow(positions.len() as u32)) {
        for j in 0..positions.len() {
            filled_seq[positions[j]] = if (i >> j) & 1 == 1 { '#' } else { '.' };
        }

        if filled_seq.iter().filter(|c| **c == '#').count() as i64 != num_broken {
            continue;
        }
        if is_valid(&filled_seq, runs) {
            permutations += 1;
        }
    }

    // println!();
    return permutations;
}

pub fn part_a() {
    if let Ok(data) = fs::read_to_string("day12.txt") {
        let mut sum = 0;
        for line in data.lines() {
            let mut parts = line.split(" ");
            let sequence = parts.next().unwrap().chars().collect();
            let values: Vec<i64> = parts
                .next()
                .unwrap()
                .split(",")
                .map(|s| i64::from_str_radix(s, 10).unwrap())
                .collect();
            sum += find_valid_permutations(&sequence, &values);
        }
        println!("{}", sum);
    }
}

fn binomial(n: i64, k: i64) -> i64 {
    if k < 0 || k > n {
        return 0;
    }
    if k == 0 || k == n {
        return 1;
    }
    let k = std::cmp::min(k, n - k);
    let mut c = 1;
    for i in 0..k {
        c = c * (n - i) / (i + 1);
    }
    return c;
}

fn recursive_find_permutations(sequence: &Vec<char>, runs: &[i64], indent: String) -> i64 {
    // println!(" {}{:?} {:?}", indent, sequence, runs);
    if runs.len() == 0 {
        if sequence.iter().all(|c| *c == '?' || *c == '.') {
            // println!(" {} returned 1 q", indent);
            return 1;
        } else {
            // println!(" {} returned 0 q", indent);
            return 0;
        }
    }
    if sequence.len() == 0 {
        // println!(" {} returned 0 q+", indent);
        return 0;
    }
    if sequence.iter().filter(|c| **c == '?').count() < 8 {
        let result = find_valid_permutations(sequence, runs);
        // println!(" {} returned {} w", indent, result);
        return result;
    }
    if sequence.iter().all(|c| *c == '?') {
        let overall_size: i64 = sequence.len() as i64;
        let num_runs: i64 = runs.len() as i64;
        let total_run_size: i64 = runs.iter().sum::<i64>() + (num_runs - 1);
        let remaining_choices = overall_size - total_run_size;
        if remaining_choices < 0 {
            // println!(" {}: returning binomial zero overall:{} totalrun:{} remaining:{}", indent, overall_size, total_run_size, remaining_choices);
            return 0;
        } else {
            // stars and bars: solution is n=remaining_choices k=num_runs+1 \binom{n+k-1}{k+1}
            let result = binomial(remaining_choices + num_runs, num_runs);
            // println!(" {}: returning binomial {}", indent, result);
            return result;
        }
    }
    let mut modifier: i64 = -1;
    let mut i = sequence.len() / 2;
    while i < sequence.len() {
        let val = sequence[i];
        if val == '.' {
            // Split into two sides. For each choice on how many groups are on each side:
            //   If both pass sanity check, multiply and add to sum.
            let mut result = 0;
            if sequence[0..i].iter().all(|c| *c == '?' || *c == '.') {
                // println!(" {}:{}", indent, 0);
                result += recursive_find_permutations(
                    &sequence[i + 1..sequence.len()].to_vec(),
                    runs,
                    indent.clone() + "  ",
                );
            }
            for j in 1..runs.len() {
                let lhs_min_size: usize = runs[0..j].iter().sum::<i64>() as usize + (j - 1);
                let rhs_min_size: usize =
                    runs[j..runs.len()].iter().sum::<i64>() as usize + (runs.len() - j - 1);

                if i >= lhs_min_size && (sequence.len() - i) >= rhs_min_size {
                    let lhs = sequence[0..i].to_vec();
                    let rhs = sequence[i + 1..sequence.len()].to_vec();

                    // println!(" {}:{}", indent, j);
                    let prod =
                        recursive_find_permutations(&lhs, &runs[0..j], indent.clone() + "  ")
                            * recursive_find_permutations(
                                &rhs,
                                &runs[j..runs.len()],
                                indent.clone() + "  ",
                            );
                    result += prod
                } else {
                    // println!(
                    //     " {}:{} failed  with lhs_min:{} (was {}) rhs_min:{} (was {})",
                    //     indent,
                    //     j,
                    //     lhs_min_size,
                    //     i,
                    //     rhs_min_size,
                    //     (sequence.len() - i)
                    // );
                    0;
                }
            }
            if sequence[i + 1..sequence.len()]
                .iter()
                .all(|c| *c == '?' || *c == '.')
            {
                // println!(" {}:{}", indent, runs.len());
                result += recursive_find_permutations(
                    &sequence[0..i].to_vec(),
                    runs,
                    indent.clone() + "  ",
                );
            }
            // println!(" {} returned {} (goal {})", indent, result, find_valid_permutations(&sequence, &runs));
            return result;
        } else if val == '#' {
            // For each choice of group for the found '#' to be in, then for every possible position within that group:
            //   Find the section up to the preceding '.' and section after the succeeding '.'.
            //   If the both sides pass sanity check, multiply and add to result.
            let mut result = 0;

            for j in 0..runs.len() {
                let group_size = runs[j] as usize;
                for k in 0..group_size {
                    let before = k;
                    let preceeding_section_valid = before <= i
                        && sequence[i - before..i]
                            .iter()
                            .all(|c| *c == '?' || *c == '#');

                    let after = group_size - k - 1;
                    let succeeding_section_valid = i + after < sequence.len()
                        && sequence[i + 1..i + after + 1]
                            .iter()
                            .all(|c| *c == '?' || *c == '#');

                    let preceeding_dot = i - before - 1;
                    let preceeding_dot_valid = i <= before || sequence[preceeding_dot] != '#';
                    let succeeding_dot = i + after + 1;
                    let succeeding_dot_valid =
                        succeeding_dot >= sequence.len() || sequence[succeeding_dot] != '#';

                    if preceeding_section_valid
                        && succeeding_section_valid
                        && preceeding_dot_valid
                        && succeeding_dot_valid
                    {
                        let lhs = if before < i {
                            sequence[0..i - before - 1].to_vec()
                        } else {
                            Vec::new()
                        };

                        let rhs = if i + after + 2 <= sequence.len() {
                            sequence[i + after + 2..sequence.len()].to_vec()
                        } else {
                            Vec::new()
                        };
                        // println!(" {}:{} ({})", indent, group_size, j);
                        let prod =
                            recursive_find_permutations(&lhs, &runs[0..j], indent.clone() + "  ")
                                * recursive_find_permutations(
                                    &rhs,
                                    &runs[j + 1..runs.len()],
                                    indent.clone() + " c",
                                );
                        result += prod;
                    } else {
                        // println!(
                        //     " {}:failed with {}({}) at offset k:{}",
                        //     indent, group_size, j, k
                        // );
                        0;
                    }
                }
            }

            // println!(
            //     " {} returned {} (goal {})",
            //     indent,
            //     result,
            //     find_valid_permutations(&sequence, &runs)
            // );
            return result;
        }
        if i == 0 {
            i = sequence.len() / 2;
            modifier = 1;
        } else {
            i = (i as i64 + modifier) as usize;
        }
    }
    // println!(" {} returned {} {:?}", indent, "FAIL", sequence);
    return 0;
}

fn efficient_find_valid_permutations(sequence: &Vec<char>, runs: &[i64]) -> i64 {
    return recursive_find_permutations(sequence, runs, "".to_string());
}

pub fn part_b() {
    if let Ok(data) = fs::read_to_string("day12.txt") {
        let mut sum = 0;
        for line in data.lines() {
            let mut parts = line.split(" ");
            let sequence: Vec<char> = parts.next().unwrap().chars().collect();
            let values: Vec<i64> = parts
                .next()
                .unwrap()
                .split(",")
                .map(|s| i64::from_str_radix(s, 10).unwrap())
                .collect();
            let mut extended_sequence = sequence.clone();
            let mut extended_values = values.clone();
            for _ in 0..4 {
                extended_sequence.push('?');
                extended_sequence.append(&mut sequence.clone());
                extended_values.append(&mut values.clone());
            }
            // println!(" {:?}", extended_sequence);
            // println!(" {:?}\n", extended_values);
            let result = efficient_find_valid_permutations(&extended_sequence, &extended_values);
            sum += result;
            // let good = find_valid_permutations(&extended_sequence, &extended_values);

            // if good != result {
            //     println!("{:?} {:?}", extended_sequence.iter().collect::<String>(), extended_values);
            // println!("found {}", result);
            // println!(
            //     "good: {}",
            //     find_valid_permutations(&extended_sequence, &extended_values)
            // );
            // println!();
            // }
        }
        println!("{}", sum);
    }
}
