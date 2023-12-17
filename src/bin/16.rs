use nalgebra::{point, vector, Point2, Vector2};
use std::collections::HashSet;

advent_of_code::solution!(16);

pub fn part_one(input: &str) -> Option<u32> {
    let contraption = Contraption::parse(input)?;
    count_energized(&contraption, Beam::new(point![0, 0], vector![1, 0]))
}

pub fn part_two(input: &str) -> Option<u32> {
    let contraption = Contraption::parse(input)?;
    let mut max = 0;
    for i in 0..contraption.width {
        max = count_energized(&contraption, Beam::new(point![i as i8, 0], vector![0, 1]))?.max(max);
        max = count_energized(
            &contraption,
            Beam::new(
                point![i as i8, (contraption.width - 1) as i8],
                vector![0, -1],
            ),
        )?
        .max(max);
    }
    for i in 0..contraption.height {
        max = count_energized(&contraption, Beam::new(point![0, i as i8], vector![1, 0]))?.max(max);
        max = count_energized(
            &contraption,
            Beam::new(
                point![(contraption.height - 1) as i8, i as i8],
                vector![-1, 0],
            ),
        )?
        .max(max);
    }
    Some(max)
}

fn count_energized(contraption: &Contraption, start: Beam) -> Option<u32> {
    let mut stack = vec![start];
    let mut beams: HashSet<Beam> = HashSet::new();
    while !stack.is_empty() {
        let beam = stack.pop()?;

        if !beam.is_bounded(contraption) || beams.contains(&beam) {
            continue;
        }

        let tile = contraption.get_tile(&beam.pos);
        match tile {
            '.' => stack.push(beam.travel()),
            '/' => stack.push(beam.slash()),
            '\\' => stack.push(beam.back_slash()),
            '|' => stack.append(&mut beam.pipe()),
            '-' => stack.append(&mut beam.dash()),
            _ => panic!(),
        }

        beams.insert(beam);
    }

    let mut energized: HashSet<Point2<i8>> = HashSet::new();
    beams.iter().for_each(|beam| {
        energized.insert(beam.pos);
    });
    Some(energized.len() as u32)
}

struct Contraption {
    tiles: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

impl Contraption {
    fn new(tiles: Vec<Vec<char>>, width: usize, height: usize) -> Self {
        Self {
            tiles,
            width,
            height,
        }
    }

    fn parse(input: &str) -> Option<Self> {
        let width = input.lines().next()?.len();
        let height = input.lines().count();
        let tiles = input.lines().map(|line| line.chars().collect()).collect();
        Some(Self::new(tiles, width, height))
    }

    fn get_tile(&self, pos: &Point2<i8>) -> char {
        self.tiles[pos.y as usize][pos.x as usize]
    }
}

#[derive(Eq, Hash, PartialEq)]
struct Beam {
    pos: Point2<i8>,
    dir: Vector2<i8>,
}

impl Beam {
    fn new(pos: Point2<i8>, dir: Vector2<i8>) -> Self {
        Self { pos, dir }
    }

    fn travel(&self) -> Self {
        Self::new(self.pos + self.dir, self.dir)
    }

    fn slash(&self) -> Self {
        let dir: Vector2<i8> = -1 * vector![self.dir.y, self.dir.x];
        Self::new(self.pos, dir).travel()
    }

    fn back_slash(&self) -> Self {
        let dir: Vector2<i8> = vector![self.dir.y, self.dir.x];
        Self::new(self.pos, dir).travel()
    }

    fn pipe(&self) -> Vec<Self> {
        vec![
            Self::new(self.pos, vector![0, 1]).travel(),
            Self::new(self.pos, vector![0, -1]).travel(),
        ]
    }

    fn dash(&self) -> Vec<Self> {
        vec![
            Self::new(self.pos, vector![-1, 0]).travel(),
            Self::new(self.pos, vector![1, 0]).travel(),
        ]
    }

    fn is_bounded(&self, contraption: &Contraption) -> bool {
        self.pos.x >= 0
            && self.pos.y >= 0
            && self.pos.x < contraption.width as i8
            && self.pos.y < contraption.height as i8
    }
}
