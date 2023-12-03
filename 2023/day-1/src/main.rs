fn main() {
    let input = include_str!("./input.txt");

    let res = input.lines().map(|line: &str| parse_number_from_line(line).unwrap()).sum::<u32>();
    
    println!("{res}");
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

    num_as_string.parse::<u32>().map_err(|error| anyhow::format_err!(error))
}

#[cfg(test)]
mod tests {
    use crate::parse_number_from_line;

    #[test]
    fn should_parse_number_from_line() {
        assert_eq!(parse_number_from_line("m2").unwrap(), 22);
        assert_eq!(parse_number_from_line("two19").unwrap(), 19);
        assert_eq!(parse_number_from_line("424").unwrap(), 44);
        assert_eq!(parse_number_from_line("7txddsmg2hzhz1fourkkthree1").unwrap(), 71);
    }
}