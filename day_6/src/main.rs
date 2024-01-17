use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const IS_SECOND_PART: bool = true;
fn main() {
    let file_name = "./src/input.txt";

    let mut n_line: usize = 0;
    let mut times: Vec<String> = vec![];
    let mut distances: Vec<String> = vec![];
    match read_lines(file_name) {
        Ok(lines) => {
            for line in lines {
                match line {
                    Ok(line) => {
                        let mut splits: Vec<&str> = line.split(' ').collect();
                        splits.retain(|&part| !part.is_empty());
                        if n_line == 0 {
                            times = splits.iter().map(|s| s.to_string()).collect();
                        } else {
                            distances = splits.iter().map(|s| s.to_string()).collect();
                        }
                        n_line += 1;
                    }
                    Err(error) => {
                        println!("Error in line: {}", error);
                    }
                }
            }
            times.remove(0);
            distances.remove(0);
            if IS_SECOND_PART {
                let time: u64 = times.join("").parse().ok().unwrap();
                let distance: u64 = distances.join("").parse().ok().unwrap();
                let race = Race(time, distance);
                let result = number_of_ways_you_can_beat_the_record(&race);
                println!("The result for the second day is:: {}", result);
            } else {
                let mut races: Vec<Race> = vec![];
                for i in 0..times.len() {
                    let time: u64 = times[i].parse().ok().unwrap();
                    let distance: u64 = distances[i].parse().ok().unwrap();
                    races.push(Race(time, distance));
                }
                let mut accumulator: u64 = 1;
                for race in races.iter() {
                    accumulator *= number_of_ways_you_can_beat_the_record(&race);
                }
                println!("The result for the first day is:: {}", accumulator);
            }

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

#[derive(Debug)]
struct Race(u64, u64);

fn miliseconds_holdig_to_win(race: &Race) -> Vec<u64> {
    let mut result = vec![];
    for time_holding in 0..(race.0 + 1) {
        let speed = time_holding;
        let time_left = race.0 - time_holding;
        let distance = speed * time_left;
        if distance > race.1 {
            result.push(time_holding);
        }
    }
    return result
}

fn number_of_ways_you_can_beat_the_record(race: &Race) -> u64 {
    return miliseconds_holdig_to_win(race).len() as u64;
}