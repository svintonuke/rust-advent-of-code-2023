use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let file_name: &str = "./src/input.txt";

    let mut calibration_value: u32 = 0;
    match read_lines(file_name) {
        Ok(lines) => {
            for line in lines {
                match line {
                    Ok(line) => {
                        calibration_value += recover_two_digit_number(&line);
                    }
                    Err(error) => {
                        println!("Error in line: {}", error);
                    }
                }
            }
            println!("The calibration value is: {}", calibration_value);
        }
        Err(error) => {
            println!("Error in file: {}", error);
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn recover_two_digit_number(input: &str) -> u32 {
    let mut first_number: Option<u32> = None;
    let mut last_number: Option<u32> = None;

    for current_chart in input.chars() {
        if current_chart.is_numeric() {
            if !first_number.is_some() {
                first_number = current_chart.to_digit(10)
            } else {
                last_number = current_chart.to_digit(10)
            }
        }
    }
    first_number.expect("Should have at least one numeric character.");

    if last_number.is_some() {
        return first_number.unwrap() * 10 + last_number.unwrap();
    } else {
        return first_number.unwrap() * 10 + first_number.unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_digits() {
        assert_eq!(recover_two_digit_number("a2a"), 22 as u32);
    }

    #[test]
    fn test_two_digits() {
        assert_eq!(recover_two_digit_number("a2a3a"), 23 as u32);
    }

    #[test]
    fn test_more_digits() {
        assert_eq!(recover_two_digit_number("a2a3a4a5a6a7a"), 27 as u32);
    }

    #[test]
    #[should_panic(expected = "Should have at least one numeric character.")]
    fn test_no_digits() {
        recover_two_digit_number("a");
    }
}