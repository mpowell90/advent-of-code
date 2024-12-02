#[derive(Copy, Clone, Debug, PartialEq)]
enum Direction {
    None,
    Incrementing,
    Decrementing,
}

impl Direction {
    pub fn parse(val: isize) -> Self {
        match val {
            0 => Self::None,
            val if val > 0 => Self::Incrementing,
            _ => Self::Decrementing,
        }
    }
}

#[derive(Clone, Debug)]
struct Report {
    pub levels: Vec<isize>,
}

impl Report {
    pub fn parse(input: &str) -> Self {
        let levels = input
            .split_whitespace()
            .map(|line| line.parse().unwrap())
            .collect();

        Self { levels }
    }

    pub fn is_safe(&self) -> bool {
        let mut direction: Option<Direction> = None;
        let mut last_level: Option<isize> = None;
        let mut max_change = 0;

        for level in self.levels.iter() {
            let Some(last_level_check) = last_level else {
                last_level = Some(*level);
                continue;
            };

            let change = *level - last_level_check;

            let next_direction = Direction::parse(change);

            if direction.is_some_and(|direction| {
                direction == Direction::None || direction != next_direction
            }) {
                return false;
            } else {
                direction = Some(next_direction);
            }

            if change.abs() > max_change {
                max_change = change.abs();
            }

            last_level = Some(*level);
        }

        if (1..=3).contains(&max_change) {
            return true;
        }

        false
    }
}

fn main() {
    let input = include_str!("./input.txt");

    let safe_report_count: usize = input
        .lines()
        .map(|line| Report::parse(line).is_safe())
        .filter(|&is_safe| is_safe)
        .count();

    println!("Safe report count: {}", safe_report_count);
}

#[cfg(test)]
mod tests {
    use crate::Report;

    #[test]
    fn should_check_report_safety() {
        assert!(Report::parse("7 6 4 2 1").is_safe());
        assert!(!Report::parse("1 2 7 8 9").is_safe());
        assert!(!Report::parse("9 7 6 2 1").is_safe());
        assert!(!Report::parse("1 3 2 4 5").is_safe());
        assert!(!Report::parse("8 6 4 4 1").is_safe());
        assert!(Report::parse("1 3 6 7 9").is_safe());
    }
}
