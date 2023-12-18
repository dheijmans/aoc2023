use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
};

use nalgebra::{point, vector, Point2, Vector2};

advent_of_code::solution!(17);

pub fn part_one(input: &str) -> Option<u32> {
    let city: City = City::parse(input)?;
    dijkstra(&city, 0, 3)
}

pub fn part_two(input: &str) -> Option<u32> {
    let city: City = City::parse(input)?;
    dijkstra(&city, 4, 10)
}

fn dijkstra(city: &City, min: u8, max: u8) -> Option<u32> {
    let mut queue: BinaryHeap<Pair> = BinaryHeap::new();
    queue.push(Pair::new(0, State::new(point![0, 0], vector![0, 0], 0)));
    let mut visited: HashSet<State> = HashSet::new();
    while !queue.is_empty() {
        let pair = queue.pop()?;
        let heat_loss = pair.heat_loss;
        let current = pair.state;

        if current
            .pos
            .eq(&point![city.width as i64 - 1, city.height as i64 - 1])
            && current.steps >= min
        {
            return Some(heat_loss);
        }

        if visited.contains(&current) {
            continue;
        }

        visited.insert(current);

        if current.steps < max && !current.dir.eq(&vector![0, 0]) {
            let new_pos = current.pos + current.dir;
            if city.is_bounded(new_pos) {
                queue.push(Pair::new(
                    heat_loss + city.get_heat_loss(new_pos),
                    State::new(new_pos, current.dir, current.steps + 1),
                ));
            }
        }
        if current.steps < min && !current.dir.eq(&vector![0, 0]) {
            continue;
        }
        for dir in [vector![1, 0], vector![0, 1], vector![-1, 0], vector![0, -1]] {
            if dir.eq(&current.dir) || dir.eq(&-current.dir) {
                continue;
            }
            let new_pos = current.pos + dir;
            if !city.is_bounded(new_pos) {
                continue;
            }
            queue.push(Pair::new(
                heat_loss + city.get_heat_loss(new_pos),
                State::new(new_pos, dir, 1),
            ));
        }
    }
    None
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Pair {
    heat_loss: u32,
    state: State,
}

impl Pair {
    fn new(heat_loss: u32, state: State) -> Self {
        Self { heat_loss, state }
    }
}

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> Ordering {
        other.heat_loss.cmp(&self.heat_loss)
    }
}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct State {
    pos: Point2<i64>,
    dir: Vector2<i64>,
    steps: u8,
}

impl State {
    fn new(pos: Point2<i64>, dir: Vector2<i64>, steps: u8) -> Self {
        Self { pos, dir, steps }
    }

    fn distance(&self) -> i64 {
        self.pos.x + self.pos.y
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance().cmp(&other.distance())
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct City {
    blocks: Vec<Vec<u8>>,
    width: usize,
    height: usize,
}

impl City {
    fn new(blocks: Vec<Vec<u8>>, width: usize, height: usize) -> Self {
        Self {
            blocks,
            width,
            height,
        }
    }

    fn parse(input: &str) -> Option<Self> {
        let width = input.lines().next()?.len();
        let height = input.lines().count();
        let blocks = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|char| char.to_digit(10).expect("digit") as u8)
                    .collect()
            })
            .collect();
        Some(Self::new(blocks, width, height))
    }

    fn get_heat_loss(&self, pos: Point2<i64>) -> u32 {
        self.blocks[pos.y as usize][pos.x as usize] as u32
    }

    fn is_bounded(&self, pos: Point2<i64>) -> bool {
        pos.x >= 0 && pos.x < self.width as i64 && pos.y >= 0 && pos.y < self.height as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(102));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }
}
