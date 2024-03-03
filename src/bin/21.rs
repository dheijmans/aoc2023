use itertools::Itertools;
use nalgebra::{point, vector, Point2, Vector2};

advent_of_code::solution!(21);

const DIRECTIONS: [Vector2<i64>; 4] =
    [vector![0, -1], vector![1, 0], vector![0, 1], vector![-1, 0]];

pub fn part_one(input: &str) -> Option<u32> {
    let garden: Garden = Garden::parse(input)?;
    let steps = count_tiles(garden, 64);
    Some(steps)
}

fn count_tiles(garden: Garden, steps: u32) -> u32 {
    let mut state = vec![garden.start];
    for _ in 0..steps {
        let mut new_state = vec![];
        while let Some(tile) = state.pop() {
            for dir in DIRECTIONS {
                let new_tile = tile + dir;
                if garden.is_plot(new_tile) {
                    new_state.push(new_tile);
                }
            }
        }
        state = new_state.into_iter().unique().collect();
    }
    state.len() as u32
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

struct Garden {
    grid: Vec<Vec<bool>>,
    width: i64,
    height: i64,
    start: Point2<i64>,
}

impl Garden {
    fn new(grid: Vec<Vec<bool>>, width: i64, height: i64, start: Point2<i64>) -> Self {
        Self {
            grid,
            width,
            height,
            start,
        }
    }

    fn parse(input: &str) -> Option<Self> {
        let width = input.lines().next()?.len();
        let height = input.lines().count();
        let mut start: Point2<i64> = point![0, 0];
        let grid: Vec<Vec<bool>> = input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, char)| match char {
                        '.' => false,
                        '#' => true,
                        'S' => {
                            start = point![x as i64, y as i64];
                            false
                        }
                        _ => panic!(),
                    })
                    .collect()
            })
            .collect();
        Some(Self::new(grid, width as i64, height as i64, start))
    }

    fn is_plot(&self, tile: Point2<i64>) -> bool {
        let x = tile.x;
        let y = tile.y;
        if x < 0 || x >= self.width {
            return false;
        }
        if y < 0 || y >= self.height {
            return false;
        }
        !self.grid[y as usize][x as usize]
    }
}
