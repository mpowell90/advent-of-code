fn main() {
    let input = include_str!("./input.txt");

    let part_1 = parse_multiline_to_number(input, |line| parse_number_from_line(line).unwrap());
    println!("part 1: {part_1}");

    let part_2 = parse_multiline_to_number(input, |line| {
        parse_number_or_number_str_from_line(line).unwrap()
    });
    println!("part 2: {part_2}");
}

pub fn simple_match_number_word_in_str_slice(
    char: char,
    line: &str,
    current_index: usize,
) -> Option<u32> {
    match char {
        'o' => {
            if current_index + "one".len() <= line.len()
                && &line[current_index..(current_index + "one".len())] == "one"
            {
                Some(1)
            } else {
                None
            }
        }
        't' => {
            if current_index + "two".len() <= line.len()
                && &line[current_index..(current_index + "two".len())] == "two"
            {
                Some(2)
            } else if current_index + "three".len() <= line.len()
                && &line[current_index..(current_index + "three".len())] == "three"
            {
                Some(3)
            } else {
                None
            }
        }
        'f' => {
            if current_index + "four".len() <= line.len()
                && &line[current_index..(current_index + "four".len())] == "four"
            {
                Some(4)
            } else if current_index + "five".len() <= line.len()
                && &line[current_index..(current_index + "five".len())] == "five"
            {
                Some(5)
            } else {
                None
            }
        }
        's' => {
            if current_index + "six".len() <= line.len()
                && &line[current_index..(current_index + "six".len())] == "six"
            {
                Some(6)
            } else if current_index + "seven".len() <= line.len()
                && &line[current_index..(current_index + "seven".len())] == "seven"
            {
                Some(7)
            } else {
                None
            }
        }
        'e' => {
            if current_index + "eight".len() <= line.len()
                && &line[current_index..(current_index + "eight".len())] == "eight"
            {
                Some(8)
            } else {
                None
            }
        }
        'n' => {
            if current_index + "nine".len() <= line.len()
                && &line[current_index..(current_index + "nine".len())] == "nine"
            {
                Some(9)
            } else {
                None
            }
        }
        _ => None,
    }
}

pub fn parse_number_from_line(line: &str) -> anyhow::Result<u32> {
    let mut numbers: Vec<u32> = vec![];
    let mut num_as_string: String = String::with_capacity(2);

    for char in line.chars() {
        if char.is_ascii_digit() {
            if let Some(number) = char.to_digit(10) {
                numbers.push(number);
            }
        }
    }

    if let Some(&first) = numbers.first() {
        num_as_string.push(char::from_digit(first, 10).unwrap());
    }

    if let Some(&last) = numbers.last() {
        num_as_string.push(char::from_digit(last, 10).unwrap());
    }

    num_as_string
        .parse::<u32>()
        .map_err(|error| anyhow::format_err!(error))
}

pub fn parse_number_or_number_str_from_line(line: &str) -> anyhow::Result<u32> {
    let numbers: Vec<u32> =
        line.chars()
            .enumerate()
            .fold(vec![], |mut acc: Vec<u32>, (idx, char)| {
                if let Some(number) = char.to_digit(10) {
                    acc.push(number);
                } else if let Some(number) = simple_match_number_word_in_str_slice(char, line, idx)
                {
                    acc.push(number);
                }
                acc
            });

    let mut num_as_string: String = String::with_capacity(2);

    if let Some(&first) = numbers.first() {
        num_as_string.push(char::from_digit(first, 10).unwrap());
    }

    if let Some(&last) = numbers.last() {
        num_as_string.push(char::from_digit(last, 10).unwrap());
    }

    num_as_string
        .parse::<u32>()
        .map_err(|error| anyhow::format_err!(error))
}

pub fn parse_multiline_to_number(multiline: &str, cb: fn(&str) -> u32) -> u32 {
    multiline.lines().map(cb).sum::<u32>()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn should_parse_number_from_line() {
        assert_eq!(parse_number_from_line("1abc2").unwrap(), 12);
        assert_eq!(parse_number_from_line("pqr3stu8vwx").unwrap(), 38);
        assert_eq!(parse_number_from_line("a1b2c3d4e5f").unwrap(), 15);
        assert_eq!(parse_number_from_line("treb7uchet").unwrap(), 77);
    }

    #[test]
    fn should_parse_number_or_number_str_from_line_with_queue() {
        assert_eq!(
            parse_number_or_number_str_from_line("two1nine").unwrap(),
            29
        );
        assert_eq!(
            parse_number_or_number_str_from_line("zoneight234").unwrap(),
            14
        );
        assert_eq!(
            parse_number_or_number_str_from_line("jnccdbplkfq6oneightd").unwrap(),
            68
        );
    }

    #[test]
    fn part_1() {
        assert_eq!(
            parse_multiline_to_number("1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet", |line| {
                parse_number_from_line(line).unwrap()
            }),
            142
        );
    }

    #[test]
    fn part_2() {
        assert_eq!(parse_multiline_to_number("two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen", |line| parse_number_or_number_str_from_line(line).unwrap()), 281);
    }
}
