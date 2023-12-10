advent_of_code::solution!(10);

// These values are dependent on the input
const WIDTH: usize = 140;
const HEIGHT: usize = 140;
const START: char = '|';

pub fn part_one(input: &str) -> Option<u32> {
    let field = Field::parse(input);
    Some(field.calculate_pipe_length()? / 2)
}

pub fn part_two(input: &str) -> Option<u32> {
    let field = Field::parse(input);
    let pipes: Vec<Vec<char>> = field.get_pipes()?;
    calculate_enclosed_tiles(pipes)
}

fn calculate_enclosed_tiles(pipes: Vec<Vec<char>>) -> Option<u32> {
    let mut count = 0;
    for row in pipes {
        let mut inside = false;
        let mut corner = '.';
        for char in row {
            match char {
                '|' => inside = !inside,
                '-' => continue,
                'L' => corner = 'L',
                'J' => match corner {
                    'L' => continue,
                    'F' => inside = !inside,
                    _ => panic!(),
                },
                '7' => match corner {
                    'F' => continue,
                    'L' => inside = !inside,
                    _ => panic!(),
                },
                'F' => corner = 'F',
                '.' => {
                    if inside {
                        count += 1;
                    }
                }
                _ => panic!(),
            }
        }
    }
    Some(count)
}

#[derive(Clone, Copy, Default, PartialEq)]
struct Coords {
    x: usize,
    y: usize,
}

#[derive(Clone, Copy)]
struct Pipe {
    ends: [Coords; 2],
    char: char,
}

impl Pipe {
    fn new(x1: usize, y1: usize, x2: usize, y2: usize, char: char) -> Option<Self> {
        Some(Self {
            ends: [Coords { x: x1, y: y1 }, Coords { x: x2, y: y2 }],
            char,
        })
    }
}
struct Field {
    tiles: Vec<Vec<Option<Pipe>>>,
    start: Coords,
}

impl Field {
    fn parse(input: &str) -> Self {
        let mut tiles: Vec<Vec<Option<Pipe>>> = vec![vec![None; HEIGHT + 1]; WIDTH + 1];
        let mut start = Coords::default();
        for (y, line) in (1usize..).zip(input.lines()) {
            for (x, char) in (1usize..).zip(line.chars()) {
                tiles[x][y] = match char {
                    '|' => Pipe::new(x, y - 1, x, y + 1, char),
                    '-' => Pipe::new(x - 1, y, x + 1, y, char),
                    'L' => Pipe::new(x, y - 1, x + 1, y, char),
                    'J' => Pipe::new(x, y - 1, x - 1, y, char),
                    '7' => Pipe::new(x - 1, y, x, y + 1, char),
                    'F' => Pipe::new(x + 1, y, x, y + 1, char),
                    'S' => {
                        start = Coords { x, y };
                        None
                    }
                    _ => None,
                }
            }
        }
        let mut start_ends: Vec<Coords> = vec![];
        ((start.x - 1)..=(start.x + 1)).for_each(|x| {
            ((start.y - 1)..=(start.y + 1)).for_each(|y| {
                if tiles[x][y].is_some_and(|pipe| pipe.ends.contains(&start)) {
                    start_ends.push(Coords { x, y });
                }
            });
        });
        tiles[start.x][start.y] = Pipe::new(
            start_ends[0].x,
            start_ends[0].y,
            start_ends[1].x,
            start_ends[1].y,
            START,
        );
        Field { tiles, start }
    }

    fn calculate_pipe_length(&self) -> Option<u32> {
        let mut previous = self.start;
        let mut current = self.tiles[previous.x][previous.y]?.ends[0];
        let mut steps = 1;
        while !current.eq(&self.start) {
            let temp = current;
            current = *self.tiles[current.x][current.y]?
                .ends
                .iter()
                .find(|&tile| !tile.eq(&previous))?;
            previous = temp;
            steps += 1;
        }
        Some(steps)
    }

    fn get_pipes(&self) -> Option<Vec<Vec<char>>> {
        let mut pipes: Vec<Vec<char>> = vec![vec!['.'; WIDTH]; HEIGHT];
        let mut previous = self.start;
        let mut current = self.tiles[previous.x][previous.y]?.ends[0];
        pipes[previous.y - 1][previous.x - 1] = START;
        while !current.eq(&self.start) {
            pipes[current.y - 1][current.x - 1] = self.tiles[current.x][current.y]?.char;
            let temp = current;
            current = *self.tiles[current.x][current.y]?
                .ends
                .iter()
                .find(|&tile| !tile.eq(&previous))?;
            previous = temp;
        }
        Some(pipes)
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
