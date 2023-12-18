static EXAMPLE1: &str = "0 3 6 9 12 15\n1 3 6 10 15 21\n10 13 16 21 30 45";

fn main() {
    let input = include_str!("./input.txt");

    let sensor = Sensor::parse(input);
    let part_1 = sensor.sum_extrapolated_next_values();
    dbg!(part_1);

    let part_2 = sensor.sum_extrapolated_previous_values();
    dbg!(part_2);
}

#[derive(Clone, Debug, PartialEq)]
pub struct SensorValue {
    pub history: Vec<isize>,
}

impl SensorValue {
    pub fn parse(input: &str) -> Self {
        let history = input
            .split_ascii_whitespace()
            .map(|item| item.parse().unwrap())
            .collect();
        Self { history }
    }

    pub fn process_history(&self) -> Vec<Vec<isize>> {
        let mut levels: Vec<Vec<isize>> = vec![self.history.clone()];
        let mut current_level_idx = 0;
        let mut idx = 1;

        loop {
            let difference = {
                let current_level = levels.get_mut(current_level_idx).unwrap();

                current_level[idx] - current_level[idx - 1]
            };

            {
                if let Some(next_level) = levels.get_mut(current_level_idx + 1) {
                    next_level.push(difference);
                } else {
                    levels.push(vec![difference]);
                    idx += 1;
                    continue;
                }
            }

            let next_level = levels.get(current_level_idx + 1).unwrap();
            let is_all_level_values_the_same = next_level
                .iter()
                .all(|&x| x == levels[current_level_idx + 1][0]);

            let is_end_of_cycle = next_level.len() + 1 == self.history.len() - current_level_idx;

            if is_end_of_cycle && is_all_level_values_the_same {
                break;
            } else if is_end_of_cycle {
                current_level_idx += 1;
                idx = 0;
            }

            idx += 1;
        }

        levels
    }

    pub fn predict_previous_step(&self) -> isize {
        let levels = self.process_history();

        levels
            .into_iter()
            .rev()
            .fold(0_isize, |acc, item| -acc + item[0])
    }

    pub fn predict_next_step(&self) -> isize {
        let levels = self.process_history();

        levels
            .into_iter()
            .rev()
            .fold(0_isize, |acc, item| acc + item[item.len() - 1])
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Sensor {
    pub values: Vec<SensorValue>,
}

impl Sensor {
    pub fn parse(input: &str) -> Self {
        let values = input
            .split_terminator('\n')
            .map(SensorValue::parse)
            .collect();
        Self { values }
    }

    pub fn sum_extrapolated_previous_values(&self) -> isize {
        self.values.iter().map(|item| item.predict_previous_step()).sum()
    }

    pub fn sum_extrapolated_next_values(&self) -> isize {
        self.values.iter().map(|item| item.predict_next_step()).sum()
    }
}

mod tests {
    use crate::*;

    fn example_sensor() -> Sensor {
        Sensor {
            values: vec![
                SensorValue {
                    history: vec![0, 3, 6, 9, 12, 15],
                },
                SensorValue {
                    history: vec![1, 3, 6, 10, 15, 21],
                },
                SensorValue {
                    history: vec![10, 13, 16, 21, 30, 45],
                },
            ],
        }
    }

    #[test]
    fn should_parse_input() {
        assert_eq!(Sensor::parse(EXAMPLE1), example_sensor())
    }

    #[test]
    fn should_predict_next_step() {
        assert_eq!(example_sensor().values[0].predict_next_step(), 18);
        assert_eq!(example_sensor().values[1].predict_next_step(), 28);
        assert_eq!(example_sensor().values[2].predict_next_step(), 68);
    }

    #[test]
    fn should_extrapolate_next_values() {
        assert_eq!(example_sensor().sum_extrapolated_next_values(), 114);
    }

    #[test]
    fn should_predict_previous_step() {
        assert_eq!(example_sensor().values[0].predict_previous_step(), -3);
        assert_eq!(example_sensor().values[1].predict_previous_step(), 0);
        assert_eq!(example_sensor().values[2].predict_previous_step(), 5);
    }

    #[test]
    fn should_extrapolate_previous_values() {
        assert_eq!(example_sensor().sum_extrapolated_previous_values(), 2);
    }
}
