use regex::Regex;

/** Parse a line of numbers, may return None */
pub fn numerical_parse<N>(input: &str) -> Option<Vec<N>> where N: std::str::FromStr, <N as std::str::FromStr>::Err: std::fmt::Debug {
    let mut result: Vec<N> = Vec::new();
    let digits_regex = Regex::new(r"\d+").unwrap();
    for mat in digits_regex.find_iter(input) {
        result.push(mat.as_str().parse::<N>().unwrap());
    }
    if result.len() == 0 {
        return None;
    }
    Some(result)
}

pub fn line_includes(input: &str, search: &str) -> bool {
    input.contains(search)
}

// pub fn is_numerical_line(input: &str) -> bool {
//     let digits_regex = Regex::new(r"\d+").unwrap();
//     digits_regex. .is_match(input)
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
