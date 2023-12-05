advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let (seeds, maps) = input.split_once("\n\n").unwrap();
    let seeds: Vec<i64> = seeds
        .strip_prefix("seeds: ")
        .unwrap()
        .split(' ')
        .map(|seed| seed.parse().unwrap())
        .collect();
    let maps: Vec<Map> = maps.split("\n\n").map(Map::parse).collect();
    let locations: Vec<i64> = maps
        .iter()
        .fold(seeds, |acc, x| x.calculate_next_seeds(acc));
    let mininum = locations.iter().fold(i64::MAX, |acc, &x| x.min(acc));
    Some(mininum as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

struct Map {
    ranges: Vec<Range>,
}

impl Map {
    fn calculate_next_seeds(&self, seeds: Vec<i64>) -> Vec<i64> {
        seeds
            .iter()
            .map(|seed| self.calculate_next_seed(*seed))
            .collect()
    }

    fn calculate_next_seed(&self, seed: i64) -> i64 {
        for range in &self.ranges {
            if range.contains(seed) {
                return range.calculate_next_seed(seed);
            }
        }
        seed
    }
}

struct Range {
    source: i64,
    destination: i64,
    length: i64,
}

impl Range {
    fn calculate_next_seed(&self, seed: i64) -> i64 {
        seed + self.destination - self.source
    }

    fn contains(&self, seed: i64) -> bool {
        seed >= self.source && seed < self.source + self.length
    }
}

trait Parse {
    fn parse(input: &str) -> Self;
}

impl Parse for Map {
    fn parse(input: &str) -> Self {
        let (_, ranges) = input.split_once('\n').unwrap();
        let ranges: Vec<Range> = ranges.lines().map(Range::parse).collect();
        Map { ranges }
    }
}

impl Parse for Range {
    fn parse(input: &str) -> Self {
        let values: Vec<i64> = input.split(' ').map(|line| line.parse().unwrap()).collect();
        Range {
            source: values[1],
            destination: values[0],
            length: values[2],
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
