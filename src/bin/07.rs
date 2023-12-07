use std::collections::BinaryHeap;

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines();
    let mut hands: BinaryHeap<Hand> = BinaryHeap::new();
    for line in lines {
        hands.push(Hand::new(line, false));
    }
    Some(calculate_winnings(hands))
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines();
    let mut hands: BinaryHeap<Hand> = BinaryHeap::new();
    for line in lines {
        hands.push(Hand::new(line, true));
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
enum HandType {
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
    hand_type: HandType,
    cards: [i8; 5],
    bid: u32,
}

impl Hand {
    fn new(line: &str, joker_rule: bool) -> Hand {
        let (hand, bid) = line.split_once(' ').unwrap();
        let bid: u32 = bid.parse().unwrap();
        let mut cards: [i8; 5] = [0; 5];
        let mut occurences: [u8; 13] = [0; 13];
        for (i, card) in hand.bytes().enumerate() {
            let value = match card {
                b'A' => 12,
                b'K' => 11,
                b'Q' => 10,
                b'J' => 9,
                b'T' => 8,
                card if card.is_ascii_digit() => (card - b'2') as i8,
                _ => panic!(),
            };
            cards[i] = value;
            occurences[value as usize] += 1;
        }
        if joker_rule {
            let jokers = occurences[9];
            occurences[9] = 0;
            let index = occurences
                .iter()
                .enumerate()
                .max_by_key(|&(_, val)| val)
                .unwrap()
                .0;
            occurences[index] += jokers;
            cards.iter_mut().for_each(|card| {
                if *card == 9 {
                    *card = -1;
                }
            });
        }
        let hand_type: HandType = match occurences {
            occ if occ.contains(&5) => HandType::FiveOfKind,
            occ if occ.contains(&4) => HandType::FourOfKind,
            occ if occ.contains(&3) && occ.contains(&2) => HandType::FullHouse,
            occ if occ.contains(&3) => HandType::ThreeOfKind,
            occ if occ.iter().filter(|&x| x.eq(&2)).count() == 2 => HandType::TwoPair,
            occ if occ.contains(&2) => HandType::OnePair,
            _ => HandType::HighCard,
        };
        Hand {
            hand_type,
            cards,
            bid,
        }
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
