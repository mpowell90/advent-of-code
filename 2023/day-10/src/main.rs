use std::collections::{BTreeMap, BTreeSet};

static EXAMPLE1: &str = ".....\n.F-7.\n.|.|.\n.L-J.\n.....";
static EXAMPLE2: &str = ".....\n.S-7.\n.|.|.\n.L-J.\n.....";

fn main() {
    let input = include_str!("./input.txt");

    let map = Map::parse(input);

    let part_1 = map.find_steps_to_farthest_point();
    dbg!(part_1);

    let part_2 = map.find_enclosed_tiles_count();
    dbg!(part_2);
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum PipeKind {
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Start,
}

impl TryFrom<char> for PipeKind {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '|' => Ok(Self::NorthSouth),
            '-' => Ok(Self::EastWest),
            'L' => Ok(Self::NorthEast),
            'J' => Ok(Self::NorthWest),
            '7' => Ok(Self::SouthWest),
            'F' => Ok(Self::SouthEast),
            '.' => Ok(Self::Ground),
            'S' => Ok(Self::Start),
            _ => Err(format!("PipeKind not recognised: {:?}", value)),
        }
    }
}

impl PipeKind {
    pub fn is_north_connected(pipe: PipeKind) -> bool {
        pipe == PipeKind::NorthSouth || pipe == PipeKind::SouthWest || pipe == PipeKind::SouthEast
    }

    pub fn is_east_connected(pipe: PipeKind) -> bool {
        pipe == PipeKind::EastWest || pipe == PipeKind::NorthWest || pipe == PipeKind::SouthWest
    }

    pub fn is_south_connected(pipe: PipeKind) -> bool {
        pipe == PipeKind::NorthSouth || pipe == PipeKind::NorthWest || pipe == PipeKind::NorthEast
    }

    pub fn is_west_connected(pipe: PipeKind) -> bool {
        pipe == PipeKind::EastWest || pipe == PipeKind::NorthEast || pipe == PipeKind::SouthEast
    }

    #[allow(clippy::eq_op, clippy::nonminimal_bool)]
    pub fn is_north_south_connected(pipe: PipeKind, direction: Direction) -> bool {
        match direction {
            Direction::North => Self::is_north_connected(pipe),
            Direction::South => Self::is_south_connected(pipe),
            _ => false,
        }
    }

    #[allow(clippy::eq_op, clippy::nonminimal_bool)]
    pub fn is_east_west_connected(pipe: PipeKind, direction: Direction) -> bool {
        match direction {
            Direction::East => Self::is_east_connected(pipe),
            Direction::West => Self::is_west_connected(pipe),
            _ => false,
        }
    }

    #[allow(clippy::eq_op, clippy::nonminimal_bool)]
    pub fn is_north_east_connected(pipe: PipeKind, direction: Direction) -> bool {
        match direction {
            Direction::North => Self::is_north_connected(pipe),
            Direction::East => Self::is_east_connected(pipe),
            _ => false,
        }
    }

    #[allow(clippy::eq_op, clippy::nonminimal_bool)]
    pub fn is_north_west_connected(pipe: PipeKind, direction: Direction) -> bool {
        match direction {
            Direction::North => Self::is_north_connected(pipe),
            Direction::West => Self::is_west_connected(pipe),
            _ => false,
        }
    }

    #[allow(clippy::eq_op, clippy::nonminimal_bool)]
    pub fn is_south_west_connected(pipe: PipeKind, direction: Direction) -> bool {
        match direction {
            Direction::South => Self::is_south_connected(pipe),
            Direction::West => Self::is_west_connected(pipe),
            _ => false,
        }
    }

    #[allow(clippy::eq_op, clippy::nonminimal_bool)]
    pub fn is_south_east_connected(pipe: PipeKind, direction: Direction) -> bool {
        match direction {
            Direction::South => Self::is_south_connected(pipe),
            Direction::East => Self::is_east_connected(pipe),
            _ => false,
        }
    }

    pub fn is_start_connected(pipe: PipeKind, direction: Direction) -> bool {
        match direction {
            Direction::North => Self::is_north_connected(pipe),
            Direction::East => Self::is_east_connected(pipe),
            Direction::South => Self::is_south_connected(pipe),
            Direction::West => Self::is_west_connected(pipe),
        }
    }

    pub fn is_connected(&self, other: PipeKind, direction: Direction) -> bool {
        match *self {
            PipeKind::Start => Self::is_start_connected(other, direction),
            PipeKind::NorthSouth => Self::is_north_south_connected(other, direction),
            PipeKind::EastWest => Self::is_east_west_connected(other, direction),
            PipeKind::NorthEast => Self::is_north_east_connected(other, direction),
            PipeKind::NorthWest => Self::is_north_west_connected(other, direction),
            PipeKind::SouthWest => Self::is_south_west_connected(other, direction),
            PipeKind::SouthEast => Self::is_south_east_connected(other, direction),
            PipeKind::Ground => false,
        }
    }

    pub fn previous_direction(&self, next_direction: Direction) -> Result<Direction, String> {
        match next_direction {
            Direction::North => match *self {
                PipeKind::NorthSouth => return Ok(Direction::South),
                PipeKind::NorthEast => return Ok(Direction::East),
                PipeKind::NorthWest => return Ok(Direction::West),
                _ => (),
            },
            Direction::East => match *self {
                PipeKind::EastWest => return Ok(Direction::West),
                PipeKind::NorthEast => return Ok(Direction::North),
                PipeKind::SouthEast => return Ok(Direction::South),
                _ => (),
            },
            Direction::South => match *self {
                PipeKind::NorthSouth => return Ok(Direction::North),
                PipeKind::SouthEast => return Ok(Direction::East),
                PipeKind::SouthWest => return Ok(Direction::West),
                _ => (),
            },
            Direction::West => match *self {
                PipeKind::NorthWest => return Ok(Direction::North),
                PipeKind::EastWest => return Ok(Direction::East),
                PipeKind::SouthWest => return Ok(Direction::South),
                _ => (),
            },
        }
        Err("No valid previous direction".to_string())
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Coord {
    row: isize,
    column: isize,
}

impl Coord {
    pub fn new(row: isize, column: isize) -> Self {
        Self { row, column }
    }

    pub fn north(&self) -> Self {
        Self {
            row: self.row - 1,
            column: self.column,
        }
    }

    pub fn east(&self) -> Self {
        Self {
            row: self.row,
            column: self.column + 1,
        }
    }

    pub fn south(&self) -> Self {
        Self {
            row: self.row + 1,
            column: self.column,
        }
    }

    pub fn west(&self) -> Self {
        Self {
            row: self.row,
            column: self.column - 1,
        }
    }

    pub fn get_direction_traversed(&self, next: Self) -> Result<Direction, String> {
        let row = next.row - self.row;
        let column = next.column - self.column;

        if row == -1 {
            Ok(Direction::North)
        } else if row == 1 {
            Ok(Direction::South)
        } else if column == -1 {
            Ok(Direction::West)
        } else if column == 1 {
            Ok(Direction::East)
        } else {
            Err(format!("Unknown direction: row = {row}, column = {column}"))
        }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Map {
    pub start_position: Option<Coord>,
    pub lookup: BTreeMap<Coord, PipeKind>,
    pub row_count: usize,
    pub column_count: usize,
}

impl Map {
    pub fn parse(input: &str) -> Self {
        let mut lookup: BTreeMap<Coord, PipeKind> = BTreeMap::new();
        let mut start_position: Option<Coord> = None;
        let row_count = input.lines().count();
        let column_count = input.len() / row_count;

        for (row, line) in input.lines().enumerate() {
            for (column, char) in line.chars().enumerate() {
                let coord = Coord::new(row as isize, column as isize);
                let pipe_kind: PipeKind = char.try_into().unwrap();
                if pipe_kind == PipeKind::Start {
                    start_position = Some(coord);
                }
                lookup.insert(coord, pipe_kind);
            }
        }

        dbg!(row_count);
        dbg!(column_count);

        Self {
            start_position,
            lookup,
            row_count,
            column_count,
        }
    }

    pub fn find_path(&self) -> Vec<Coord> {
        let mut path = Vec::from([self.start_position.unwrap()]);
        let mut is_path_found = false;
        let mut path_idx = 0;

        while !is_path_found {
            let current_position = path[path_idx];
            let current_kind = self.lookup.get(&current_position).unwrap();

            let mut found = 0;

            for next_position in [
                current_position.north(),
                current_position.east(),
                current_position.south(),
                current_position.west(),
            ]
            .iter()
            {
                let next_kind = self.lookup.get(next_position).unwrap();
                let direction = current_position
                    .get_direction_traversed(*next_position)
                    .unwrap();
                if current_kind.is_connected(*next_kind, direction) && !path.contains(next_position)
                {
                    path.push(*next_position);
                    found += 1;
                    break;
                }
            }

            if found == 0 {
                is_path_found = true;
            } else {
                path_idx += 1;
            }
        }

        path
    }

    pub fn find_steps_to_farthest_point(&self) -> usize {
        let path = self.find_path();
        path.len() / 2
    }

    pub fn find_enclosed_tiles_count(&self) -> usize {
        let path = self.find_path();
        let coords_in_path = BTreeSet::from_iter(path.clone());
        let all_coords: BTreeSet<Coord> = self.lookup.keys().cloned().collect();
        let enclosed_coords = all_coords
            .difference(&coords_in_path)
            .filter(|coord| {
                path.iter().any(|coord_in_path| {
                    coord_in_path.row == coord.row || coord_in_path.column == coord.column
                })
            })
            .collect::<Vec<&Coord>>();
        enclosed_coords.len()
    }
}

mod tests {
    use crate::*;

    #[test]
    fn should_parse_input() {
        assert_eq!(
            Map::parse(EXAMPLE1),
            Map {
                start_position: None,
                lookup: BTreeMap::from([
                    (Coord::new(0, 0), PipeKind::Ground),
                    (Coord::new(0, 1), PipeKind::Ground),
                    (Coord::new(0, 2), PipeKind::Ground),
                    (Coord::new(0, 3), PipeKind::Ground),
                    (Coord::new(0, 4), PipeKind::Ground),
                    (Coord::new(1, 0), PipeKind::Ground),
                    (Coord::new(1, 1), PipeKind::SouthEast),
                    (Coord::new(1, 2), PipeKind::EastWest),
                    (Coord::new(1, 3), PipeKind::SouthWest),
                    (Coord::new(1, 4), PipeKind::Ground),
                    (Coord::new(2, 0), PipeKind::Ground),
                    (Coord::new(2, 1), PipeKind::NorthSouth),
                    (Coord::new(2, 2), PipeKind::Ground),
                    (Coord::new(2, 3), PipeKind::NorthSouth),
                    (Coord::new(2, 4), PipeKind::Ground),
                    (Coord::new(3, 0), PipeKind::Ground),
                    (Coord::new(3, 1), PipeKind::NorthEast),
                    (Coord::new(3, 2), PipeKind::EastWest),
                    (Coord::new(3, 3), PipeKind::NorthWest),
                    (Coord::new(3, 4), PipeKind::Ground),
                    (Coord::new(4, 0), PipeKind::Ground),
                    (Coord::new(4, 1), PipeKind::Ground),
                    (Coord::new(4, 2), PipeKind::Ground),
                    (Coord::new(4, 3), PipeKind::Ground),
                    (Coord::new(4, 4), PipeKind::Ground),
                ]),
                row_count: 5,
                column_count: 5
            }
        );
    }

    fn map_example_2() -> Map {
        Map {
            start_position: Some(Coord::new(1, 1)),
            lookup: BTreeMap::from([
                (Coord::new(0, 0), PipeKind::Ground),
                (Coord::new(0, 1), PipeKind::Ground),
                (Coord::new(0, 2), PipeKind::Ground),
                (Coord::new(0, 3), PipeKind::Ground),
                (Coord::new(0, 4), PipeKind::Ground),
                (Coord::new(1, 0), PipeKind::Ground),
                (Coord::new(1, 1), PipeKind::Start),
                (Coord::new(1, 2), PipeKind::EastWest),
                (Coord::new(1, 3), PipeKind::SouthWest),
                (Coord::new(1, 4), PipeKind::Ground),
                (Coord::new(2, 0), PipeKind::Ground),
                (Coord::new(2, 1), PipeKind::NorthSouth),
                (Coord::new(2, 2), PipeKind::Ground),
                (Coord::new(2, 3), PipeKind::NorthSouth),
                (Coord::new(2, 4), PipeKind::Ground),
                (Coord::new(3, 0), PipeKind::Ground),
                (Coord::new(3, 1), PipeKind::NorthEast),
                (Coord::new(3, 2), PipeKind::EastWest),
                (Coord::new(3, 3), PipeKind::NorthWest),
                (Coord::new(3, 4), PipeKind::Ground),
                (Coord::new(4, 0), PipeKind::Ground),
                (Coord::new(4, 1), PipeKind::Ground),
                (Coord::new(4, 2), PipeKind::Ground),
                (Coord::new(4, 3), PipeKind::Ground),
                (Coord::new(4, 4), PipeKind::Ground),
            ]),
            row_count: 5,
            column_count: 5,
        }
    }

    #[test]
    fn should_parse_input_with_start() {
        assert_eq!(Map::parse(EXAMPLE2), map_example_2());
    }

    #[test]
    fn should_find_steps_to_farthest_point() {
        assert_eq!(map_example_2().find_steps_to_farthest_point(), 4);
    }

    #[test]
    fn should_find_pipekind_in_direction() {
        assert!(PipeKind::NorthSouth.is_connected(PipeKind::NorthSouth, Direction::North));
        assert!(!PipeKind::NorthSouth.is_connected(PipeKind::EastWest, Direction::North));
    }
}
