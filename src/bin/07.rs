use std::collections::BinaryHeap;

advent_of_code::solution!(7);

const JOKER_INDEX: usize = 9;

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines();
    let mut hands: BinaryHeap<Hand> = BinaryHeap::new();
    for line in lines {
        hands.push(Hand::parse(line, false));
    }
    Some(calculate_winnings(hands))
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines();
    let mut hands: BinaryHeap<Hand> = BinaryHeap::new();
    for line in lines {
        hands.push(Hand::parse(line, true));
    }
    Some(calculate_winnings(hands))
}

fn calculate_winnings(hands: BinaryHeap<Hand>) -> u32 {
    hands
        .into_sorted_vec()
        .iter()
        .zip(1..)
        .fold(0, |acc, x| acc + x.0.bid * x.1)
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Rank {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
    hand_type: Rank,
    cards: [i8; 5],
    bid: u32,
}

impl Hand {
    fn new(hand_type: Rank, cards: [i8; 5], bid: u32) -> Self {
        Hand {
            hand_type,
            cards,
            bid,
        }
    }

    fn parse(line: &str, joker_rule: bool) -> Hand {
        let (hand, bid) = line.split_once(' ').unwrap();
        let bid: u32 = bid.parse().unwrap();
        let (mut cards, mut occurences) = parse_hand(hand);
        if joker_rule {
            apply_joker_rule(&mut cards, &mut occurences)
        }
        let hand_type: Rank = determine_rank(&occurences);
        Hand::new(hand_type, cards, bid)
    }
}

fn parse_hand(input: &str) -> ([i8; 5], [u8; 13]) {
    let mut cards = [0; 5];
    let mut occurences = [0; 13];
    for (i, card) in input.bytes().enumerate() {
        let value: i8 = match card {
            b'A' => 12,
            b'K' => 11,
            b'Q' => 10,
            b'J' => JOKER_INDEX as i8,
            b'T' => 8,
            card if card.is_ascii_digit() => (card - b'2') as i8,
            _ => panic!(),
        };
        cards[i] = value;
        occurences[value as usize] += 1;
    }
    (cards, occurences)
}

fn apply_joker_rule(cards: &mut [i8; 5], occurences: &mut [u8; 13]) {
    let jokers = occurences[JOKER_INDEX];
    occurences[JOKER_INDEX] = 0;
    let index = occurences
        .iter()
        .enumerate()
        .max_by_key(|&(_, val)| val)
        .unwrap()
        .0;
    occurences[index] += jokers;
    cards.iter_mut().for_each(|card| {
        if *card == JOKER_INDEX as i8 {
            *card = -1;
        }
    });
}

fn determine_rank(occurences: &[u8; 13]) -> Rank {
    match occurences {
        occ if occ.contains(&5) => Rank::FiveOfKind,
        occ if occ.contains(&4) => Rank::FourOfKind,
        occ if occ.contains(&3) && occ.contains(&2) => Rank::FullHouse,
        occ if occ.contains(&3) => Rank::ThreeOfKind,
        occ if occ.iter().filter(|&x| x.eq(&2)).count() == 2 => Rank::TwoPair,
        occ if occ.contains(&2) => Rank::OnePair,
        _ => Rank::HighCard,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
