use std::collections::{HashMap, HashSet};

advent_of_code::solution!(3);

#[allow(clippy::needless_range_loop)]
pub fn part_one(input: &str) -> Option<u32> {
    let schematic: &[&[u8]] = &input
        .lines()
        .map(|line| line.as_bytes())
        .collect::<Vec<&[u8]>>();
    let mut sum = 0;
    let width = schematic[0].len();
    let height = schematic.len();

    for i in 0..height {
        let mut num = 0;
        let mut add = false;
        for j in 0..width {
            if schematic[i][j].is_ascii_digit() {
                num = num * 10 + (schematic[i][j] - b'0') as u32;
                for i in i.decr_or_remain()..(i + 2).min(height) {
                    for j in j.decr_or_remain()..(j + 2).min(width) {
                        add |= schematic[i][j].is_symbol()
                    }
                }
            } else {
                if add {
                    sum += num;
                }
                num = 0;
                add = false;
            }
        }
        if add {
            sum += num;
        }
    }
    Some(sum)
}

#[allow(clippy::needless_range_loop)]
pub fn part_two(input: &str) -> Option<u32> {
    let schematic: &[&[u8]] = &input
        .lines()
        .map(|line| line.as_bytes())
        .collect::<Vec<&[u8]>>();
    let width = schematic[0].len();
    let height = schematic.len();
    let mut map: HashMap<(usize, usize), (u32, bool)> = HashMap::new();

    for i in 0..height {
        let mut num = 0;
        let mut gears: HashSet<(usize, usize)> = HashSet::new();
        for j in 0..width {
            if schematic[i][j].is_ascii_digit() {
                num = num * 10 + (schematic[i][j] - b'0') as u32;
                for i in i.decr_or_remain()..(i + 2).min(height) {
                    for j in j.decr_or_remain()..(j + 2).min(width) {
                        if schematic[i][j].is_asterisk() {
                            gears.insert((i, j));
                        }
                    }
                }
            } else {
                for &(i, j) in &gears {
                    if map.get(&(i, j)).is_none() {
                        map.insert((i, j), (num, false));
                    } else {
                        map.insert((i, j), (map.get(&(i, j))?.0 * num, true));
                    }
                }
                num = 0;
                gears = HashSet::new();
            }
        }
        for &(i, j) in &gears {
            if map.get(&(i, j)).is_none() {
                map.insert((i, j), (num, false));
            } else {
                map.insert((i, j), (map.get(&(i, j))?.0 * num, true));
            }
        }
    }
    let sum = map
        .iter()
        .fold(0, |acc, x| if x.1 .1 { acc + x.1 .0 } else { acc });
    Some(sum)
}

trait Character {
    fn is_symbol(&self) -> bool;

    fn is_asterisk(&self) -> bool;
}

impl Character for u8 {
    fn is_symbol(&self) -> bool {
        !(self.is_ascii_digit() || self == &b'.')
    }

    fn is_asterisk(&self) -> bool {
        self == &b'*'
    }
}

trait Unsinged {
    fn decr_or_remain(&self) -> Self;
}

impl Unsinged for usize {
    fn decr_or_remain(&self) -> usize {
        if *self == 0 {
            *self
        } else {
            *self - 1
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
