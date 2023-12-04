use regex::Regex;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines();
    let mut sum = 0;
    for line in lines {
        sum += Card::from(line).calculate_points()
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

struct Card {
    id: u32,
    winning_numbers: Vec<u32>,
    numbers: Vec<u32>,
}

impl Card {
    fn calculate_points(self) -> u32 {
        let mut points = 0;
        for number in self.numbers {
            if self.winning_numbers.contains(&number) {
                if points == 0 {
                    points = 1;
                    continue;
                }
                points *= 2;
            }
        }
        points
    }
}

impl From<&str> for Card {
    fn from(card: &str) -> Self {
        let regex = Regex::new(r"Card |: | \| ").unwrap();
        let mut splits = regex.split(card).filter(|&x| !x.is_empty());
        let id: u32 = splits.next().unwrap().trim().parse::<u32>().unwrap();
        let winning_numbers: Vec<u32> = parse_numbers(splits.next().unwrap());
        let numbers: Vec<u32> = parse_numbers(splits.next().unwrap());
        Card {
            id,
            winning_numbers,
            numbers,
        }
    }
}

fn parse_numbers(list: &str) -> Vec<u32> {
    list.split(' ')
        .filter(|&x| !x.is_empty())
        .map(|x| x.parse::<u32>().unwrap())
        .collect()
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
