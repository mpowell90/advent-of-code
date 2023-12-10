use std::collections::{BTreeMap, HashSet};

fn main() {
    let input = include_str!("./input.txt");
    let schematic = Schematic::parse(input).unwrap();
    let part_1 = schematic
        .find_valid_part_numbers()
        .into_iter()
        .sum::<usize>();
    dbg!(part_1);
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Coord {
    row: usize,
    column: usize,
}

impl Coord {
    pub fn new(row: usize, column: usize) -> Self {
        Self { row, column }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Schematic {
    pub symbols: Vec<Coord>,
    pub numbers: Vec<usize>,
    pub number_lookup: BTreeMap<Coord, usize>,
}

impl Schematic {
    pub fn parse(input: &str) -> Result<Self, String> {
        let mut number_string: Option<String> = None;

        let mut symbols = vec![];
        let mut numbers = vec![];
        let mut number_lookup = BTreeMap::new();

        for (row, line) in input.lines().enumerate() {
            for (column, ch) in line.char_indices() {
                match ch {
                    ch if ch.is_ascii_digit() => {
                        number_lookup.insert(Coord::new(row, column), numbers.len());

                        if let Some(ref mut inner) = number_string {
                            inner.push(ch);
                        } else {
                            let mut string = String::new();
                            string.push(ch);
                            number_string = Some(string);
                        }
                    }
                    ch => {
                        if let Some(ref inner) = number_string {
                            let number = inner.parse::<usize>().map_err(|error| {
                                format!("Failed conversion to i32: {:?}", error.kind())
                            })?;
                            numbers.push(number);
                            number_string = None;
                        }

                        if ch != '.' {
                            symbols.push(Coord::new(row, column));
                        }
                    }
                }
            }

            // a number could be present at the end of a row and the start of a new row
            if let Some(ref inner) = number_string {
                let number = inner
                    .parse::<usize>()
                    .map_err(|error| format!("Failed conversion to i32: {:?}", error.kind()))?;
                numbers.push(number);
                number_string = None;
            }
        }

        Ok(Self {
            symbols,
            numbers,
            number_lookup,
        })
    }

    pub fn find_valid_part_numbers(&self) -> Vec<usize> {
        let mut part_number_index_log: HashSet<usize> = HashSet::new();

        for coord in self.symbols.iter() {
            if let Some(top_left) = self
                .number_lookup
                .get(&Coord::new(coord.row - 1, coord.column - 1))
            {
                part_number_index_log.insert(*top_left);
            }
            if let Some(top_middle) = self
                .number_lookup
                .get(&Coord::new(coord.row - 1, coord.column))
            {
                part_number_index_log.insert(*top_middle);
            }
            if let Some(top_right) = self
                .number_lookup
                .get(&Coord::new(coord.row - 1, coord.column + 1))
            {
                part_number_index_log.insert(*top_right);
            }
            if let Some(left) = self
                .number_lookup
                .get(&Coord::new(coord.row, coord.column - 1))
            {
                part_number_index_log.insert(*left);
            }
            // middle will always be current coord
            if let Some(right) = self
                .number_lookup
                .get(&Coord::new(coord.row, coord.column + 1))
            {
                part_number_index_log.insert(*right);
            }
            if let Some(bottom_left) = self
                .number_lookup
                .get(&Coord::new(coord.row + 1, coord.column - 1))
            {
                part_number_index_log.insert(*bottom_left);
            }
            if let Some(bottom_middle) = self
                .number_lookup
                .get(&Coord::new(coord.row + 1, coord.column))
            {
                part_number_index_log.insert(*bottom_middle);
            }
            if let Some(bottom_right) = self
                .number_lookup
                .get(&Coord::new(coord.row + 1, coord.column + 1))
            {
                part_number_index_log.insert(*bottom_right);
            }
        }

        part_number_index_log
            .into_iter()
            .map(|idx| self.numbers[idx])
            .collect()
    }
}

mod tests {
    static EXAMPLE1: &str = "467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..";

    static EXAMPLE2: &str = "...*.=....376......................................69..........&.....116*949.......................186..295............%.................833\n261...853...*...81........993.191.810...731..........*575......636..................306...........*............129....691....999*.+712......";

    #[test]
    fn should_parse_line() {
        assert_eq!(
            crate::Schematic::parse("467..114..").unwrap(),
            crate::Schematic {
                symbols: vec![],
                numbers: vec![467, 114],
                number_lookup: std::collections::BTreeMap::from([
                    (crate::Coord::new(0, 0), 0),
                    (crate::Coord::new(0, 1), 0),
                    (crate::Coord::new(0, 2), 0),
                    (crate::Coord::new(0, 5), 1),
                    (crate::Coord::new(0, 6), 1),
                    (crate::Coord::new(0, 7), 1)
                ]),
            }
        );
        assert_eq!(
            crate::Schematic::parse(".....+.58.").unwrap(),
            crate::Schematic {
                symbols: vec![crate::Coord::new(0, 5)],
                numbers: vec![58],
                number_lookup: std::collections::BTreeMap::from([
                    (crate::Coord::new(0, 7), 0),
                    (crate::Coord::new(0, 8), 0)
                ]),
            }
        );
        assert_eq!(
            crate::Schematic::parse("...$.*....").unwrap(),
            crate::Schematic {
                symbols: vec![crate::Coord::new(0, 3), crate::Coord::new(0, 5)],
                numbers: vec![],
                number_lookup: std::collections::BTreeMap::new(),
            }
        );
    }

    #[test]
    fn should_parse_multiline_numbers() {
        assert_eq!(
            crate::Schematic::parse(EXAMPLE2).unwrap().numbers,
            vec![
                376, 69, 116, 949, 186, 295, 833, 261, 853, 81, 993, 191, 810, 731, 575, 636, 306,
                129, 691, 999, 712
            ]
        );
    }

    #[test]
    fn should_find_valid_part_numbers() {
        let mut part_numbers = crate::Schematic::parse(EXAMPLE1)
            .unwrap()
            .find_valid_part_numbers();

        part_numbers.sort();

        let mut correct: Vec<usize> = vec![467, 35, 633, 617, 592, 755, 664, 598];
        correct.sort();

        assert_eq!(part_numbers, correct);
    }
}
