use std::collections::BTreeSet;

fn main() {
    let input = include_str!("./input.txt");

    let part_1 = input.lines().map(|line| ScratchCard::parse(line).unwrap().calculate_points()).sum::<usize>();
    dbg!(part_1);
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScratchCard {
    pub id: usize,
    pub winning_numbers: BTreeSet<usize>,
    pub your_numbers: BTreeSet<usize>,
}

impl ScratchCard {
    pub fn parse(line: &str) -> Result<Self, String> {
        let line_parts: Vec<&str> = line.split_terminator(": ").collect();

        let id_parts: Vec<&str> = line_parts[0].split_ascii_whitespace().collect();

        let id = id_parts[1]
            .parse::<usize>()
            .map_err(|error| format!("Failed conversion to i32: {:?}", error.kind()))?;

        let card_parts = line_parts[1].split_terminator(" | ").collect::<Vec<&str>>();

        let winning_numbers = card_parts[0]
            .split_ascii_whitespace()
            .map(|number_string| number_string.parse::<usize>().unwrap())
            .collect::<BTreeSet<usize>>();

        let your_numbers = card_parts[1]
            .split_ascii_whitespace()
            .map(|number_string| number_string.parse::<usize>().unwrap())
            .collect::<BTreeSet<usize>>();

        Ok(Self {
            id,
            winning_numbers,
            your_numbers,
        })
    }

    pub fn find_matching_numbers(&self) -> Vec<usize> {
        self.winning_numbers.intersection(&self.your_numbers).cloned().collect()
    }

    pub fn calculate_points(&self) -> usize {
        self.find_matching_numbers().into_iter().fold(0, |acc, _item| {
            if acc == 0 {
                1
            } else {
                acc + acc
            }
        })
    }
}

mod tests {
    use std::collections::BTreeSet;

    #[test]
    fn should_parse_line() {
        assert_eq!(
            crate::ScratchCard::parse("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53").unwrap(),
            crate::ScratchCard {
                id: 1,
                winning_numbers: BTreeSet::from([41, 48, 83, 86, 17]),
                your_numbers: BTreeSet::from([83, 86, 6, 31, 17, 9, 48, 53]),
            }
        );
    }

    #[test]
    fn should_find_matching_numbers() {
        assert_eq!(
            crate::ScratchCard::parse("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53").unwrap().find_matching_numbers(),
            vec![17, 48, 83, 86]
        );
    }

    #[test]
    fn should_calculate_points() {
        assert_eq!(
            crate::ScratchCard::parse("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53").unwrap().calculate_points(),
            8
        );
    }
}
