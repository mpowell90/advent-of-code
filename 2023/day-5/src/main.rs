fn main() {
    let input = include_str!("./input.txt");
    let almanac = Almanac::parse(input).unwrap();
    let part_1 = almanac.lowest_location_number();
    dbg!(part_1);
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MapRange {
    destination_start: usize,
    source_start: usize,
    range_length: usize,
}

impl MapRange {
    pub fn parse(line: &str) -> Result<Self, String> {
        let map_range_parts = line.split_ascii_whitespace().collect::<Vec<&str>>();

        if map_range_parts.len() < 3 {
            return Err("not enough values to parse map range".to_string());
        }

        let map_range_numbers = map_range_parts
            .into_iter()
            .map(|number_string| number_string.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        Ok(Self {
            destination_start: map_range_numbers[0],
            source_start: map_range_numbers[1],
            range_length: map_range_numbers[2],
        })
    }

    pub fn get_mapped_value(&self, input: usize) -> Option<usize> {
        let range = self.source_start..=self.source_start + self.range_length;

        if range.contains(&input) {
            let difference = self.destination_start as isize - self.source_start as isize;
            Some(input.saturating_add_signed(difference))
        } else {
            None
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MapType {
    SeedToSoil,
    SoilToFertiliser,
    FertiliserToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumidity,
    HumidityToLocation,
}

impl<'a> TryFrom<&'a str> for MapType {
    type Error = String;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        match value {
            "seed-to-soil" => Ok(MapType::SeedToSoil),
            "soil-to-fertilizer" => Ok(MapType::SoilToFertiliser),
            "fertilizer-to-water" => Ok(MapType::FertiliserToWater),
            "water-to-light" => Ok(MapType::WaterToLight),
            "light-to-temperature" => Ok(MapType::LightToTemperature),
            "temperature-to-humidity" => Ok(MapType::TemperatureToHumidity),
            "humidity-to-location" => Ok(MapType::HumidityToLocation),
            _ => Err("No match".to_string()),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Category {
    Seed(usize),
    Soil(usize),
    Fertilizer(usize),
    Water(usize),
    Light(usize),
    Temperature(usize),
    Humidity(usize),
    Location(usize),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Map {
    kind: MapType,
    map_ranges: Vec<MapRange>,
}

impl Map {
    pub fn parse(input: &str) -> Result<Self, String> {
        let mut map_parts = input.split_terminator(" map:\n");

        let Some(map_type_part) = map_parts.next() else {
            return Err("Cannot parse map_type".to_string());
        };

        let kind: MapType = map_type_part.try_into()?;

        let Some(map_ranges_part) = map_parts.next() else {
            return Err("Cannot parse map_ranges".to_string());
        };

        let map_ranges = map_ranges_part
            .lines()
            .map(|line| MapRange::parse(line).unwrap())
            .collect::<Vec<MapRange>>();

        Ok(Self { kind, map_ranges })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Almanac {
    seeds: Vec<usize>,
    maps: Vec<Map>,
}

impl Almanac {
    pub fn parse(input: &str) -> Result<Self, String> {
        let mut input_parts = input.split_terminator("\n\n");

        let Some(seed_string) = input_parts.next() else {
            return Err("Seed string failed".to_string());
        };

        let seed_string_parts = seed_string.split_terminator(": ").collect::<Vec<&str>>();
        let seeds = seed_string_parts[1]
            .split_ascii_whitespace()
            .map(|item| item.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        let maps = input_parts
            .map(|input_part| Map::parse(input_part).unwrap())
            .collect();

        Ok(Self { seeds, maps })
    }

    pub fn traverse_categories(&self, seed: usize) -> usize {
        let mut current = seed;

        for map in self.maps.iter() {
            for map_range in map.map_ranges.iter() {
                if let Some(mapped_value) = map_range.get_mapped_value(current) {
                    current = mapped_value;
                    break;
                }
            }
        }

        current
    }

    pub fn lowest_location_number(&self) -> usize {
        let mut locations = self.seeds.iter().map(|seed| self.traverse_categories(*seed)).collect::<Vec<usize>>();

        locations.sort();

        locations[0]
    }
}

mod tests {
    use crate::Almanac;

    static EXAMPLE1: &str = "seeds: 79 14 55 13\n\nseed-to-soil map:\n50 98 2\n52 50 48\n\nsoil-to-fertilizer map:\n0 15 37\n37 52 2\n39 0 15\n\nfertilizer-to-water map:\n49 53 8\n0 11 42\n42 0 7\n57 7 4\n\nwater-to-light map:\n88 18 7\n18 25 70\n\nlight-to-temperature map:\n45 77 23\n81 45 19\n68 64 13\n\ntemperature-to-humidity map:\n0 69 1\n1 0 69\n\nhumidity-to-location map:\n60 56 37\n56 93 4";

    #[test]
    fn should_parse_line_to_map_range() {
        assert_eq!(
            crate::MapRange::parse("50 98 2").unwrap(),
            crate::MapRange {
                destination_start: 50,
                source_start: 98,
                range_length: 2,
            }
        );
    }

    #[test]
    fn should_parse_input_into_map() {
        assert_eq!(
            crate::Map::parse("seed-to-soil map:\n50 98 2\n52 50 48").unwrap(),
            crate::Map {
                kind: crate::MapType::SeedToSoil,
                map_ranges: vec![
                    crate::MapRange {
                        destination_start: 50,
                        source_start: 98,
                        range_length: 2
                    },
                    crate::MapRange {
                        destination_start: 52,
                        source_start: 50,
                        range_length: 48
                    }
                ]
            }
        );
    }

    fn return_parsed_example_almanac() -> Almanac {
        crate::Almanac {
            seeds: vec![79, 14, 55, 13],
            maps: vec![
                crate::Map {
                    kind: crate::MapType::SeedToSoil,
                    map_ranges: vec![
                        crate::MapRange {
                            destination_start: 50,
                            source_start: 98,
                            range_length: 2,
                        },
                        crate::MapRange {
                            destination_start: 52,
                            source_start: 50,
                            range_length: 48,
                        },
                    ],
                },
                crate::Map {
                    kind: crate::MapType::SoilToFertiliser,
                    map_ranges: vec![
                        crate::MapRange {
                            destination_start: 0,
                            source_start: 15,
                            range_length: 37,
                        },
                        crate::MapRange {
                            destination_start: 37,
                            source_start: 52,
                            range_length: 2,
                        },
                        crate::MapRange {
                            destination_start: 39,
                            source_start: 0,
                            range_length: 15,
                        },
                    ],
                },
                crate::Map {
                    kind: crate::MapType::FertiliserToWater,
                    map_ranges: vec![
                        crate::MapRange {
                            destination_start: 49,
                            source_start: 53,
                            range_length: 8,
                        },
                        crate::MapRange {
                            destination_start: 0,
                            source_start: 11,
                            range_length: 42,
                        },
                        crate::MapRange {
                            destination_start: 42,
                            source_start: 0,
                            range_length: 7,
                        },
                        crate::MapRange {
                            destination_start: 57,
                            source_start: 7,
                            range_length: 4,
                        },
                    ],
                },
                crate::Map {
                    kind: crate::MapType::WaterToLight,
                    map_ranges: vec![
                        crate::MapRange {
                            destination_start: 88,
                            source_start: 18,
                            range_length: 7,
                        },
                        crate::MapRange {
                            destination_start: 18,
                            source_start: 25,
                            range_length: 70,
                        },
                    ],
                },
                crate::Map {
                    kind: crate::MapType::LightToTemperature,
                    map_ranges: vec![
                        crate::MapRange {
                            destination_start: 45,
                            source_start: 77,
                            range_length: 23,
                        },
                        crate::MapRange {
                            destination_start: 81,
                            source_start: 45,
                            range_length: 19,
                        },
                        crate::MapRange {
                            destination_start: 68,
                            source_start: 64,
                            range_length: 13,
                        },
                    ],
                },
                crate::Map {
                    kind: crate::MapType::TemperatureToHumidity,
                    map_ranges: vec![
                        crate::MapRange {
                            destination_start: 0,
                            source_start: 69,
                            range_length: 1,
                        },
                        crate::MapRange {
                            destination_start: 1,
                            source_start: 0,
                            range_length: 69,
                        },
                    ],
                },
                crate::Map {
                    kind: crate::MapType::HumidityToLocation,
                    map_ranges: vec![
                        crate::MapRange {
                            destination_start: 60,
                            source_start: 56,
                            range_length: 37,
                        },
                        crate::MapRange {
                            destination_start: 56,
                            source_start: 93,
                            range_length: 4,
                        },
                    ],
                },
            ],
        }
    }

    #[test]
    fn should_parse_input_into_almanac() {
        assert_eq!(
            crate::Almanac::parse(EXAMPLE1).unwrap(),
            return_parsed_example_almanac()
        )
    }

    #[test]
    fn should_return_lowest_location_number() {
        assert_eq!(
            return_parsed_example_almanac()
                .lowest_location_number(),
            35
        )
    }

    // #[test]
    // fn should_process_total_scratch_cards_from_tally() {
    //     assert_eq!(
    //         crate::CardDeck::parse(EXAMPLE1)
    //             .process_tally()
    //             .into_values()
    //             .sum::<usize>(),
    //         30
    //     );
    // }
}
