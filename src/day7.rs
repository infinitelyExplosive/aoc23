use std::fs;

fn is_5kind(cards: &Vec<i64>) -> bool {
    return cards.iter().any(|c| *c == 5);
}

fn is_4kind(cards: &Vec<i64>) -> bool {
    return cards.iter().any(|c| *c == 4);
}

fn is_fullhouse(cards: &Vec<i64>) -> bool {
    return cards.iter().any(|c| *c == 3) && cards.iter().any(|c| *c == 2);
}

fn is_3kind(cards: &Vec<i64>) -> bool {
    return cards.iter().any(|c| *c == 3) && cards.iter().all(|c| *c != 2);
}

fn is_2pair(cards: &Vec<i64>) -> bool {
    return cards.iter().filter(|c| **c == 2).count() == 2;
}

fn is_pair(cards: &Vec<i64>) -> bool {
    return cards.iter().filter(|c| **c == 2).count() == 1 && cards.iter().all(|c| *c != 3);
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Type {
    FIVE,
    FOUR,
    FULLHOUSE,
    THREE,
    TWOPAIR,
    PAIR,
    HIGH,
}

fn get_type(cards: &Vec<i64>) -> Type {
    if is_5kind(cards) {
        Type::FIVE
    } else if is_4kind(cards) {
        Type::FOUR
    } else if is_fullhouse(cards) {
        Type::FULLHOUSE
    } else if is_3kind(cards) {
        Type::THREE
    } else if is_2pair(cards) {
        Type::TWOPAIR
    } else if is_pair(cards) {
        Type::PAIR
    } else {
        Type::HIGH
    }
}

#[derive(Debug)]
struct Hand {
    cards: Vec<i64>,
    hand: String,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_type = get_type(&self.cards);
        let other_type = get_type(&other.cards);

        let type_comparison = self_type.cmp(&other_type);
        if type_comparison == std::cmp::Ordering::Equal {
            return other.hand.cmp(&self.hand);
        }
        return type_comparison;
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return Some(self.cmp(&other));
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        return self.hand == other.hand;
    }
}

impl Eq for Hand {}

fn card_index(card: char) -> usize {
    if card.is_numeric() {
        return card.to_digit(10).unwrap() as usize - 2;
    } else if card == 'T' {
        return 8;
    } else if card == 'J' {
        return 9;
    } else if card == 'Q' {
        return 10;
    } else if card == 'K' {
        return 11;
    } else {
        return 12;
    }
}

pub fn part_a() {
    if let Ok(data) = fs::read_to_string("day7.txt") {
        let lines = data.lines();

        let mut plays: Vec<(Hand, i64)> = Vec::new();
        for line in lines {
            let mut parts = line.split(' ');
            let hand = parts.next().unwrap();
            let bid = i64::from_str_radix(parts.next().unwrap(), 10).unwrap();

            let mut counts: Vec<i64> = vec![0; 13];
            for card in hand.chars() {
                counts[card_index(card)] += 1;
            }

            let replaced_hand = hand
                .replace("A", "E")
                .replace("T", "A")
                .replace("J", "B")
                .replace("Q", "C")
                .replace("K", "D");

            plays.push((
                Hand {
                    cards: counts,
                    hand: replaced_hand,
                },
                bid,
            ));
        }

        plays.sort_by(|a, b| a.0.cmp(&b.0));

        let mut sum = 0;
        for (i, val) in plays.iter().rev().enumerate() {
            sum += (i + 1) as i64 * val.1;
        }
        print!("{}", sum);
    }
}

pub fn part_b() {
    if let Ok(data) = fs::read_to_string("day7.txt") {
        let lines = data.lines();

        let mut plays: Vec<(Hand, i64)> = Vec::new();
        for line in lines {
            let mut parts = line.split(' ');
            let hand = parts.next().unwrap();
            let bid = i64::from_str_radix(parts.next().unwrap(), 10).unwrap();

            let mut variants = Vec::new();
            for variant_i in 0..13 {
                if variant_i == 9 {
                    continue;
                }
                let mut counts: Vec<i64> = vec![0; 13];
                for card in hand.chars() {
                    counts[card_index(card)] += 1;
                }

                counts[variant_i] += counts[9];
                counts[9] = 0;

                let replaced_hand = hand
                    .replace("A", "E")
                    .replace("T", "A")
                    .replace("J", "0")
                    .replace("Q", "C")
                    .replace("K", "D");

                variants.push(Hand{cards: counts, hand: replaced_hand});
            }
            // println!("{:?}", variants);
            variants.sort();
            // println!("{:?}\n", variants);

            plays.push((variants.remove(0), bid));
        }

        plays.sort_by(|a, b| a.0.cmp(&b.0));
        // println!("{:?}", plays);

        let mut sum = 0;
        for (i, val) in plays.iter().rev().enumerate() {
            sum += (i + 1) as i64 * val.1;
        }
        print!("{}", sum);
    }
}
