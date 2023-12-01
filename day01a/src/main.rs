use std::{collections::HashMap, ops::IndexMut};

fn get_number_hashmap() -> HashMap<&'static str, &'static str> {
    let mut numbers: HashMap<_, _> = HashMap::new();

    numbers.insert("one", "1");
    numbers.insert("two", "2");
    numbers.insert("three", "3");
    numbers.insert("four", "4");
    numbers.insert("five", "5");
    numbers.insert("six", "6");
    numbers.insert("seven", "7");
    numbers.insert("eight", "8");
    numbers.insert("nine", "9");

    return numbers;
}

fn main() {
    let input = include_str!("../input.txt");    
    let mut total: u32 = 0;

    let numbers = get_number_hashmap();

    for immutable_line in input.lines() {
        let mut mutable_line = immutable_line.to_string();
        let mut first: Option<char> = None;
        let mut last: Option<char> = None;
        
        // Take each number word and find it's index in the line
        // Add the index and the number word to a vector
        let mut word_indexes: Vec<(usize, &str)> = Vec::new();

        // Naively do this 4 times, incase words are repeated
        for _ in 0..4 {
            numbers.iter().for_each(|(key, value)| {
                let index = mutable_line.find(key);
                match index {
                    Some(i) => word_indexes.push((i, value)),
                    _ => {}
                }
            });
            // take the index of each number (AKA first character of the number word)
            // replace it with the number it corresponds to
            word_indexes.iter().for_each(|(index, value)| {
                mutable_line.replace_range(index..&(index+1), value)
            });
            // println!("{}", mutable_line);
        }

        let mut found_first = first.is_some();
        for character in mutable_line.chars() {
            match character.is_numeric() {
                // if the line has 1 number, then the first and last will be the same
                true => {
                    if first.is_none() {
                        first = Some(character);
                        last = Some(character);
                        found_first = true;
                    } else if found_first {
                        last = Some(character);
                    }
                },
                _ => {}
            }
            // print!("{}", character);
        }
        total += format!("{}{}", first.unwrap(), last.unwrap()).parse::<u32>().unwrap();
    }
    //p2 example: 281
    println!("Total: {}", total);

}
