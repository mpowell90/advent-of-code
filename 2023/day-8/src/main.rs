use std::collections::{BTreeMap, VecDeque};

static EXAMPLE1: &str = "RL\n\nAAA = (BBB, CCC)\nBBB = (DDD, EEE)\nCCC = (ZZZ, GGG)\nDDD = (DDD, DDD)\nEEE = (EEE, EEE)\nGGG = (GGG, GGG)\nZZZ = (ZZZ, ZZZ)";

fn main() {
    let input = include_str!("./input.txt");

    let map = Map::parse(input);
    let part_1 = map.calculate_steps_to_traverse();
    dbg!(part_1);
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum InstructionKind {
    Left,
    Right,
}

impl TryFrom<char> for InstructionKind {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Ok(Self::Left),
            'R' => Ok(Self::Right),
            _ => Err("Instruction kind cannot be converted".to_string()),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct NetworkSegment {
    pub left: String,
    pub right: String,
}

impl NetworkSegment {
    pub fn new(left: String, right: String) -> Self {
        Self { left, right }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Map {
    pub instructions: VecDeque<InstructionKind>,
    pub network: BTreeMap<String, NetworkSegment>,
}

impl Map {
    pub fn parse(input: &str) -> Self {
        let mut input_parts = input.split_terminator("\n\n");

        let instructions = input_parts
            .next()
            .unwrap()
            .chars()
            .map(|ch| InstructionKind::try_from(ch).unwrap())
            .collect();

        let network = input_parts
            .next()
            .unwrap()
            .lines()
            .map(|line| {
                let mut line_parts = line.split_terminator(" = ");

                let start = line_parts.next().unwrap().to_string();

                let mut segments = line_parts.next().unwrap().split_terminator(", ");
                let left = segments.next().unwrap().replace('(', "");
                let right = segments.next().unwrap().replace(')', "");

                (start, NetworkSegment { left, right })
            })
            .collect();

        Self {
            instructions,
            network,
        }
    }

    pub fn calculate_steps_to_traverse(&self) -> usize {
        let mut instructions = self.instructions.clone();
        let mut steps = 0;

        let mut next_address = Some("AAA".to_string());

        while let Some(address) = next_address.as_ref() {
            let next_instruction = if let Some(next_instruction) = instructions.pop_front() {
                next_instruction
            } else {
                instructions = self.instructions.clone();
                instructions.pop_front().unwrap()
            };

            if let Some(network_segment) = self.network.get(address) {
                let network_segment_address = match next_instruction {
                    InstructionKind::Left => network_segment.left.to_owned(),
                    InstructionKind::Right => network_segment.right.to_owned(),
                };

                steps += 1;

                if &network_segment_address == "ZZZ" {
                    break;
                } else {
                    next_address = Some(network_segment_address);
                }
            } else {
                panic!("fail!")
            }
        }

        steps
    }
}

mod tests {
    use crate::*;

    fn example_map() -> Map {
        Map {
            instructions: VecDeque::from([InstructionKind::Right, InstructionKind::Left]),
            network: BTreeMap::from([
                (
                    "AAA".to_string(),
                    NetworkSegment::new("BBB".to_string(), "CCC".to_string()),
                ),
                (
                    "BBB".to_string(),
                    NetworkSegment::new("DDD".to_string(), "EEE".to_string()),
                ),
                (
                    "CCC".to_string(),
                    NetworkSegment::new("ZZZ".to_string(), "GGG".to_string()),
                ),
                (
                    "DDD".to_string(),
                    NetworkSegment::new("DDD".to_string(), "DDD".to_string()),
                ),
                (
                    "EEE".to_string(),
                    NetworkSegment::new("EEE".to_string(), "EEE".to_string()),
                ),
                (
                    "GGG".to_string(),
                    NetworkSegment::new("GGG".to_string(), "GGG".to_string()),
                ),
                (
                    "ZZZ".to_string(),
                    NetworkSegment::new("ZZZ".to_string(), "ZZZ".to_string()),
                ),
            ]),
        }
    }

    #[test]
    fn should_parse_input() {
        assert_eq!(Map::parse(EXAMPLE1), example_map())
    }

    #[test]
    fn should_calculate_steps_to_traverse() {
        assert_eq!(example_map().calculate_steps_to_traverse(), 2)
    }
}
