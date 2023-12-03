use core::fmt;

/** ###  SCRAPPED ATTEMPT  ### */
// An hour and a half I'll never get back

struct SchematicMap {
    // value, index
    numbers: Vec<(u32, u32)>,
    symbols: Vec<(char, u32)>,
    row_length: u32,
}

impl SchematicMap {
    fn new() -> SchematicMap {
        SchematicMap {
            numbers: Vec::new(),
            symbols: Vec::new(),
            row_length: 0,
        }
    }

    fn get_number_values_for_indexes(&self, indexes: Vec<u32>) -> Vec<u32> {
        let mut values = Vec::new();
        for (number, index) in &self.numbers {
            if indexes.contains(index) {
                values.push(*number);
            }
        }
        values
    }

    fn get_full_number_from_index(&self, base_index: &u32) -> u32 {
        let mut numbers: Vec<u32> = Vec::new();
        // get numbers with index of +1 or -1, until the gap is broken
        // let mut index = *base_index;
        let look_for_next_number = |idx: &u32, ahead: bool| -> Option<&u32> {
            for (num, num_index) in &self.numbers {
                if ahead {
                    if num_index == &(idx + 1) {
                        numbers.push(*idx);
                        // index += 1;
                        return Some(num_index);
                    }
                } else {
                    if num_index == &(idx - 1) {
                        numbers.push(*idx);
                        // index -= 1;
                        return Some(num_index);
                    }
                }
            }
            return None;
        };



            
        // concatenate numbers into string and convert back to u32
        let number_string = numbers.into_iter().map(|n| n.to_string()).collect::<String>();
        number_string.parse::<u32>().unwrap()
    }

    fn get_number_indexes_adjacent_to_symbols(&self) -> Vec<u32> {
        let mut adjacent_number_indexes = Vec::new();
        for (symbol, symbol_index) in &self.symbols {
            // println!("symbol: {} {}", symbol_index, symbol);
            for (number, number_index) in &self.numbers {
                // let symbol_index: i32 = *symbol_index as i32;
                let number_index = *number_index as i32;
                let row_length = self.row_length as i32;
                let number_index_above = number_index - row_length;
                let number_index_below = number_index + row_length;
                let number_index_left = number_index - 1;
                let number_index_right = number_index + 1;
                let number_index_above_left = number_index_above - 1;
                let number_index_above_right = number_index_above + 1;
                let number_index_below_left = number_index_below - 1;
                let number_index_below_right = number_index_below + 1;
                let number_index_adjacent = vec![
                    number_index_above,
                    number_index_below,
                    number_index_left,
                    number_index_right,
                    number_index_above_left,
                    number_index_above_right,
                    number_index_below_left,
                    number_index_below_right,
                ];
                // remove negatives from number_index_adjacent
                let number_index_adjacent: Vec<u32> = number_index_adjacent.into_iter()
                    .filter(|i| *i >= 0)
                    .map(|i| i as u32)
                    .collect();
                
                if let Some(_) = number_index_adjacent.iter().find(|i| i == &symbol_index ) {
                    // println!("number_index_adjacent: {} {} | {} {} | {:?}", symbol_index, symbol, index, number, number_index_adjacent);
                    adjacent_number_indexes.push(number_index as u32);
                }
            }
        }
        adjacent_number_indexes
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let mut index = 0;
    let mut map = SchematicMap::new();

    for row in input.lines() {
        println!("{}", row);
        map.row_length = row.len().try_into().unwrap();
        for c in row.chars() {
            match c {
                '0'..='9' => {
                    map.numbers.push((c.to_digit(10).unwrap(), index));
                },
                '.' => {},
                _ => {
                    map.symbols.push((c, index));
                }
            }
            index += 1;
        }
    }

    let indexes = map.get_number_indexes_adjacent_to_symbols();
    let valid_numbers: Vec<u32> = indexes.iter().map(|index| {
        map.get_full_number_from_index(index)
    }).collect();
    println!("{:?}", map.row_length);
    println!("{:?}", map.symbols);
    println!("{:?}", indexes);
    println!("{:?}", valid_numbers);
    println!("{:?}", map.get_number_values_for_indexes(indexes));

    // println!("{}", map);
}




impl std::fmt::Display for SchematicMap {
    // print numbers and symbols, showing the index
    // print a newline every time the index is a multiple of rowLength
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut index = 0;
        for (number, number_index) in &self.numbers {
            while index < *number_index {
                write!(f, " ")?;
                index += 1;
            }
            write!(f, "{}", number)?;
            index += 1;
        }
        write!(f, "\n")?;
        index = 0;
        for (symbol, symbol_index) in &self.symbols {
            while index < *symbol_index {
                write!(f, " ")?;
                index += 1;
            }
            write!(f, "{}", symbol)?;
            index += 1;
        }
        write!(f, "\n")
    }

}
