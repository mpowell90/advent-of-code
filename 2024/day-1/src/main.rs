use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");

    let total_distance = calculate_total_distance(input);
    println!("Part 1 total distance: {}", total_distance);

    let similarity_score = calculate_similarity_score(input);
    println!("Part 2 similarity score: {}", similarity_score);
}

fn split_input_into_pairs(input: &str) -> (Vec<usize>, Vec<usize>) {
    input
        .lines()
        .map(|line| {
            let mut iter = line.split_ascii_whitespace();
            let left: usize = iter.next().unwrap().parse().unwrap();
            let right: usize = iter.next().unwrap().parse().unwrap();
            (left, right)
        })
        .unzip()
}

fn calculate_total_distance(input: &str) -> isize {
    let (mut left_values, mut right_values): (Vec<usize>, Vec<usize>) =
        split_input_into_pairs(input);

    left_values.sort();
    right_values.sort();

    let total_distance: isize = std::iter::zip(left_values, right_values)
        .map(|(left, right)| (left as isize - right as isize).abs())
        .sum();

    total_distance
}

fn calculate_similarity_score(input: &str) -> usize {
    let (left_values, right_values): (Vec<usize>, Vec<usize>) = split_input_into_pairs(input);

    let right_lookup =
        right_values
            .into_iter()
            .fold(HashMap::new(), |mut acc: HashMap<usize, usize>, val| {
                *acc.entry(val).or_default() += 1;
                acc
            });

    let similarity_score = left_values
        .into_iter()
        .map(|val| val * right_lookup.get(&val).unwrap_or(&0))
        .sum();

    similarity_score
}
