#![allow(clippy::comparison_chain)]
use itertools::Itertools;

struct Hand {
    cards: String,
    bet: usize,
}

const ORDER: &str = "AKQJT98765432";
const ORDER_P2: &str = "AKQT98765432J";

pub fn run() {
    let input = include_str!("../../inputs/day07.txt");
    let mut classement_p1: Vec<(Hand, usize)> = vec![];
    let mut classement_p2: Vec<(Hand, usize)> = vec![];

    for line in input.lines() {
        let (cards, bet) = line.split_once(' ').unwrap();

        classement_p1.push((
            Hand {
                cards: cards.to_string(),
                bet: bet.parse::<usize>().unwrap(),
            },
            get_rank(cards).unwrap(),
        ));

        classement_p2.push((
            Hand {
                cards: cards.to_string(),
                bet: bet.parse::<usize>().unwrap(),
            },
            get_rank_p2(cards.to_string()),
        ))
    }

    let sum: usize = classement_p1
        .into_iter()
        .sorted_by(|a, b| cmp_hand(a, b, ORDER.to_string()))
        .enumerate()
        .map(|(i, (hand, _))| (i + 1) * hand.bet)
        .sum();

    let sum_p2: usize = classement_p2
        .into_iter()
        .sorted_by(|a, b| cmp_hand(a, b, ORDER_P2.to_string()))
        .enumerate()
        .map(|(i, (hand, _))| (i + 1) * hand.bet)
        .sum();

    println!("part1 : {}, part2 : {}", sum, sum_p2);
}

fn get_rank(cards: &str) -> Option<usize> {
    let occurence = cards.chars().counts();
    let diff_card = occurence.len();
    let max = occurence.iter().max_by_key(|x| x.1).unwrap().1;
    let min = occurence.iter().min_by_key(|x| x.1).unwrap().1;

    match (max, min, diff_card) {
        (5, 5, 1) => Some(6),
        (4, 1, 2) => Some(5),
        (3, 2, 2) => Some(4),
        (3, 1, 3) => Some(3),
        (2, 1, 3) => Some(2),
        (2, 1, 4) => Some(1),
        (1, 1, 5) => Some(0),
        _ => None,
    }
}

fn get_rank_p2(cards: String) -> usize {
    if !cards.contains('J') {
        return get_rank(cards.as_str()).unwrap();
    } else {
        let mut rank = 0;
        for card in ORDER_P2.chars() {
            let new_hand = cards.replace('J', card.to_string().as_str());
            let new_rank = get_rank(&new_hand).unwrap();
            rank = if new_rank > rank { new_rank } else { rank };
        }

        rank
    }
}

fn cmp_hand(a: &(Hand, usize), b: &(Hand, usize), order: String) -> std::cmp::Ordering {
    if a.1.cmp(&b.1) == std::cmp::Ordering::Equal {
        for (i, card) in a.0.cards.chars().enumerate() {
            if order.find(card) > order.find(b.0.cards.chars().nth(i).unwrap()) {
                return std::cmp::Ordering::Less;
            } else if order.find(card) < order.find(b.0.cards.chars().nth(i).unwrap()) {
                return std::cmp::Ordering::Greater;
            }
        }
        panic!("Wrong input");
    } else {
        a.1.cmp(&b.1)
    }
}
