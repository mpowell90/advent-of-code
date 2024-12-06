use std::collections::HashSet;

fn main() {
    let input = include_str!("./input.txt");

    println!("Part 1: {}", Map::parse(input).walk_path().len());
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Coordinate {
    pub x: isize,
    pub y: isize,
}

#[derive(Copy, Clone, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug)]
pub struct Map {
    pub obstructions: Vec<Coordinate>,
    pub guard_position: Coordinate,
    pub row_count: isize,
    pub col_count: isize,
}

impl Map {
    pub fn parse(input: &str) -> Self {
        let lines = input.lines();
        let row_count = lines.clone().count();
        let col_count = lines.clone().next().unwrap().chars().count();

        let mut obstructions: Vec<Coordinate> = Vec::with_capacity(row_count);
        let mut guard_position: Option<Coordinate> = None;

        for (row_idx, row) in lines.enumerate() {
            for (col_idx, ch) in row.chars().enumerate() {
                match ch {
                    '#' => obstructions.push(Coordinate {
                        x: col_idx as isize,
                        y: row_idx as isize,
                    }),
                    '^' => {
                        guard_position = Some(Coordinate {
                            x: col_idx as isize,
                            y: row_idx as isize,
                        })
                    }
                    _ => {},
                }
            }
        }

        Self {
            obstructions,
            guard_position: guard_position.unwrap(),
            row_count: row_count as isize,
            col_count: col_count as isize,
        }
    }

    pub fn is_within_bounds(&self, position: Coordinate) -> bool {
        position.x >= 0
            && position.x < self.col_count
            && position.y >= 0
            && position.y < self.row_count
    }

    pub fn turn_right(&self, direction: Direction) -> Direction {
        match direction {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    pub fn step_forwards(&self, position: Coordinate, direction: Direction) -> Coordinate {
        match direction {
            Direction::North => Coordinate {
                x: position.x,
                y: position.y - 1,
            },
            Direction::East => Coordinate {
                x: position.x + 1,
                y: position.y,
            },
            Direction::South => Coordinate {
                x: position.x,
                y: position.y + 1,
            },
            Direction::West => Coordinate {
                x: position.x - 1,
                y: position.y,
            },
        }
    }

    pub fn walk_path(&self) -> HashSet<Coordinate> {
        let mut path: HashSet<Coordinate> = HashSet::from([self.guard_position]);
        let mut current_position = self.guard_position;
        let mut current_direction = Direction::North;

        loop {
            let next_position = self.step_forwards(current_position, current_direction);

            if !self.is_within_bounds(next_position) {
                break;
            }

            if self.obstructions.contains(&next_position) {
                current_direction = self.turn_right(current_direction);
            } else {
                current_position = next_position;
                path.insert(current_position);
            }
        }

        path
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_parse() {
        let input = "....#.....\n\
                           .........#\n\
                           ..........\n\
                           ..#.......\n\
                           .......#..\n\
                           ..........\n\
                           .#..^.....\n\
                           ........#.\n\
                           #.........\n\
                           ......#...";

        assert_eq!(Map::parse(input).walk_path().len(), 41);
    }
}