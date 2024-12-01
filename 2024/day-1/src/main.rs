fn main() {
    let input = include_str!("./input.txt");

    let (mut left_values, mut right_values): (Vec<isize>, Vec<isize>) = input
        .lines()
        .map(|line| {
            let mut iter = line.split_ascii_whitespace();
            let left: isize = iter.next().unwrap().parse().unwrap();
            let right: isize = iter.next().unwrap().parse().unwrap();
            (left, right)
        })
        .unzip();

    left_values.sort();
    right_values.sort();

    let total_distance: isize = std::iter::zip(left_values, right_values)
        .map(|(left, right)| (left - right).abs())
        .sum();

    dbg!(total_distance);
}
