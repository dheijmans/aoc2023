use std::collections::BinaryHeap;

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines();
    let mut hands: BinaryHeap<Hand> = BinaryHeap::new();
    for line in lines {
        hands.push(Hand::new(line));
    }
    let total_winnings = hands.iter().zip(1..).fold(0, |acc, x| acc + x.0.bid * x.1);
    Some(total_winnings)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

enum HandType {
    FiveOfKind,
    FourOfKind,
    FullHouse,
    ThreeOfKind,
    TwoPair,
    OnePair,
    HighCard,
}

struct Hand {
    hand_type: HandType,
    cards: [u8; 5],
    bid: u32,
}

impl Hand {
    fn new(line: &str) -> Hand {
        let (hand, bid) = line.split_once(' ').unwrap();
        let bid: u32 = bid.parse().unwrap();
        let mut cards: [u8; 5] = [0; 5];
        let mut occurences: [u8; 13] = [0; 13];
        for (i, card) in hand.bytes().enumerate() {
            let value = match card {
                b'A' => 12,
                b'K' => 11,
                b'Q' => 10,
                b'J' => 9,
                b'T' => 8,
                card if card.is_ascii_digit() => card - b'2',
                _ => panic!(),
            };
            cards[i] = value;
            occurences[value as usize] += 1;
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
