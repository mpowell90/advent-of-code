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
    static EXAMPLE1: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\nCard 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\nCard 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\nCard 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\nCard 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\nCard 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    // static EXAMPLE2: &str = "...*.=....376......................................69..........&.....116*949.......................186..295............%.................833\n261...853...*...81........993.191.810...731..........*575......636..................306...........*............129....691....999*.+712......";

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
