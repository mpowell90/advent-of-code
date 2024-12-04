fn main() {
    let input = include_str!("./input.txt");

    println!("Part 1: {}", part_1(input));
}

fn part_1(input: &str) -> usize {
    let mut valid_opening = 0;
    let mut first_number = String::new();
    let mut first_complete = false;
    let mut second_number = String::new();
    let mut output = 0;

    for ch in input.chars() {
        match ch {
            'm' if valid_opening == 0 => {
                valid_opening += 1;
            }
            'u' if valid_opening == 1 => {
                valid_opening += 1;
            }
            'l' if valid_opening == 2 => {
                valid_opening += 1;
            }
            '(' if valid_opening == 3 => {
                valid_opening += 1;
            }
            ',' if valid_opening == 4 && first_number.len() < 4 => {
                first_complete = true;
            }
            ')' if valid_opening == 4 && first_number.len() < 4 && second_number.len() < 4 => {
                let first_val = first_number.parse::<usize>().unwrap();
                let second_val = second_number.parse::<usize>().unwrap();

                output += first_val * second_val;

                valid_opening = 0;
                first_number.clear();
                second_number.clear();
                first_complete = false;
            }
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' if valid_opening == 4 => {
                if first_complete && second_number.len() < 4 {
                    second_number.push(ch);
                } else if !first_complete && first_number.len() < 4 {
                    first_number.push(ch);
                } else {
                    valid_opening = 0;
                    first_number.clear();
                    second_number.clear();
                    first_complete = false;
                }
            }
            _ => {
                valid_opening = 0;
                first_number.clear();
                second_number.clear();
                first_complete = false;
            }
        }
    }

    output
}
