use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let file_name = "./src/input.txt";

    let mut matriz = Matriz::new(140, 140);
    let mut n_row: usize = 0;
    match read_lines(file_name) {
        Ok(lines) => {
            for line in lines {
                match line {
                    Ok(line) => {
                        store_line_in_matriz(line.clone(), n_row, &mut matriz);
                        n_row += 1;
                    }
                    Err(error) => {
                        println!("Error in line: {}", error);
                    }
                }
            }
            let mut calibration_value_1: u32 = 0;
            let mut calibration_value_2: u32 = 0;
            for n_row in 0..matriz.size().0 {
                calibration_value_1 += get_sum_of_number_adjacent_to_a_symbol(n_row, &mut matriz);
                calibration_value_2 += get_sum_of_gear_ratio(n_row, &mut matriz);
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

fn store_line_in_matriz(line: String, n_row: usize, matriz: &mut Matriz) -> () {
    let mut n_col: usize = 0;
    for current_char in line.chars() {
        matriz.store_at((n_row, n_col), current_char);
        n_col += 1;
    }
}

fn get_sum_of_number_adjacent_to_a_symbol(n_row: usize, matriz: &mut Matriz) -> u32 {
    let mut sum_of_valid_nums = 0;
    let mut n_col = 0;
    while n_col < matriz.size().1 {
        if matriz.get_at((n_row, n_col)).is_numeric() {
            let mut current_number_position = n_col;
            let mut string_number = String::from("");
            let mut number_is_valid = false;
            while current_number_position < matriz.size().1 && matriz.get_at((n_row, current_number_position)).is_numeric() {
                if matriz.get_adjacent_at((n_row, current_number_position)).iter().any(|&c| is_symbol(c)) {
                    number_is_valid = true;
                }
                string_number.push(matriz.get_at((n_row, current_number_position)));
                current_number_position += 1;
            }
            if number_is_valid {
                sum_of_valid_nums += parse_number(string_number);
            }
            n_col = current_number_position;
        }
        n_col += 1;
    }
    return sum_of_valid_nums
}

fn get_sum_of_gear_ratio(n_row: usize, matriz: &mut Matriz) -> u32 {
    let mut sum_of_gear_ratio = 0;
    for n_col in 0..matriz.size().1 {
        if matriz.get_at((n_row, n_col)) == '*' {
            let adjacents = matriz.get_adjacent_at((n_row, n_col));
            if adjacent_to_at_least_two_char_numbers(adjacents.clone()) {
                //println!("In Coord in Visual {:?}", (n_row+1, n_col+1));
                let coords_adjacents = matriz.get_coords_adjacent_at((n_row, n_col));
                let mut adjacents_position_and_full_number: Vec<((usize, usize), u32)> = vec![];
                for (index, adjacent) in adjacents.iter().enumerate() {
                    if adjacent.is_numeric() {
                        //println!("{:?}", adjacents);
                        //println!("{:?}", coords_adjacents[index]);
                        let number_row = n_row as i32 + coords_adjacents[index].0;
                        let number_col = n_col as i32 + coords_adjacents[index].1;
                        let position_and_number = get_position_and_full_number_at((number_row as usize, number_col as usize), matriz);
                        if !adjacents_position_and_full_number.contains(&position_and_number) {
                            adjacents_position_and_full_number.push(position_and_number);
                        }
                    }
                }
                if adjacents_position_and_full_number.len() == 2 {
                    //println!("{:?}", adjacents_position_and_full_number);
                    sum_of_gear_ratio += adjacents_position_and_full_number[0].1 * adjacents_position_and_full_number[1].1;
                }
            }
        }
    }
    return sum_of_gear_ratio
}

//363

fn is_symbol(c: char) -> bool {
    return !c.is_numeric() && c != '.';
}

fn adjacent_to_at_least_two_char_numbers(adjacents: Vec<char>) -> bool {
    let mut numbers_adjacents = 0;
    for a in adjacents.clone() {
        if a.is_numeric() {
            numbers_adjacents += 1;
        }
    }
    return numbers_adjacents >= 2;
}

fn parse_number(string_number: String) -> u32 {
    match string_number.parse::<u32>() {
        Ok(num) => {
            return num;
        }
        Err(_) => {
            panic!("Should be a number");
        }
    }
}

fn get_position_and_full_number_at(pos: (usize, usize), matriz: &mut Matriz) -> ((usize, usize), u32) {
    let mut index_pos = pos.clone();
    while index_pos.1 != 0 && matriz.get_at((index_pos.0, index_pos.1 - 1)).is_numeric() {
        index_pos.1 -= 1;
    }
    let mut full_number = String::from("");
    let mut i = index_pos.1;
    while i < matriz.size().1 && matriz.get_at((index_pos.0, i)).is_numeric() {
        full_number.push(matriz.get_at((index_pos.0, i)));
        i += 1;
    }
    //println!("{:?}", full_number);
    return (index_pos, parse_number(full_number));
}





struct Matriz {
    size: (usize, usize),
    data: Vec<Vec<char>>
}

impl Matriz {
    fn new(max_row: usize, max_col: usize) -> Self {
        let mut data = vec![];
        for _x in 0..max_row {
            let mut row = vec![];
            for _y in 0..max_col {
                row.push(' ');
            }
            data.push(row);
        }

        Matriz {
            size: (max_row, max_col),
            data
        }
    }

    fn size(&self) -> (usize, usize) {
        return self.size;
    }

    fn get_at(&self, pos: (usize, usize)) -> char {
        self.panic_if_out_of_range(pos);

        return self.data[pos.0][pos.1];
    }

    fn store_at(&mut self, pos: (usize, usize), data: char) -> bool {
        self.panic_if_out_of_range(pos);

        self.data[pos.0][pos.1] = data;
        return true;
    }

    fn is_on_range(&self, pos: (usize, usize)) -> bool {
        if pos.0 >= self.size.0 || pos.1 >= self.size.1  {
            return false
        }
        return true;
    }

    fn panic_if_out_of_range(&self, pos: (usize, usize)) -> () {
        if pos.0 >= self.size.0 {
            panic!("index row out of bounds: the len is {} but the index is {}", self.size.0 + 1, pos.0)
        }
        if pos.1 >= self.size.1 {
            panic!("index column out of bounds: the len is {} but the index is {}", self.size.1 + 1, pos.1)
        }
    }

    fn get_adjacent_at(&self, pos: (usize, usize)) -> Vec<char> {
        const ADJACENT_COORDS: [(i32, i32); 8] = [
            (-1,-1),(-1,0),(-1,1),
            ( 0,-1),       ( 0,1),
            ( 1,-1),( 1,0),( 1,1)
        ];
        let mut adjacents = vec![];
        for coord in &ADJACENT_COORDS {
            let new_x = pos.0 as i32 + coord.0;
            let new_y = pos.1 as i32 + coord.1;

            if new_x >= 0 && new_y >= 0 {
                let new_pos = (new_x as usize, new_y as usize);
                if self.is_on_range(new_pos) {
                    adjacents.push(self.get_at(new_pos));
                }
            }
        }
        return adjacents
    }

    fn get_coords_adjacent_at(&self, pos: (usize, usize)) -> Vec<(i32, i32)> {
        const ADJACENT_COORDS: [(i32, i32); 8] = [
            (-1,-1),(-1,0),(-1,1),
            ( 0,-1),       ( 0,1),
            ( 1,-1),( 1,0),( 1,1)
        ];
        let mut adjacents = vec![];
        for coord in &ADJACENT_COORDS {
            let new_x = pos.0 as i32 + coord.0;
            let new_y = pos.1 as i32 + coord.1;

            if new_x >= 0 && new_y >= 0 {
                let new_pos = (new_x as usize, new_y as usize);
                if self.is_on_range(new_pos) {
                    adjacents.push(coord.clone());
                }
            }
        }
        return adjacents
    }
}


#[cfg(test)]
mod test_matriz {
    use super::*;

    #[test]
    fn test_size() {
        let my_matriz = Matriz::new(3, 5);
        assert_eq!(my_matriz.size(), (3, 5));
    }

    #[test]
    fn test_get_at() {
        let my_matriz = Matriz::new(3, 5);
        assert_eq!(my_matriz.get_at((1, 0)), ' ');
    }

    #[test]
    fn test_store_at() {
        let mut my_matriz = Matriz::new(3, 5);
        my_matriz.store_at((1, 0), 'A');
        assert_eq!(my_matriz.get_at((1, 0)), 'A');
    }

    #[test]
    fn test_store_at_other_value() {
        let mut my_matriz = Matriz::new(3, 5);
        my_matriz.store_at((1, 0), 'A');
        my_matriz.store_at((2, 3), 'B');
        assert_eq!(my_matriz.get_at((1, 0)), 'A');
        assert_eq!(my_matriz.get_at((2, 3)), 'B');
    }

    #[test]
    #[should_panic(expected = "index row out of bounds: the len is 4 but the index is 3")]
    fn test_out_of_range_row() {
        let mut my_matriz = Matriz::new(3, 5);
        my_matriz.store_at((3, 4), 'A');
    }

    #[test]
    #[should_panic(expected = "index column out of bounds: the len is 6 but the index is 5")]
    fn test_out_of_range_column() {
        let mut my_matriz = Matriz::new(3, 5);
        my_matriz.store_at((2, 5), 'A');
    }

    #[test]
    fn test_get_adjacent_at() {
        /*
                .....
                .123.
                .4x5.
                .678.
                .....
        */
        let mut my_matriz = Matriz::new(5, 5);
        my_matriz.store_at((0, 0), '.'); my_matriz.store_at((0, 1), '.'); my_matriz.store_at((0, 2), '.'); my_matriz.store_at((0, 3), '.'); my_matriz.store_at((0, 4), '.');
        my_matriz.store_at((1, 0), '.'); my_matriz.store_at((1, 1), '1'); my_matriz.store_at((1, 2), '2'); my_matriz.store_at((1, 3), '3'); my_matriz.store_at((1, 4), '.');
        my_matriz.store_at((2, 0), '.'); my_matriz.store_at((2, 1), '4'); my_matriz.store_at((2, 2), 'x'); my_matriz.store_at((2, 3), '5'); my_matriz.store_at((2, 4), '.');
        my_matriz.store_at((3, 0), '.'); my_matriz.store_at((3, 1), '6'); my_matriz.store_at((3, 2), '7'); my_matriz.store_at((3, 3), '8'); my_matriz.store_at((3, 4), '.');
        my_matriz.store_at((4, 0), '.'); my_matriz.store_at((4, 1), '.'); my_matriz.store_at((4, 2), '.'); my_matriz.store_at((4, 3), '.'); my_matriz.store_at((4, 4), '.');

        assert_eq!(my_matriz.get_adjacent_at((2, 2)), vec!['1', '2', '3', '4', '5', '6', '7', '8']);
    }

    #[test]
    fn test_get_adjacent_at_border() {
        /*
                4x5
        */
        let mut my_matriz = Matriz::new(1, 3);
        my_matriz.store_at((0, 0), '4'); my_matriz.store_at((0, 1), 'x'); my_matriz.store_at((0, 2), '5');

        assert_eq!(my_matriz.get_adjacent_at((0, 1)), vec!['4', '5']);
    }

    #[test]
    fn test_get_coords_adjacent_at() {
        /*
                .....
                .123.
                .4x5.
                .678.
                .....
        */
        let mut my_matriz = Matriz::new(5, 5);
        assert_eq!(my_matriz.get_coords_adjacent_at((2, 2)), vec![(-1,-1), (-1,0), (-1,1), (0,-1), (0,1), (1,-1), (1,0), (1,1)]);
    }

    #[test]
    fn test_get_coords_adjacent_at_border() {
        /*
                4x5
        */
        let mut my_matriz = Matriz::new(1, 3);
        assert_eq!(my_matriz.get_coords_adjacent_at((0, 1)), vec![(0,-1), (0,1)]);
    }
}