use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const N_SEEDS: usize = 20;
const IS_SECOND_PART: bool = false;
fn main() {
    let file_name = "./src/input.txt";

    match read_lines(file_name) {
        Ok(lines) => {
            let mut n_line: u64 = 0;
            
            let mut seeds: Vec<u64> = vec![];
            let mut categories: Vec<CategoryMap> = vec![];

            let mut source = String::from("");
            let mut destination = String::from("");
            let mut descriptions = vec![];

            for line in lines {
                match line {
                    Ok(line) => {
                        if n_line == 0 {
                            seeds = (line[7..].to_string())
                                        .splitn(N_SEEDS, " ")
                                        .collect::<Vec<&str>>()
                                        .iter()
                                        .filter_map(|&x| x.parse().ok())
                                        .collect();
                        } else if n_line == 1 {
                            
                        } else if line.contains("-to-") {
                            //line[0..(line.len() - 6)];
                            source = String::from("");
                            destination = String::from("");
                        } else if line.is_empty() {
                            categories.push(CategoryMap::new(source, destination, descriptions));
                            source = String::from("");
                            destination = String::from("");
                            descriptions = vec![];
                        } else {
                            descriptions.push(line);
                        }
                    }
                    Err(error) => {
                        println!("Error in line: {}", error);
                    }
                }
                n_line += 1;
            }
            categories.push(CategoryMap::new(source, destination, descriptions));

            if IS_SECOND_PART {
                let mut new_seeds: Vec<(u64, u64)> = vec![];
                for i in (0..seeds.len()).step_by(2) {
                    let start = seeds[i];
                    let length = seeds[i + 1];
                    new_seeds.push((start, length));
                }

                let mut location: u64 = 6603921;
                let mut lowest_location: u64 = 0;
                loop {
                    println!("Trying location: {}", location);
                    for category in categories.iter().rev() {
                        lowest_location = category.source_result(location);
                    }
                    println!("      source required is: {}", lowest_location);
                    if seed_includes(&new_seeds, lowest_location) {
                        break;
                    }
                    location += 1;
                }
                println!("The lowest location number that corresponds to any of the initial seeds is: {}", lowest_location);
            } else {
                let mut locations: Vec<u64> = vec![];
                //let harcoded_seeds = vec![79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66 ,67];
                for seed in seeds.iter() {
                    let mut location = *seed;
                    print!("{}", location);
                    for category in categories.iter() {
                        location = category.result(location);
                        print!(" -> {}", location);
                    }
                    println!("");
                    println!(" - {} - ", location);
                    locations.push(location);
                }

                let lowest_location: u64 = *locations.iter().min().unwrap();
                println!("The lowest location number that corresponds to any of the initial seeds is: {}", lowest_location);
            }

        }
        Err(error) => {
            println!("Error in file: {}", error);
        }
    }
}

fn seed_includes(seeds: &Vec<(u64, u64)>, seed: u64) -> bool {
    for seed_group in seeds.iter() {
        if (seed_group.0 <= seed) && (seed <= (seed_group.0 + seed_group.1 - 1)) {
            return true
        }
    }
    return false
}

#[cfg(test)]
mod test_seed_includes {
    use super::*;

    #[test]
    fn test_seed_includes() {
        assert_eq!(seed_includes(&vec![(79 as u64, 14 as u64), (55 as u64, 13 as u64)], 78), false);
        assert_eq!(seed_includes(&vec![(79 as u64, 14 as u64), (55 as u64, 13 as u64)], 79), true);
        assert_eq!(seed_includes(&vec![(79 as u64, 14 as u64), (55 as u64, 13 as u64)], 92), true);
        assert_eq!(seed_includes(&vec![(79 as u64, 14 as u64), (55 as u64, 13 as u64)], 93), false);

        assert_eq!(seed_includes(&vec![(79 as u64, 14 as u64), (55 as u64, 13 as u64)], 54), false);
        assert_eq!(seed_includes(&vec![(79 as u64, 14 as u64), (55 as u64, 13 as u64)], 55), true);
        assert_eq!(seed_includes(&vec![(79 as u64, 14 as u64), (55 as u64, 13 as u64)], 67), true);
        assert_eq!(seed_includes(&vec![(79 as u64, 14 as u64), (55 as u64, 13 as u64)], 68), false);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[warn(dead_code)]
#[warn(unused_variables)]
#[derive(Debug)]
struct CategoryMap {
    source: String,
    destination: String,
    maps: Vec<InternalMap>
}

impl CategoryMap {
    fn new(source: String, destination: String, descriptions: Vec<String>) -> Self {
        let mut maps: Vec<InternalMap> = vec![];
        for description in descriptions.iter() {
            maps.push(InternalMap::new(description.to_string()));
        }

        CategoryMap {
            source,
            destination,
            maps
        }
    }

    fn result(&self, source: u64) -> u64 {
        for map in self.maps.iter() {
            if map.is_on_range(source) {
                return map.result(source)
            }
        }
        return source
    }

    fn source_result(&self, destination: u64) -> u64 {
        for map in self.maps.iter() {
            if map.source_is_on_range(destination) {
                return map.result_source(destination)
            }
        }
        return destination
    }
}

#[cfg(test)]
mod test_category_map {
    use super::*;

    fn initialize_seed_to_soil_map() -> CategoryMap {
        let source = String::from("seed");
        let destination = String::from("soil");
        let description = vec![
            String::from("50 98 2"),
            String::from("52 50 48")
        ];
        return CategoryMap::new(source, destination, description);
    }

    #[test]
    fn test_no_mapped() {
        let my_seed_to_soil_map = initialize_seed_to_soil_map();
        assert_eq!(my_seed_to_soil_map.result(0), 0);
    }

    #[test]
    fn test_mapped_fisrt_map_a() {
        let my_seed_to_soil_map = initialize_seed_to_soil_map();
        assert_eq!(my_seed_to_soil_map.result(98), 50);
    }

    #[test]
    fn test_mapped_fisrt_map_b() {
        let my_seed_to_soil_map = initialize_seed_to_soil_map();
        assert_eq!(my_seed_to_soil_map.result(99), 51);
    }

    #[test]
    fn test_mapped_second_map_a() {
        let my_seed_to_soil_map = initialize_seed_to_soil_map();
        assert_eq!(my_seed_to_soil_map.result(50), 52);
    }

    #[test]
    fn test_mapped_second_map_b() {
        let my_seed_to_soil_map = initialize_seed_to_soil_map();
        assert_eq!(my_seed_to_soil_map.result(51), 53);
    }
}

#[derive(Debug)]
struct InternalMap {
    source_range_start: u64,
    destination_range_start: u64,
    range_length: u64
}

impl InternalMap {
    fn new(description: String) -> Self {
        let splited_description: Vec<u64> = description.splitn(3, ' ')
        .collect::<Vec<&str>>()
        .iter()
        .filter_map(|&x| x.parse().ok())
        .collect();

        if splited_description.len() < 3 {
            print!("{:?}", splited_description);
            panic!("invalid description");
        }

        InternalMap {
            destination_range_start: splited_description[0],
            source_range_start: splited_description[1],
            range_length: splited_description[2]
        }
    }

    fn is_on_range(&self, source: u64) -> bool {
        return !((source < self.source_range_start) | (self.source_range_start + self.range_length - 1 < source))
    }

    fn result(&self, source: u64) -> u64 {
        if (self.is_on_range(source)) {
            return self.destination_range_start + (source - self.source_range_start); 
        } else {
            panic!("out of range");
        }
    }

    fn source_is_on_range(&self, destination: u64) -> bool {
        return !((destination < self.destination_range_start) | (self.destination_range_start + self.range_length - 1 < destination))
    }

    fn result_source(&self, destination: u64) -> u64 {
        if (self.source_is_on_range(destination)) {
            return self.source_range_start + (destination - self.destination_range_start); 
        } else {
            panic!("out of range");
        }
    }
}

#[cfg(test)]
mod test_internal_map {
    use super::*;

    fn initialize_seed_to_soil_internal_map() -> InternalMap {
        return InternalMap::new(String::from("50 98 2"));
    }

    #[test]
    fn test_mapped_first() {
        let my_seed_to_soil_map = initialize_seed_to_soil_internal_map();
        assert_eq!(my_seed_to_soil_map.result(98), 50);
    }

    #[test]
    fn test_mapped_second() {
        let my_seed_to_soil_map = initialize_seed_to_soil_internal_map();
        assert_eq!(my_seed_to_soil_map.result(99), 51);
    }

    #[test]
    #[should_panic(expected = "out of range")]
    fn test_mapped_out_of_range() {
        let my_seed_to_soil_map = initialize_seed_to_soil_internal_map();
        my_seed_to_soil_map.result(1);
    }

    #[test]
    fn test_is_not_on_range_low() {
        let my_seed_to_soil_map = initialize_seed_to_soil_internal_map();
        assert_eq!(my_seed_to_soil_map.is_on_range(97), false);
    }

    #[test]
    fn test_is_on_range() {
        let my_seed_to_soil_map = initialize_seed_to_soil_internal_map();
        assert_eq!(my_seed_to_soil_map.is_on_range(98), true);
    }

    #[test]
    fn test_is_not_on_range_greater() {
        let my_seed_to_soil_map = initialize_seed_to_soil_internal_map();
        assert_eq!(my_seed_to_soil_map.is_on_range(100), false);
    }

    #[test]
    #[should_panic(expected = "invalid description")]
    fn test_invalid_description_empty_string() {
        InternalMap::new(String::from(""));
    }

    #[test]
    #[should_panic(expected = "invalid description")]
    fn test_invalid_description() {
        InternalMap::new(String::from("01 01"));
    }

    #[test]
    fn test_mapped_source_first() {
        let my_seed_to_soil_map = initialize_seed_to_soil_internal_map();
        assert_eq!(my_seed_to_soil_map.result_source(50), 98);
    }

    #[test]
    fn test_mapped_source_second() {
        let my_seed_to_soil_map = initialize_seed_to_soil_internal_map();
        assert_eq!(my_seed_to_soil_map.result_source(51), 99);
    }

    #[test]
    #[should_panic(expected = "out of range")]
    fn test_mapped_source_out_of_range() {
        let my_seed_to_soil_map = initialize_seed_to_soil_internal_map();
        my_seed_to_soil_map.result_source(1);
    }

    #[test]
    fn test_source_is_not_on_range_low() {
        let my_seed_to_soil_map = initialize_seed_to_soil_internal_map();
        assert_eq!(my_seed_to_soil_map.source_is_on_range(49), false);
    }

    #[test]
    fn test_source_is_on_range() {
        let my_seed_to_soil_map = initialize_seed_to_soil_internal_map();
        assert_eq!(my_seed_to_soil_map.source_is_on_range(50), true);
    }

    #[test]
    fn test_source_is_not_on_range_greater() {
        let my_seed_to_soil_map = initialize_seed_to_soil_internal_map();
        assert_eq!(my_seed_to_soil_map.source_is_on_range(52), false);
    }
}
