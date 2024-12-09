fn main() {
    let input = include_str!("./input.txt");

    println!("Part 1: {}", AllCalibrations::parse(input).total_valid_calibrations());
}

#[derive(Copy, Clone, Debug)]
pub enum Operator {
    Add,
    Multiply,
}

#[derive(Clone, Debug)]
pub struct Calibration {
    pub expected_result: u64,
    pub values: Vec<u64>,
}

impl Calibration {
    pub fn parse(input: &str) -> Self {
        let mut parts = input.split(": ");
        let expected_result = parts.next().unwrap().parse().unwrap();
        let values = parts
            .next()
            .unwrap()
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        Self {
            expected_result,
            values,
        }
    }

    pub fn produces_expected_result(&self) -> bool {
        // Odometer represents the current state of the operators
        // 0 = Add, 1 = Multiply
        // for each iteration the operator is incremented until it reaches the maximum values then it resets to 0 advancing the more significant array index
        let mut odometer = 0usize;
        let mut is_valid = false;

        // (2 operator options) ^ (2 operators required due to 3 values) = 4 permutations
        // because of this max-operators is always values - 1
        let max_bits = 2usize.pow(self.values.len() as u32 - 1);
        // 2^2 = 4 max permutations
        // p1: 0, 0 e.g 1 + 2 + 3
        // p2: 0, 1 e.g 1 + 2 * 3
        // p3: 1, 0 e.g 1 * 2 + 3
        // p4: 1, 1 e.g 1 * 2 * 3

        while odometer < max_bits {
            let mut result = self.values[0];

            // Iterate through each bit (from LSB to MSB)
            for i in 0..self.values.len() - 1 {
                let bit = (odometer >> i) & 1; // Extract the i-th bit
                // println!("Bit {}: {}", i, bit);
                // println!("Value: {}, idx: {}", self.values[i + 1], i + 1);
                match bit {
                    0 => result += self.values[i + 1],
                    1 => result *= self.values[i + 1],
                    _ => panic!("Invalid operator"),
                }
            }

            if result == self.expected_result {
                is_valid = true;
                break;
            }

            odometer += 1;
        }

        is_valid
    }
}

#[derive(Clone, Debug)]
pub struct AllCalibrations {
    pub calibrations: Vec<Calibration>,
}

impl AllCalibrations {
    pub fn parse(input: &str) -> Self {
        Self {
            calibrations: input.lines().map(Calibration::parse).collect::<Vec<_>>(),
        }
    }

    pub fn total_valid_calibrations(&self) -> u64 {
        self.calibrations
            .iter()
            .filter(|c| c.produces_expected_result())
            .map(|c| c.expected_result)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn should_produce_expected_results() {
        assert!(Calibration::parse("190: 10 19").produces_expected_result());
        assert!(Calibration::parse("3267: 81 40 27").produces_expected_result());
        assert!(Calibration::parse("292: 11 6 16 20").produces_expected_result());
        assert!(!Calibration::parse("83: 17 5").produces_expected_result());
        assert!(!Calibration::parse("7290: 6 8 6 15").produces_expected_result());
    }

    #[test]
    fn should_sum_valid_calibrations() {
        let input = "190: 10 19\n\
                           3267: 81 40 27\n\
                           83: 17 5\n\
                           156: 15 6\n\
                           7290: 6 8 6 15\n\
                           161011: 16 10 13\n\
                           192: 17 8 14\n\
                           21037: 9 7 18 13\n\
                           292: 11 6 16 20";
        
        assert_eq!(AllCalibrations::parse(input).total_valid_calibrations(), 3749);
    }
}
