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

    fn navigate(&self, start_ptr: &NetworkMapLookupPtr, end_ptrs: Vec<&NetworkMapLookupPtr>) -> u32 {
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
            if end_ptrs.contains(&current_ptr) {
                break;
            }
        }
        count
    }

    fn get_shared_lcm(&self, counts: Vec<u32>) -> u32 {
        // lcm(a,b,c) = lcm(a,lcm(b,c))
    }

    /**
     * Run through the navigation, navigating each start_ptr at the same time.
     * If there is a moment when all current ptrs are in the set of end_ptrs, return steps
     */
    fn group_navigate(&self, start_ptrs: Vec<&NetworkMapLookupPtr>, end_ptrs: Vec<&NetworkMapLookupPtr>) -> u32 {
        let mut count = 0;
        let mut current_ptrs = start_ptrs;
        let mut pattern = self.lr_pattern.iter();
        
        loop {
            if let Some(direction) = pattern.next() {
                
                let mut routes: Vec<Box<dyn FnMut()>> = Vec::new();
                for ptr in &mut current_ptrs {
                    routes.push(Box::new(|| 
                        match direction {
                            Directions::LEFT => {
                                *ptr = &&self.maps.get(ptr).unwrap().0;
                            },
                            Directions::RIGHT => {
                                *ptr = &&self.maps.get(ptr).unwrap().1;
                            }
                        }));
                }
                
                for closure in &mut routes {
                    closure();
                }
                
                count += 1;
            } else {
                // restart the pattern if we run out
                pattern = self.lr_pattern.iter();
            }

            // if all current_ptrs are in the end_ptrs, break
            let mut all_at_end = true;
            for ptr in &current_ptrs {
                if !end_ptrs.contains(ptr) {
                    all_at_end = false;
                }
            }
            if all_at_end {
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

    // part 1
    // Start from AAA
    let first_ptr = lookup_ptr_map.get("AAA").unwrap();
    let end_ptr = lookup_ptr_map.get("ZZZ").unwrap();
    let mut end_ptrs: Vec<&NetworkMapLookupPtr> = Vec::new();
    end_ptrs.push(end_ptr);

    let count = navigation.navigate(first_ptr, end_ptrs);
    println!("Part 1: Navigation steps: {}", count);
    let duration = start.elapsed();
    println!("{:?}s", duration.as_secs_f64());

    // part 2

    // get all nodes ending in A:
    let start_ptrs: Vec<&NetworkMapLookupPtr> = lookup_ptr_map.iter().filter(|(key, _)| key.ends_with("A")).map(|(_, value)| value).collect();
    // get all nodes ending in Z:
    let end_ptrs: Vec<&NetworkMapLookupPtr> = lookup_ptr_map.iter().filter(|(key, _)| key.ends_with("Z")).map(|(_, value)| value).collect();

    let mut counts: Vec<u32> = Vec::new();

    for ptr in start_ptrs {
        let count = navigation.navigate(ptr, end_ptrs);
        counts.push(count);
    }

    let result = navigation.get_shared_lcm(counts);

    // let count = navigation.group_navigate(start_ptrs, end_ptrs);
    println!("Part 2: Navigation result: {}", result);
    let duration = start.elapsed();
    println!("{:?}s", duration.as_secs_f64());
}
