use std::collections::{BTreeSet, VecDeque};

fn main() {
    let input = include_str!("./input.txt");

    let map = Map::parse(input);
    println!("Part 1: {}", walk_path(map.col_count, map.row_count, map.guard_position, &map.obstructions).len());
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Coordinate {
    pub x: isize,
    pub y: isize,
}

impl Coordinate {
    pub fn step_forwards(self, direction: Direction) -> Coordinate {
        match direction {
            Direction::North => Coordinate {
                x: self.x,
                y: self.y - 1,
            },
            Direction::East => Coordinate {
                x: self.x + 1,
                y: self.y,
            },
            Direction::South => Coordinate {
                x: self.x,
                y: self.y + 1,
            },
            Direction::West => Coordinate {
                x: self.x - 1,
                y: self.y,
            },
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn turn_right(self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
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

    pub fn is_within_bounds(col_count: isize, row_count: isize, position: Coordinate) -> bool {
        position.x >= 0
            && position.x < col_count
            && position.y >= 0
            && position.y < row_count
    }
}

pub fn walk_path(col_count: isize, row_count: isize, start_position: Coordinate, obstructions: &[Coordinate]) -> BTreeSet<Coordinate> {
    let mut path: BTreeSet<Coordinate> = BTreeSet::from([start_position]);
    let mut current_position = start_position;
    let mut current_direction = Direction::North;

    loop {
        let next_position = current_position.step_forwards(current_direction);

        if !Map::is_within_bounds(col_count, row_count, next_position) {
            break;
        }

        if obstructions.contains(&next_position) {
            current_direction = current_direction.turn_right();
        } else {
            current_position = next_position;
            path.insert(current_position);
        }
    }

    path
}

pub fn loop_creator(map: &Map) -> usize {
    let path: VecDeque<Coordinate> = walk_path(map.col_count, map.row_count, map.guard_position, &map.obstructions).into_iter().collect();

    let mut loop_count = 0;

    // let mut current_position = self.guard_position;
    // let mut current_direction = Direction::North;

    loop {
        let next_position = self.step_forwards(current_position, current_direction);

        if !self.is_within_bounds(next_position) {
            break;
        }

        if self.obstructions.contains(&next_position) {
            current_direction = self.turn_right(current_direction);
        } else {
            current_position = next_position;
            loop_count += 1;
        }

        if path.contains(&current_position) {
            break;
        }
    }

    loop_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_walk_path() {
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

        let map = Map::parse(input);

        assert_eq!(walk_path(map.col_count, map.row_count, map.guard_position, &map.obstructions).len(), 41);
    }

    // #[test]
    // fn should_create_loops() {
    //     let input = "....#.....\n\
    //                        .........#\n\
    //                        ..........\n\
    //                        ..#.......\n\
    //                        .......#..\n\
    //                        ..........\n\
    //                        .#..^.....\n\
    //                        ........#.\n\
    //                        #.........\n\
    //                        ......#...";

    //     assert_eq!(Map::parse(input).walk_path().len(), 6);
    // }
}