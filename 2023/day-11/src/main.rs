#[derive(Copy, Clone, Debug, PartialEq)]
struct Galaxy {
    pub row: usize,
    pub col: usize,
}

fn main() {
    let input = include_str!("./input.txt");

    let universe_map = expand_universe(input);

    let mut galaxies: Vec<Galaxy> = Vec::new();

    for (row_idx, line) in universe_map.into_iter().enumerate() {
        for (col_idx, ch) in line.chars().enumerate() {
            if ch == '#' {
                galaxies.push(Galaxy {
                    row: row_idx,
                    col: col_idx,
                });
            }
        }
    }

    let sum_of_lengths: i32 = create_galaxy_pairs(galaxies)
        .into_iter()
        .map(|(galaxy1, galaxy2)| {
            (galaxy1.row as i32 - galaxy2.row as i32).abs()
                + (galaxy1.col as i32 - galaxy2.col as i32).abs()
        })
        .sum();

    println!("Sum of lengths: {}", sum_of_lengths);
}

fn expand_rows(mut lines: Vec<String>) -> Vec<String> {
    let column_count = lines[0].chars().count();

    let mut current_row_idx = 0;

    // insert empty rows
    for line in lines.clone().iter() {
        if line.chars().all(|ch| ch == '.') {
            lines.insert(current_row_idx, ".".repeat(column_count));
            current_row_idx += 1;
        }
        current_row_idx += 1;
    }

    lines
}

fn expand_columns(mut lines: Vec<String>) -> Vec<String> {
    let column_count = lines[0].chars().count();

    let mut current_column_idx = 0;

    let lines_to_iter = lines.clone();

    // insert empty cols
    for col_idx in 0..column_count {
        let mut has_galaxy = false;

        for line in lines_to_iter.iter() {
            if line.chars().nth(col_idx).unwrap() == '#' {
                has_galaxy = true;
                break;
            }
        }

        if !has_galaxy {
            for line in lines.iter_mut() {
                line.insert(current_column_idx, '.');
            }
            current_column_idx += 1;
        }

        current_column_idx += 1;
    }

    lines
}

fn expand_universe(input: &str) -> Vec<String> {
    let lines: Vec<String> = input.lines().map(String::from).collect();
    expand_columns(expand_rows(lines))
}

fn create_galaxy_pairs(galaxies: Vec<Galaxy>) -> Vec<(Galaxy, Galaxy)> {
    let mut galaxy_pairs: Vec<(Galaxy, Galaxy)> = Vec::new();

    for (idx, galaxy) in galaxies.iter().enumerate() {
        for other_galaxy in galaxies.iter().skip(idx + 1) {
            galaxy_pairs.push((*galaxy, *other_galaxy));
        }
    }

    galaxy_pairs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_expand_rows() {
        let lines = vec![
            String::from("....."),
            String::from("....."),
            String::from(".#..."),
            String::from("..#.."),
            String::from("....."),
        ];
        assert_eq!(
            expand_rows(lines),
            vec![
                String::from("....."),
                String::from("....."),
                String::from("....."),
                String::from("....."),
                String::from(".#..."),
                String::from("..#.."),
                String::from("....."),
                String::from("....."),
            ]
        )
    }

    #[test]
    fn should_expand_columns() {
        let lines = vec![
            String::from("....."),
            String::from("....."),
            String::from(".#..."),
            String::from("..#.."),
            String::from("....."),
        ];
        assert_eq!(
            expand_columns(lines),
            vec![
                String::from("........"),
                String::from("........"),
                String::from("..#....."),
                String::from("...#...."),
                String::from("........"),
            ]
        )
    }

    #[test]
    fn should_expand_universe() {
        let input = "..#..\n.....\n.....\n...#.\n.....";

        let expected_output =
            "....#...\n........\n........\n........\n........\n.....#..\n........\n........";

        assert_eq!(&expand_universe(input).join("\n"), expected_output);
    }

    #[test]
    fn should_create_galaxy_pairs() {
        assert_eq!(
            create_galaxy_pairs(vec![
                Galaxy { row: 0, col: 0 },
                Galaxy { row: 1, col: 2 },
                Galaxy { row: 3, col: 4 },
                Galaxy { row: 5, col: 6 },
            ]),
            vec![
                (Galaxy { row: 0, col: 0 }, Galaxy { row: 1, col: 2 }),
                (Galaxy { row: 0, col: 0 }, Galaxy { row: 3, col: 4 }),
                (Galaxy { row: 0, col: 0 }, Galaxy { row: 5, col: 6 }),
                (Galaxy { row: 1, col: 2 }, Galaxy { row: 3, col: 4 }),
                (Galaxy { row: 1, col: 2 }, Galaxy { row: 5, col: 6 }),
                (Galaxy { row: 3, col: 4 }, Galaxy { row: 5, col: 6 }),
            ]
        );
    }
}
