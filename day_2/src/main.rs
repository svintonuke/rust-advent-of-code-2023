use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::fmt;
use std::collections::HashMap;

fn main() {
    let file_name = "./src/input.txt";

    let cubes_in_the_bag = vec![
        CubesOfColor { color: String::from("red"), number: 12 },
        CubesOfColor { color: String::from("green"), number: 13 },
        CubesOfColor { color: String::from("blue"), number: 14 }
    ];

    let mut calibration_value_1: u32 = 0;
    let mut calibration_value_2: u32 = 0;
    let mut current_parsed_line: ParsedLine;
    match read_lines(file_name) {
        Ok(lines) => {
            for line in lines {
                match line {
                    Ok(line) => {
                        current_parsed_line = parse_line(line);
                        calibration_value_1 += get_id_if_possible_or_zero(&current_parsed_line, &cubes_in_the_bag);
                        calibration_value_2 += calculate_power_of_the_minimum_set_of_cubes(&current_parsed_line);
                    }
                    Err(error) => {
                        println!("Error in line: {}", error);
                    }
                }
            }
            println!("The calibration value for day 1 is: {} and for day 2 is: {}", calibration_value_1, calibration_value_2);
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

struct ParsedLine(u32, Vec<SetOfCubes>);
type SetOfCubes = Vec<CubesOfColor>;
struct CubesOfColor {
    color: String,
    number: u32
}

impl fmt::Debug for CubesOfColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "CubesOfColor {{ color: {}, number: {} }}",
            self.color, self.number
        )
    }
}

impl fmt::Debug for ParsedLine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ParsedLine (Id: {}, SetOfCubes: {:?} }})",
            self.0, self.1
        )
    }
}

impl PartialEq for CubesOfColor {
    fn eq(&self, other: &Self) -> bool {
        self.color == other.color && self.number == other.number
    }
}

fn parse_line(string: String) -> ParsedLine {
    let id_and_description_as_plain_text: Vec<&str> = string.splitn(2, ':').collect();

    let id_as_plain_text = id_and_description_as_plain_text[0];
    let id_number_as_plain_text: Vec<&str> = id_as_plain_text.splitn(2, ' ').collect();
    let mut id = 0;
    match id_number_as_plain_text[1].parse::<u32>() {
        Ok(parsed) => {
            id = parsed;
        }
        Err(error) => {
            println!("Error: {}", error);
        }
    };
    if id == 0 { panic!("Invalid Id"); }
    
    let description_as_plain_text = id_and_description_as_plain_text[1];
    let sets_as_plain_text: Vec<&str> = description_as_plain_text.split(';').collect();
    
    let mut sets_of_cubes: Vec<SetOfCubes> = Vec::new();
    for set in sets_as_plain_text {
        let cubes_of_color_as_plain_text: Vec<&str> = set.split(',').collect();
        let cubes_of_color_as_plain_text_without_the_first_space: Vec<&str> = cubes_of_color_as_plain_text.iter()
            .map(|&s| &s[1..])
            .collect();

        let mut cubes_of_color: Vec<CubesOfColor> = Vec::new();
        for color_as_plain_text in cubes_of_color_as_plain_text_without_the_first_space {
            let color: Vec<&str> = color_as_plain_text.split(' ').collect();
            let mut number_of_cubes = 0;

            match color[0].parse::<u32>() {
                Ok(parsed) => {
                    number_of_cubes = parsed;
                }
                Err(error) => {
                    println!("Error: {}", error);
                }
            };
            if number_of_cubes == 0 { panic!("Invalid number of cubes"); }

            cubes_of_color.push(CubesOfColor { color: color[1].to_string(), number: number_of_cubes });
        }
        sets_of_cubes.push(cubes_of_color);
    }

    let result = ParsedLine(id, sets_of_cubes);
    return result
}

#[cfg(test)]
mod parse_line {
    use super::*;

    #[test]
    fn test_id() {
        assert_eq!(parse_line(String::from("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green")).0, 1);
    }

    #[test]
    fn test_id_two_digits() {
        assert_eq!(parse_line(String::from("Game 10: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green")).0, 10);
    }

    #[test]
    #[should_panic(expected = "Invalid Id")]
    fn test_id_invalid() {
        parse_line(String::from("Game AA: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"));
    }

    #[test]
    fn test_one_game_one_color() {
        assert_eq!(parse_line(String::from("Game 1: 3 blue")).1[0][0], CubesOfColor { color: String::from("blue"), number: 3 });
    }

    #[test]
    fn test_one_game_two_color() {
        let first_color = CubesOfColor { color: String::from("blue"), number: 3 };
        let second_color = CubesOfColor { color: String::from("red"), number: 5 };
        assert_eq!(parse_line(String::from("Game 1: 3 blue, 5 red")).1[0], vec![ first_color, second_color ]);
    }

    #[test]
    fn test_two_games_one_color() {
        let first_game = vec![
            CubesOfColor { color: String::from("blue"), number: 3 }
        ];
        let second_game = vec![
            CubesOfColor { color: String::from("green"), number: 4 }
        ];
        assert_eq!(parse_line(String::from("Game 1: 3 blue; 4 green")).1, vec![ first_game, second_game ]);
    }

    #[test]
    fn test_two_games_two_color() {
        let first_game = vec![
            CubesOfColor { color: String::from("blue"), number: 3 },
            CubesOfColor { color: String::from("red"), number: 5 }
        ];
        let second_game = vec![
            CubesOfColor { color: String::from("green"), number: 4 },
            CubesOfColor { color: String::from("blue"), number: 1 }
        ];
        assert_eq!(parse_line(String::from("Game 1: 3 blue, 5 red; 4 green, 1 blue")).1, vec![ first_game, second_game ]);
    }

    #[test]
    fn test_three_games_three_color() {
        let first_game = vec![
            CubesOfColor { color: String::from("blue"), number: 1 },
            CubesOfColor { color: String::from("green"), number: 2 }
        ];
        let second_game = vec![
            CubesOfColor { color: String::from("green"), number: 3 },
            CubesOfColor { color: String::from("blue"), number: 4 },
            CubesOfColor { color: String::from("red"), number: 1 }
        ];
        let third_game = vec![
            CubesOfColor { color: String::from("green"), number: 1 },
            CubesOfColor { color: String::from("blue"), number: 1 }
        ];
        assert_eq!(parse_line(String::from("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue")).1, vec![ first_game, second_game, third_game ]);
    }
}

fn get_id_if_possible_or_zero(line: &ParsedLine, cubes_in_the_bag: &Vec<CubesOfColor>) -> u32 {
    for set_of_cubes in &line.1 {
        for cube in set_of_cubes {
            for posibble_cube in cubes_in_the_bag {
                if cube.color == posibble_cube.color {
                    if cube.number > posibble_cube.number {
                        return 0
                    }
                }
                
            }
        }
    }
    return line.0
}

#[cfg(test)]
mod get_id_if_possible_or_zero {
    use super::*;

    #[test]
    fn test_posibble_game() {
        let cubes_in_the_bag = vec![
            CubesOfColor { color: String::from("red"), number: 12 },
            CubesOfColor { color: String::from("green"), number: 13 },
            CubesOfColor { color: String::from("blue"), number: 14 }
        ];
        let parsed_line = ParsedLine (1, vec![
            vec![
                CubesOfColor { color: String::from("blue"), number: 3 }
            ]
        ]);
        assert_eq!(get_id_if_possible_or_zero(&parsed_line, &cubes_in_the_bag), 1);
    }

    #[test]
    fn test_imposibble_game() {
        let cubes_in_the_bag = vec![
            CubesOfColor { color: String::from("red"), number: 12 },
            CubesOfColor { color: String::from("green"), number: 13 },
            CubesOfColor { color: String::from("blue"), number: 14 }
        ];
        let parsed_line = ParsedLine (3, vec![vec![CubesOfColor { color: String::from("red"), number: 20 }]]);
        assert_eq!(get_id_if_possible_or_zero(&parsed_line, &cubes_in_the_bag), 0);
    }
}

fn calculate_power_of_the_minimum_set_of_cubes(line: &ParsedLine) -> u32 {
    let mut minimum_set_of_cubes: HashMap<String, u32> = HashMap::new();

    for set_of_cubes in &line.1 {
        for cube in set_of_cubes {
            if let Some(current_min) = minimum_set_of_cubes.get_mut(&cube.color) {
                if cube.number > *current_min {
                    *current_min = cube.number;
                }
            } else {
                minimum_set_of_cubes.insert(String::from(cube.color.clone()), cube.number);
            }
        }
    }

    let mut power = 1;
    for (_color, number) in &minimum_set_of_cubes {
        power *= number;
    }
    return power
}

#[cfg(test)]
mod calculate_power_of_the_minimum_set_of_cubes {
    use super::*;

    #[test]
    fn test_posibble_game() {
        let parsed_line = ParsedLine (1, vec![
            vec![
                CubesOfColor { color: String::from("blue"), number: 3 },
                CubesOfColor { color: String::from("red"), number: 4 }
            ],
            vec![
                CubesOfColor { color: String::from("red"), number: 1 },
                CubesOfColor { color: String::from("green"), number: 2 },
                CubesOfColor { color: String::from("blue"), number: 6 }
            ],
            vec![
                CubesOfColor { color: String::from("green"), number: 2 }
            ]
        ]);
        assert_eq!(calculate_power_of_the_minimum_set_of_cubes(&parsed_line), 48);
    }

}