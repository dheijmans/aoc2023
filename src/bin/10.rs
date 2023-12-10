advent_of_code::solution!(10);

const WIDTH: usize = 141;
const HEIGHT: usize = 141;

pub fn part_one(input: &str) -> Option<u32> {
    let field = Field::parse(input);
    Some(field.calculate_pipe_length()? / 2)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[derive(Clone, Copy, Default, PartialEq)]
struct Coords {
    x: usize,
    y: usize,
}

#[derive(Clone, Copy)]
struct Pipe {
    ends: [Coords; 2],
}

impl Pipe {
    fn new(x1: usize, y1: usize, x2: usize, y2: usize) -> Option<Self> {
        Some(Self {
            ends: [Coords { x: x1, y: y1 }, Coords { x: x2, y: y2 }],
        })
    }
}
struct Field {
    tiles: Vec<Vec<Option<Pipe>>>,
    start: Coords,
}

impl Field {
    fn parse(input: &str) -> Self {
        let mut tiles: Vec<Vec<Option<Pipe>>> = vec![vec![None; HEIGHT]; WIDTH];
        let mut start = Coords::default();
        for (y, line) in (1usize..).zip(input.lines()) {
            for (x, char) in (1usize..).zip(line.chars()) {
                tiles[x][y] = match char {
                    '|' => Pipe::new(x, y - 1, x, y + 1),
                    '-' => Pipe::new(x - 1, y, x + 1, y),
                    'L' => Pipe::new(x, y - 1, x + 1, y),
                    'J' => Pipe::new(x, y - 1, x - 1, y),
                    '7' => Pipe::new(x - 1, y, x, y + 1),
                    'F' => Pipe::new(x + 1, y, x, y + 1),
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
