fn main() {
    let input = include_str!("./input.txt");

    let competition = Competition::parse(input);
    let part_1 = competition.calculate_margin_of_error();
    dbg!(part_1);
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Race {
    time: usize,     // milliseconds
    distance: usize, // millimeters
}

impl Race {
    pub fn new(time: usize, distance: usize) -> Self {
        Self { time, distance }
    }

    pub fn calculate_winning_races(&self) -> Vec<usize> {
        (1..self.time).fold(vec![], |mut acc, press_time| {
            let distance_covered = (self.time - press_time) * press_time;

            if distance_covered > self.distance {
                acc.push(press_time);
            }

            acc
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Competition {
    pub races: Vec<Race>,
}

impl Competition {
    pub fn parse(input: &str) -> Self {
        let input_parts = input.split_terminator('\n').collect::<Vec<&str>>();

        let times = input_parts[0]
            .split_terminator(':')
            .skip(1)
            .flat_map(|result_items| result_items.trim().split_ascii_whitespace())
            .map(|number_string| number_string.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        let distances = input_parts[1]
            .split_terminator(':')
            .skip(1)
            .flat_map(|result_items| result_items.trim().split_ascii_whitespace())
            .map(|number_string| number_string.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        let races = times
            .into_iter()
            .zip(distances)
            .map(|(time, distance)| Race::new(time, distance))
            .collect();

        Self { races }
    }

    pub fn calculate_margin_of_error(&self) -> usize {
        self.races.iter().fold(1, |mut acc, race| {
            acc *= race.calculate_winning_races().len();
            acc
        })
    }
}

mod tests {
    static EXAMPLE1: &str = "Time:      7  15   30\nDistance:  9  40  200";

    #[test]
    fn should_parse_competition() {
        assert_eq!(
            crate::Competition::parse(EXAMPLE1),
            crate::Competition {
                races: vec![
                    crate::Race::new(7, 9),
                    crate::Race::new(15, 40),
                    crate::Race::new(30, 200)
                ]
            }
        );
    }

    #[test]
    fn should_calculate_winning_races() {
        assert_eq!(
            crate::Race::new(7, 9).calculate_winning_races(),
            vec![2, 3, 4, 5]
        );
    }

    #[test]
    fn should_calculate_margin_of_errors() {
        let competition = crate::Competition {
            races: vec![
                crate::Race::new(7, 9),
                crate::Race::new(15, 40),
                crate::Race::new(30, 200),
            ],
        };

        assert_eq!(competition.calculate_margin_of_error(), 288);
    }
}
