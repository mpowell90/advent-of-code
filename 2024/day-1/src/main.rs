use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");

    let total_distance = calculate_total_distance(input);
    println!("Part 1 total distance: {}", total_distance);

    let similarity_score = calculate_similarity_score(input);
    println!("Part 2 similarity score: {}", similarity_score);
}

fn split_input_into_pairs(input: &str) -> impl Iterator<Item = (usize, usize)> + '_ {
    input.lines().map(|line| {
        let mut iter = line.split_ascii_whitespace();
        let left = iter.next().unwrap().parse().unwrap();
        let right = iter.next().unwrap().parse().unwrap();
        (left, right)
    })
}

fn calculate_total_distance(input: &str) -> isize {
    let (mut left_values, mut right_values): (Vec<usize>, Vec<usize>) =
        split_input_into_pairs(input).unzip();

    left_values.sort();
    right_values.sort();

    let total_distance: isize = std::iter::zip(left_values, right_values)
        .map(|(left, right)| (left as isize - right as isize).abs())
        .sum();

    total_distance
}

fn calculate_similarity_score(input: &str) -> usize {
    let (left_values, right_values): (Vec<usize>, Vec<usize>) =
        split_input_into_pairs(input).unzip();

    let left_lookup = left_values.iter().fold(
        HashMap::new(),
        |mut acc: HashMap<usize, usize>, left_value| {
            let right_count = right_values.iter().filter(|&x| x == left_value).count();

            *acc.entry(*left_value).or_insert(0) += right_count;

            acc
        },
    );

    let similarity_score = left_values
        .into_iter()
        .map(|val| val * left_lookup.get(&val).unwrap_or(&0))
        .sum();

    similarity_score
}
