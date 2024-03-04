use itertools::Itertools;
use nalgebra::{point, vector, Point2, Vector2};

advent_of_code::solution!(21);

const DIRECTIONS: [Vector2<i64>; 4] =
    [vector![0, -1], vector![1, 0], vector![0, 1], vector![-1, 0]];

pub fn part_one(input: &str) -> Option<i64> {
    let garden: Garden = Garden::parse(input)?;
    let steps = count_tiles(&garden, 64, false);
    Some(steps[64])
}

pub fn part_two(input: &str) -> Option<i64> {
    let garden: Garden = Garden::parse(input)?;
    let (x0, x1, x2) = (
        (garden.width / 2) as usize,
        ((garden.width / 2) + garden.width) as usize,
        ((garden.width / 2) + 2 * garden.width) as usize,
    );
    let steps = count_tiles(&garden, x2 as i64, true);
    let (y0, y1, y2) = (steps[x0], steps[x1], steps[x2]);
    let c = y0;
    let b = (4 * y1 - y2 - 3 * c) / 2;
    let a = y1 - b - c;
    let x = (26_501_365 - x0 as i64) / garden.width;
    Some(a * x * x + b * x + c)
}

fn count_tiles(garden: &Garden, steps: i64, repeat: bool) -> Vec<i64> {
    let mut state = vec![garden.start];
    let mut tiles: Vec<i64> = vec![1];
    for _ in 0..steps {
        let mut new_state = vec![];
        while let Some(tile) = state.pop() {
            for dir in DIRECTIONS {
                let new_tile = tile + dir;
                if garden.is_plot(new_tile, repeat) {
                    new_state.push(new_tile);
                }
            }
        }
        state = new_state.into_iter().unique().collect();
        tiles.push(state.len() as i64)
    }
    tiles
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

    fn is_plot(&self, tile: Point2<i64>, repeat: bool) -> bool {
        let mut x = tile.x;
        let mut y = tile.y;
        if repeat {
            x = ((tile.x % self.width) + self.width) % self.width;
            y = ((tile.y % self.height) + self.height) % self.height;
        } else if x < 0 || x >= self.width {
            return false;
        } else if y < 0 || y >= self.height {
            return false;
        }
        !self.grid[y as usize][x as usize]
    }
}
