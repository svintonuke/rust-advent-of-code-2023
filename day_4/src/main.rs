use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn main() {
    let file_name = "./src/input.txt";

    match read_lines(file_name) {
        Ok(lines) => {
            let mut cards: Vec<Card> = vec![];
            for line in lines {
                match line {
                    Ok(line) => {
                        cards.push(Card::new(line.clone()));
                    }
                    Err(error) => {
                        println!("Error in line: {}", error);
                    }
                }
            }
            let mut calibration_value_1: u32 = 0;
            let mut calibration_value_2: u32 = 0;
            let mut cache: HashMap<u32, u32> = HashMap::new();
            for card in cards.iter().rev() {
                //println!("Current Card: {}", card.id);
                calibration_value_1 += card.points();
                calibration_value_2 += card.recursive_n_cards_by_points(&cards, &mut cache);
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

struct Card {
    id: u32,
    winning_numbers: Vec<u32>,
    numbers_you_have: Vec<u32>
}

impl Card {
    fn new(description: String) -> Self {
        let id_and_numbers: Vec<&str> = description.splitn(2, ':').collect();
        let id = id_and_numbers[0][5..].replace(" ", "").parse().ok().unwrap_or_else(|| {
            panic!("Id should be a number");
        });
        let numbers = id_and_numbers[1];

        let winning_numbers_and_numbers_you_have: Vec<&str> = numbers.splitn(2, '|').collect();

        let winning_numbers: Vec<u32> = winning_numbers_and_numbers_you_have[0].split(' ')
            .collect::<Vec<&str>>()
            .iter()
            .filter(|&&x| x != "")
            .filter_map(|&x| x.parse().ok())
            .collect();

        let numbers_you_have: Vec<u32> = winning_numbers_and_numbers_you_have[1].split(' ')
            .collect::<Vec<&str>>()
            .iter()
            .filter(|&&x| x != "")
            .filter_map(|&x| x.parse().ok())
            .collect();

        Card {
            id,
            winning_numbers,
            numbers_you_have
        }
    }

    fn points(&self) -> u32 {
        let mut points = 0;
        for number in self.numbers_you_have.iter() {
            if self.winning_numbers.contains(number) {
                if points == 0 {
                    points = 1;
                } else {
                    points *= 2;
                }
            }
        }
        return points
    }

    fn lineal_points(&self) -> u32 {
        let mut points = 0;
        for number in self.numbers_you_have.iter() {
            if self.winning_numbers.contains(number) {
                points += 1;
            }
        }
        return points
    }

    fn recursive_n_cards_by_points(&self, cards: &Vec<Card>, cache: &mut HashMap<u32, u32>) -> u32 {
        if cache.contains_key(&self.id) {
            match cache.get(&self.id) {
                Some(ncards) => {
                    return *ncards;
                }
                None => {

                }
            }
        }
        let mut ncards: u32 = 1;

        for i in (self.id)..(self.id + self.lineal_points()) {
            ncards += cards[i as usize].recursive_n_cards_by_points(cards, cache);
        }
        cache.insert(self.id, ncards);
        return ncards;
    }

}

impl Clone for Card {
    fn clone(&self) -> Card {
        Card {
            id: self.id,
            winning_numbers: self.winning_numbers.clone(),
            numbers_you_have: self.numbers_you_have.clone(),
        }
    }
}

#[cfg(test)]
mod test_card {
    use super::*;

    #[test]
    fn test_card_new() {
        let my_card = Card::new(String::from("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"));
        assert_eq!(my_card.id, 1);
        assert_eq!(my_card.winning_numbers, vec![41, 48, 83, 86, 17]);
        assert_eq!(my_card.numbers_you_have, vec![83, 86, 6, 31, 17, 9, 48, 53]);
    }

    #[test]
    #[should_panic(expected = "Id should be a number")]
    fn test_card_new_from_invalid_id() {
        Card::new(String::from("Card A: 1 | 1"));
    }

    #[test]
    fn test_points() {
        let my_card = Card::new(String::from("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"));
        assert_eq!(my_card.points(), 8);
    }

    #[test]
    fn test_points_2() {
        let my_card = Card::new(String::from("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19"));
        assert_eq!(my_card.points(), 2);
    }

    #[test]
    fn test_points_no_points() {
        let my_card = Card::new(String::from("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36"));
        assert_eq!(my_card.points(), 0);
    }

    #[test]
    fn test_sum_recursive_n_cards_by_points() {
        let mut sum_of_points = 0;
        let mut cache: HashMap<u32, u32> = HashMap::new();
        let my_cards = vec![
            Card::new(String::from("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53")),
            Card::new(String::from("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19")),
            Card::new(String::from("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1")),
            Card::new(String::from("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83")),
            Card::new(String::from("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36")),
            Card::new(String::from("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11")),
        ];
        for card in my_cards.iter() {
            sum_of_points += card .recursive_n_cards_by_points(&my_cards, &mut cache);
        }
        assert_eq!(sum_of_points, 30);
    }
}