use std::{collections::HashMap, time::Instant};

enum Directions {
    LEFT,
    RIGHT,
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
struct NetworkMapLookupPtr(*const u8);

struct NetworkNavigation {
    lr_pattern: Vec<Directions>,
    // this hash map should be keys of pointers to each lookup (left column)
    // then values are tuples of the LR direction pointers
    maps: HashMap<NetworkMapLookupPtr, (NetworkMapLookupPtr, NetworkMapLookupPtr)>,
    // maps: Vec<CamelMap>,
}

impl NetworkNavigation {
    fn new(lr_pattern: &str) -> NetworkNavigation {
        NetworkNavigation {
            lr_pattern: lr_pattern
                .chars()
                .map(|char| match char {
                    'L' => Directions::LEFT,
                    'R' => Directions::RIGHT,
                    _ => !panic!("Direction not valid for this dimension!"),
                })
                .collect(),
            maps: HashMap::new(),
        }
    }

    fn build_maps(
        &mut self,
        lookup_map: &HashMap<&str, NetworkMapLookupPtr>,
        direction_map: &Vec<CamelMap>,
    ) {
        // println!("lookup_map {:?}", lookup_map);
        for map in direction_map {
            // println!("left '{}', right '{}'", map.left_str.as_str(), map.right_str.as_str());
            let lookup_ptr = lookup_map.get(map.lookup_str.as_str()).expect("Pointer not found");
            let left_ptr = lookup_map
                .get(map.left_str.as_str())
                .expect("Pointer not found");
            let right_ptr = lookup_map
                .get(map.right_str.as_str())
                .expect("Pointer not found");

            self.maps.insert(*lookup_ptr, (*left_ptr, *right_ptr));
        }
    }

    fn navigate(&self, start_ptr: &NetworkMapLookupPtr, end_ptr: &NetworkMapLookupPtr) -> u32 {
        let mut count = 0;
        let mut current_ptr = start_ptr;
        let mut pattern = self.lr_pattern.iter();

        loop {
            if let Some(direction) = pattern.next() {
                match direction {
                    Directions::LEFT => {
                        current_ptr = &self.maps.get(current_ptr).unwrap().0;
                    },
                    Directions::RIGHT => {
                        current_ptr = &self.maps.get(current_ptr).unwrap().1;
                    }
                }
                count += 1;
            } else {
                // restart the pattern if we run out
                pattern = self.lr_pattern.iter();
            }
            if current_ptr == end_ptr {
                break;
            }
        }
        count
    }
}

struct CamelMap {
    // lookup: String,
    lookup_str: String,
    // left: String,
    left_str: String,
    // right: String,
    right_str: String,
}

fn main() {
    let start = Instant::now();

    let file_input = include_str!("../input.txt");
    let mut lr_str = "";
    let mut maps: Vec<CamelMap> = Vec::new();
    let mut lookup_ptr_map: HashMap<&str, NetworkMapLookupPtr> = HashMap::new();
    let mut lines = file_input.lines();
    while let Some(input) = lines.next() {
        if lr_str == "" {
            lr_str = input;
            continue;
        }
        if input.len() == 0 {
            continue;
        }
        let (lookup_str, directions_str) = input.split_once('=').unwrap();
        let lookup_str = lookup_str.trim();

        let (left_str, right_str) = directions_str.split_once(',').unwrap();
        let left_str = left_str.replace(&['(', ')', ' '][..], "");
        let right_str = right_str.replace(&['(', ')', ' '][..], "");

        lookup_ptr_map.insert(lookup_str, NetworkMapLookupPtr(lookup_str.as_ptr()));

        let camel_map = CamelMap {
            lookup_str: lookup_str.to_string(),
            left_str: left_str.to_string(),
            right_str: right_str.to_string(),
        };

        maps.push(camel_map);
        // Later, lets replace all the LR arms with their respective pointers from the lookup table
    }

    let mut navigation = NetworkNavigation::new(lr_str);
    navigation.build_maps(&lookup_ptr_map, &maps);

    // Start from AAA
    let first_ptr = lookup_ptr_map.get("AAA").unwrap();
    let end_ptr = lookup_ptr_map.get("ZZZ").unwrap();

    let count = navigation.navigate(first_ptr, end_ptr);
    println!("Navigation steps: {}", count);
    let duration = start.elapsed();
    println!("{:?}s", duration.as_secs_f64());
}
