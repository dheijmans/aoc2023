use std::collections::VecDeque;

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
    let (seeds, maps) = input.split_once("\n\n").unwrap();
    let seeds: Vec<i64> = seeds
        .strip_prefix("seeds: ")
        .unwrap()
        .split(' ')
        .map(|seed| seed.parse().unwrap())
        .collect();
    let mut seed_ranges = vec![];
    for i in (0..seeds.len()).step_by(2) {
        seed_ranges.push(SeedRange {
            start: seeds[i],
            length: seeds[i + 1],
        });
    }
    let maps: Vec<Map> = maps.split("\n\n").map(Map::parse).collect();
    let locations: Vec<SeedRange> = maps.iter().fold(combine(&mut seed_ranges), |acc, x| {
        x.calculate_next_seed_ranges(acc)
    });
    let mininum = locations.iter().fold(i64::MAX, |acc, x| x.start.min(acc));
    Some(mininum as u32)
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

    fn calculate_next_seed_ranges(&self, seed_ranges: Vec<SeedRange>) -> Vec<SeedRange> {
        let mut next_seed_ranges: Vec<SeedRange> = vec![];
        let mut deque: VecDeque<SeedRange> = VecDeque::from(seed_ranges);
        'outer: while let Some(seed_range) = deque.pop_front() {
            for range in &self.ranges {
                let (next_seed_range, residue): (Option<SeedRange>, Vec<SeedRange>) =
                    range.calculate_next_seed_range(&seed_range);
                if let Some(next_seed_range) = next_seed_range {
                    residue.iter().for_each(|&x| deque.push_back(x));
                    next_seed_ranges.push(next_seed_range);
                    continue 'outer;
                }
            }
            next_seed_ranges.push(seed_range);
        }
        combine(&mut next_seed_ranges)
    }
}

fn combine(seed_ranges: &mut Vec<SeedRange>) -> Vec<SeedRange> {
    let mut combined: Vec<SeedRange> = vec![];
    seed_ranges.sort_by(|a, b| {
        if !a.eq(b) {
            return (a.start + a.length).cmp(&(b.start + b.length));
        }
        b.start.cmp(&a.start)
    });
    while let Some(seed_range) = seed_ranges.pop() {
        let end = seed_range.start + seed_range.length - 1;
        let mut start = seed_range.start;
        while let Some(extending_seed_range) = seed_ranges.pop() {
            if seed_range.contains(extending_seed_range.start + extending_seed_range.length - 1) {
                start = start.min(extending_seed_range.start);
                continue;
            }
            seed_ranges.push(extending_seed_range);
            break;
        }
        combined.push(SeedRange {
            start,
            length: end - start + 1,
        });
    }
    combined
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

    fn calculate_next_seed_range(
        &self,
        seed_range: &SeedRange,
    ) -> (Option<SeedRange>, Vec<SeedRange>) {
        let mut residue: Vec<SeedRange> = vec![];
        if !self.overlaps(seed_range) {
            return (None, vec![]);
        }
        if seed_range.start < self.source {
            residue.push(SeedRange {
                start: seed_range.start,
                length: self.source - seed_range.start,
            });
        }
        if seed_range.start + seed_range.length > self.source + self.length {
            residue.push(SeedRange {
                start: self.source + self.length,
                length: seed_range.start + seed_range.length - self.source - self.length,
            });
        }
        let start = self.source.max(seed_range.start);
        let length =
            (self.source + self.length - start).min(seed_range.start + seed_range.length - start);
        (
            Some(SeedRange {
                start: start + self.destination - self.source,
                length,
            }),
            residue,
        )
    }

    fn overlaps(&self, seed_range: &SeedRange) -> bool {
        seed_range.start < self.source + self.length
            && self.source < seed_range.start + seed_range.length
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
struct SeedRange {
    start: i64,
    length: i64,
}

impl SeedRange {
    fn contains(&self, seed: i64) -> bool {
        seed >= self.start && seed < self.start + self.length
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
