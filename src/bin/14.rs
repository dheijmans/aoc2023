use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

advent_of_code::solution!(14);

pub fn part_one(input: &str) -> Option<u32> {
    let mut matrix = parse(input)?;
    tilt(&mut matrix, false);
    calculate_load(&matrix)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut matrix = parse(input)?;
    let mut hashes = vec![];
    for i in 0..1000000000 {
        do_cycle(&mut matrix);
        let hash = calculate_hash(&matrix);
        if hashes.contains(&hash) {
            let j = hashes.iter().position(|&h| h == hash)?;
            for _ in 0..(1000000000 - i) % (i - j) - 1 {
                do_cycle(&mut matrix);
            }
            return calculate_load(&matrix);
        }
        hashes.push(hash);
    }
    calculate_load(&matrix)
}

fn do_cycle(matrix: &mut Matrix) {
    let reverse = [false, false, true, true];
    reverse.iter().for_each(|&reverse| {
        tilt(matrix, reverse);
        matrix.transpose();
    });
}

fn parse(input: &str) -> Option<Matrix> {
    let width = input.lines().next()?.len();
    let height = input.lines().count();
    let mut matrix = Matrix::new(width, height);
    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            matrix.data[x][y] = char;
        }
    }
    Some(matrix)
}

fn tilt(matrix: &mut Matrix, reverse: bool) {
    for column in &mut matrix.data {
        tilt_slice(column, reverse);
    }
}

fn tilt_slice(slice: &mut [char], reverse: bool) {
    slice
        .split_mut(|char| char.eq(&'#'))
        .for_each(|section| match reverse {
            true => section.sort(),
            false => section.sort_by(|a, b| b.cmp(a)),
        });
}

fn calculate_load(matrix: &Matrix) -> Option<u32> {
    let mut sum = 0;
    matrix.data.iter().for_each(|column| {
        column.iter().rev().zip(1..).for_each(|(char, load)| {
            if char.eq(&'O') {
                sum += load
            }
        })
    });
    Some(sum)
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

#[derive(Hash)]
struct Matrix {
    width: usize,
    height: usize,
    data: Vec<Vec<char>>,
}

impl Matrix {
    fn new(width: usize, height: usize) -> Self {
        Self {
            data: vec![vec!['.'; height]; width],
            width,
            height,
        }
    }

    fn transpose(&mut self) {
        for y in 0..self.height {
            for x in (y + 1)..self.width {
                let temp = self.data[x][y];
                self.data[x][y] = self.data[y][x];
                self.data[y][x] = temp;
            }
        }
    }
}
