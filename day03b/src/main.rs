use regex::Regex;

struct SchematicMap {
    // value, start, end
    numbers: Vec<(u32, (u32, u32))>,
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

    fn get_numbers_adjacent_to_symbols(&self) -> Vec<u32> {
        // number, start index
        let mut adjacent_number_indexes: Vec<(u32, u32)> = Vec::new();
        // Find all indexes adjacent to this symbol
        for (symbol, symbol_index) in &self.symbols {
            // println!("symbol: {} {}", symbol_index, symbol);
            let symbol_index = *symbol_index as i32;
            let row_length = self.row_length as i32;
            let symbol_index_above = symbol_index - row_length;
            let symbol_index_below = symbol_index + row_length;
            let symbol_index_left = symbol_index - 1;
            let symbol_index_right = symbol_index + 1;
            let symbol_index_above_left = symbol_index_above - 1;
            let symbol_index_above_right = symbol_index_above + 1;
            let symbol_index_below_left = symbol_index_below - 1;
            let symbol_index_below_right = symbol_index_below + 1;
            let symbol_index_adjacent = vec![
                symbol_index_above,
                symbol_index_below,
                symbol_index_left,
                symbol_index_right,
                symbol_index_above_left,
                symbol_index_above_right,
                symbol_index_below_left,
                symbol_index_below_right,
            ];

            // remove negatives from symbol_index_adjacent
            let symbol_index_adjacent: Vec<u32> = symbol_index_adjacent.into_iter()
                .filter(|i| *i >= 0)
                .map(|i| i as u32)
                .collect();
            // println!("symbol_index_adjacent: {} {} | {:?}", symbol_index, symbol, symbol_index_adjacent);

            // Find all numbers which fall within this index
            for (number, (start, end)) in &self.numbers {
                for idx in *start..*end {
                    // if number == &467 && symbol == &'*' {
                        // println!("Search: {} {}", number, idx);
                    // }
                    if let Some(_) = symbol_index_adjacent.iter().find(|i| *i == &idx ) {
                        // if the previous number is different, add this number
                        if let Some((_, start_index)) = adjacent_number_indexes.last() {
                            // println!("prev_number: {} number: {}", prev_number, number);
                            if start_index != start {
                                adjacent_number_indexes.push((*number, *start));
                            }
                        }  else {
                            adjacent_number_indexes.push((*number, *start));
                        }
                    }
                }
            }
        
                
        }

        adjacent_number_indexes.iter().map(|(number, _)| *number).collect()
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let mut index = 0;
    let mut map = SchematicMap::new();

    let re = Regex::new(r"\d+").unwrap();

    let mut row_offset: u32 = 0;
    for row in input.lines() {
        map.row_length = row.len().try_into().unwrap();
        // Collect numbers and their indexes
        for mat in re.find_iter(row) {
            // println!("Found number {} at index {} {}", &input[mat.start()..mat.end()], mat.start(), mat.end() - 1);
            map.numbers.push((row[mat.start()..mat.end()].parse::<u32>().unwrap(), ((mat.start() as u32) + row_offset, (mat.end() as u32) + row_offset)));
        }
        row_offset += map.row_length;
        // println!("{}", row);
        for c in row.chars() {
            match c {
                // Collect symbol indexes
                '0'..='9' => {},
                '.' => {},
                _ => {
                    map.symbols.push((c, index));
                }
            }
            index += 1;
        }
    }

    // println!("{:?}", map.numbers);
    // println!("{:?}", map.symbols);
    let result_numbers = map.get_numbers_adjacent_to_symbols();
    // println!("{:?}", result_numbers);
    // sum the numbers
    let result: u32 = result_numbers.into_iter().sum();
    println!("Result: {}", result);
}
