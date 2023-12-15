use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let file_name = "./src/input.txt";

    let mut calibration_value: u32 = 0;
    match read_lines(file_name) {
        Ok(lines) => {
            for line in lines {
                match line {
                    Ok(line) => {
                        calibration_value += recover_two_digit_number(string_number_to_string_char_number(line.clone()));
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

fn recover_two_digit_number(string: String) -> u32 {
    let mut first_number: Option<u32> = None;
    let mut last_number: Option<u32> = None;

    for current_chart in string.chars() {
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
mod recover_two_digit_number {
    use super::*;

    #[test]
    fn test_one_digits() {
        assert_eq!(recover_two_digit_number(String::from("a2a")), 22 as u32);
    }

    #[test]
    fn test_two_digits() {
        assert_eq!(recover_two_digit_number(String::from("a2a3a")), 23 as u32);
    }

    #[test]
    fn test_more_digits() {
        assert_eq!(recover_two_digit_number(String::from("a2a3a4a5a6a7a")), 27 as u32);
    }

    #[test]
    #[should_panic(expected = "Should have at least one numeric character.")]
    fn test_no_digits() {
        recover_two_digit_number(String::from("a"));
    }
}

fn string_number_to_string_char_number(string: String) -> String {
    let numbers = [("one", "1"), ("two", "2"), ("three", "3"), ("four", "4"), ("five", "5"), ("six", "6"), ("seven", "7"), ("eight", "8"), ("nine", "9")];

    if string.len() == 0 {
        return String::from("")
    }

    for number in numbers {
        let number_string = number.0;
        let number_string_size = number_string.len();

        if number_string_size > string.len() {
            continue;
        } else if string[0..number_string_size].to_string() == number_string.to_string() {
            return format!("{}{}", number.1.to_string(), string_number_to_string_char_number(string[number_string_size..].to_string()))
        }
    }

    return format!("{}{}", string[0..1].to_string(), string_number_to_string_char_number(string[1..].to_string()))
}

#[cfg(test)]
mod string_number_to_string_char_number {
    use super::*;

    #[test]
    fn test_one_string_number() {
        assert_eq!(string_number_to_string_char_number(String::from("aonea")), String::from("a1a"));
    }

    #[test]
    fn test_two_strings_numbers() {
        assert_eq!(string_number_to_string_char_number(String::from("aoneatwoa")), String::from("a1a2a"));
    }

    #[test]
    fn test_three_strings_numbers() {
        assert_eq!(string_number_to_string_char_number(String::from("aoneatwoathreea")), String::from("a1a2a3a"));
    }

    #[test]
    fn test_empty_string() {
        assert_eq!(string_number_to_string_char_number(String::from("")), String::from(""));
    }
}