const MAX_RED: u8 = 12;
const MAX_GREEN: u8 = 13;
const MAX_BLUE: u8 = 14;

fn main() {
    let input = include_str!("./input.txt");

    let part_1 = input.lines().fold(0, |mut acc: i32, line| {
        let game = Game::parse(line).unwrap();

        let is_possible = game.rounds.into_iter().all(|round| {
            round.red <= MAX_RED && round.green <= MAX_GREEN && round.blue <= MAX_BLUE
        });

        if is_possible {
            acc += game.id as i32;
        }

        acc
    });
    dbg!(part_1);
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Round {
    red: u8,
    green: u8,
    blue: u8,
}

impl Round {
    pub fn parse(line: &str) -> Result<Self, String> {
        let line_parts: Vec<&str> = line.split_terminator(", ").collect();

        let round = line_parts.into_iter().try_fold(
            Self {
                red: 0,
                green: 0,
                blue: 0,
            },
            |mut round, line_part| {
                let colour_parts: Vec<&str> = line_part.split_ascii_whitespace().collect();

                let value = colour_parts[0]
                    .parse::<u8>()
                    .map_err(|error| format!("Failed conversion to i32: {:?}", error.kind()))?;

                match colour_parts[1] {
                    "red" => round.red = value,
                    "green" => round.green = value,
                    "blue" => round.blue = value,
                    _ => return Err(String::from("Fail")),
                }

                Ok(round)
            },
        )?;

        Ok(round)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Game {
    id: u8,
    rounds: Vec<Round>,
}

impl Game {
    pub fn parse(line: &str) -> Result<Self, String> {
        let line_parts: Vec<&str> = line.split_terminator(": ").collect();

        let game_parts: Vec<&str> = line_parts[0].split_ascii_whitespace().collect();

        let id = game_parts[1]
            .parse::<u8>()
            .map_err(|error| format!("Failed conversion to i32: {:?}", error.kind()))?;

        let rounds = line_parts[1]
            .split_terminator("; ")
            .map(|round_line| Round::parse(round_line).unwrap())
            .collect();

        Ok(Self { id, rounds })
    }
}

mod tests {
    #[test]
    fn should_parse_line_into_game() {
        assert_eq!(
            crate::Game::parse("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green").unwrap(),
            crate::Game {
                id: 1,
                rounds: Vec::from([
                    crate::Round {
                        red: 4,
                        green: 0,
                        blue: 3
                    },
                    crate::Round {
                        red: 1,
                        green: 2,
                        blue: 6
                    },
                    crate::Round {
                        red: 0,
                        green: 2,
                        blue: 0
                    }
                ])
            }
        );
        assert_eq!(
            crate::Game::parse("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue")
                .unwrap(),
            crate::Game {
                id: 2,
                rounds: Vec::from([
                    crate::Round {
                        red: 0,
                        green: 2,
                        blue: 1
                    },
                    crate::Round {
                        red: 1,
                        green: 3,
                        blue: 4
                    },
                    crate::Round {
                        red: 0,
                        green: 1,
                        blue: 1
                    }
                ])
            }
        );
    }
}
