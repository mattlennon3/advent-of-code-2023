fn main() {
    let input = include_str!("../input.txt");    
    let mut total: u32 = 0;

    for one_line in input.lines() {
        let mut first: Option<char> = None;
        let mut last: Option<char> = None;
        // println!("{}", one_line);
        let mut found_first = first.is_some();
        for character in one_line.chars() {
            match character.is_numeric() {
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
    println!("Total: {}", total);

}
