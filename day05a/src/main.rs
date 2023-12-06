use std::{collections::HashMap, time::Instant};

use aoc_crate::{numerical_parse, line_includes};

#[derive(PartialEq, Eq)]
struct AlmanacEntry {
    source: String,
    destination: String,
    // source, destination
    map: HashMap<u32, u32>
}
// <-- 
// source -to- destination

struct Almanac {
    entries: Vec<AlmanacEntry>
}

impl Almanac {
    fn new() -> Almanac {
        Almanac {
            entries: Vec::new()
        }
    }

    fn traverse_almanac(&self, seed: u32, destination: String) -> u32 {
        let mut current = seed;
        let mut current_entry = &self.entries[0];
        loop {
            // change seed for whatever material
            current = current_entry.get_destination_from_source(current);
            // find next entry with current destination as source
            current_entry = self.entries.iter().find(|entry| entry.source == current_entry.destination).unwrap();
            // If destination reached, break
            if current_entry.destination == destination {
                break;
            }
        }

        current_entry.get_destination_from_source(current)
    }
}

impl AlmanacEntry {
    fn new() -> AlmanacEntry {
        AlmanacEntry {
            source: String::new(),
            destination: String::new(),
            map: HashMap::new()
        }
    }

    fn construct_map(&mut self, source: u32, destination: u32, range_length: u32) {
        let start = Instant::now();
        for i in 0..range_length {
            self.map.insert(destination + i, source + i);
        }
        let duration = start.elapsed();
        println!("{:?}s", duration.as_secs_f64());
    }

    fn get_source_from_destination(&self, destination: u32) -> u32 {
        if let Some(source) = self.map.iter().find(|(_, value)| **value == destination).map(|(key, _)| key) {
            return source.clone();
        }
        destination
    }
    
    fn get_destination_from_source(&self, source: u32) -> u32 {
        if let Some(destination) = self.map.get(&source) {
            return destination.clone();
        }
        source
    }
}

impl std::fmt::Debug for AlmanacEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut map_string = String::new();
        for (key, value) in &self.map {
            map_string.push_str(&format!("{} -> {}\n", key, value));
        }
        write!(f, "AlmanacEntry {{ source: {}, destination: {}, map: {} }}", self.source, self.destination, map_string)
    }
}

fn main() {
    let start = Instant::now();

    let file_input = include_str!("../input.txt");

    let mut seed_input: Vec<u32> = Vec::new();
    let mut almanac: Almanac = Almanac::new();
    let mut entry = AlmanacEntry::new();

    // let ran = 0..100000;
    // ran.contains(item);


    for input in file_input.lines() {
        if line_includes(input, "-to-") {
            let (start, end) = input.split_once("-to-").unwrap();
            let (end, _) = end.split_once(" ").unwrap();
            entry.source = start.to_string();
            entry.destination = end.to_string();
        } else if line_includes(input, "seeds:") {
            let (_, end) = input.split_once(":").unwrap();
            seed_input = numerical_parse::<u32>(end).unwrap();
        } else if input.len() == 0 {
            if entry.map.len() > 0 {
                almanac.entries.push(entry);
                entry = AlmanacEntry::new();
            }
        } else {
            // numeric line
            let numbers: Vec<u32> = numerical_parse::<u32>(input).expect("Bad input");
            let (source, destination,range_length) = (numbers[0], numbers[1], numbers[2]);
            println!("Constructing map... {}-to-{}", entry.source, entry.destination);
            entry.construct_map(source, destination, range_length);
        }
    }

    if !almanac.entries.contains(&entry) {
        almanac.entries.push(entry);
    }

    println!("PARSED INPUT!");

    let duration = start.elapsed();
    println!("Time elapsed in seconds: {:?}", duration.as_secs_f64());

    let mut location_results: Vec<u32> = Vec::new();

    for seed in seed_input {
        let result = almanac.traverse_almanac(seed, "location".to_string());
        println!("Traverse seed {}: {}", seed, result);
        location_results.push(result)
    }

    println!("Lowest location value: {}" , location_results.iter().min().unwrap());

    let duration = start.elapsed();
    println!("Time elapsed in seconds: {:?}", duration.as_secs_f64());
}
