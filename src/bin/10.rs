advent_of_code::solution!(10);

const WIDTH: usize = 140;
const HEIGHT: usize = 140;

pub fn part_one(input: &str) -> Option<u32> {
    let field = Field::parse(input);
    field.calculate_pipe_length()
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

struct Field {
    tiles: [[char; HEIGHT]; WIDTH],
    start: (usize, usize),
}

impl Field {
    fn parse(input: &str) -> Self {
        let mut tiles = [['.'; HEIGHT]; WIDTH];
        let mut start = (0, 0);
        for (y, line) in input.lines().enumerate() {
            for (x, char) in line.chars().enumerate() {
                tiles[x][y] = char;
                if char.eq(&'S') {
                    start = (x, y);
                }
            }
        }
        Field { tiles, start }
    }

    fn calculate_pipe_length(&self) -> Option<u32> {
        // TODO
        None
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
