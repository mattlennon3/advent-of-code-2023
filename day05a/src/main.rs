use std::{collections::HashMap, time::Instant};
use std::ops::{Range, Add};
use aoc_crate::{numerical_parse, line_includes};

#[derive(Clone, PartialEq, Eq)]
struct RangeMap {
    source_range: Range<u32>,
    destination_range: Range<u32>,
    range_length: u32,
}

#[derive(PartialEq, Eq)]
struct AlmanacEntry {
    source: String,
    destination: String,
    // source, destination
    // map: HashMap<u32, u32>,
    ranges: Vec<RangeMap>,
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
        if seed == 14 {
            println!("Entry1: {:?}", current_entry);
        }
        loop {
            // change seed for whatever material
            println!("TRAVERSE ({}): {} -> {} | {}", seed, current_entry.source, current_entry.destination, current);
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
            ranges: Vec::new(),
        }
    }

    fn construct_map(&mut self, source: u32, destination: u32, range_length: u32) {
        let start = Instant::now();
        let range_map = RangeMap {
            source_range: source..source + range_length - 1,
            destination_range: destination..destination + range_length - 1,
            range_length,
        };

        println!("Added range map: {:?}", range_map);

        self.ranges.push(range_map);

        // for i in 0..range_length {
        //     self.map.insert(destination + i, source + i);
        // }
        let duration = start.elapsed();
        // println!("{:?}s", duration.as_secs_f64());
    }

    fn get_source_from_destination(&self, destination: u32) -> u32 {

        self.ranges.iter().find(|range_map| range_map.destination_range.contains(&destination)).map(|range_map| {
            let offset = destination - range_map.destination_range.start;
            range_map.source_range.start + offset
        }).unwrap_or(destination)

        // if let Some(source) = self.map.iter().find(|(_, value)| **value == destination).map(|(key, _)| key) {
        //     return source.clone();
        // }
        // destination
    }
    
    // if source == 14 {
    //     println!("Rangemap: {:?}", range_map);
    //     println!("{} -> {} | {}", range_map.source_range.start, range_map.destination_range.start, offset);
    // }
    fn get_destination_from_source(&self, source: u32) -> u32 {

        // self.ranges.iter().find(|range_map| range_map.source_range.contains(&source)).map(|range_map| {
        //     let index = range_map.source_range.position(|i| i == source).unwrap();
        //     let offset = range_map.destination_range.start.add(index) as u32;
        //     println!("OFFSET {}", offset);
        //     return offset;
        // }).unwrap_or(source)

        self.ranges.iter().find(|range_map| range_map.source_range.contains(&source)).map(|range_map| {
            range_map.source_range.
            let offset = source - range_map.source_range.start;
            range_map.destination_range.start + offset
        }).unwrap_or(source)
    }
}

impl std::fmt::Debug for RangeMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{ source_range: {}..{}, destination_range: {}..{} }}",
            self.source_range.start, self.source_range.end, self.destination_range.start, self.destination_range.end
        )
    }
}

impl std::fmt::Debug for AlmanacEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ranges: Vec<RangeMap> = self.ranges.iter().map(|range_map| range_map.clone()).collect();
        write!(
            f,
            "AlmanacEntry {{ source: {}, destination: {}, ranges: {:?} }}",
            self.source, self.destination, ranges
        )
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
            if entry.ranges.len() > 0 {
                almanac.entries.push(entry);
                entry = AlmanacEntry::new();
            }
        } else {
            // numeric line
            let numbers: Vec<u32> = numerical_parse::<u32>(input).expect("Bad input");
            let (source, destination,range_length) = (numbers[0], numbers[1], numbers[2]);
            println!("Constructing map... {}-to-{}", entry.source, entry.destination);
            entry.construct_map(destination, source, range_length);
        }
    }

    if !almanac.entries.contains(&entry) {
        almanac.entries.push(entry);
    }

    println!("PARSED INPUT!");

    let duration = start.elapsed();
    println!("Time elapsed in seconds: {:?}", duration.as_secs_f64());

    let mut location_results: Vec<u32> = Vec::new();

    println!("Entry1: 14: {:?}", almanac.entries[1].get_destination_from_source(14));
    println!("Entry1: 0: {:?}", almanac.entries[1].get_destination_from_source(0));
    println!("Entry1: 37: {:?}", almanac.entries[1].get_destination_from_source(37));
    println!("Entry1: 39: {:?}", almanac.entries[1].get_destination_from_source(39));
    println!("Entry1: {:?}", almanac.entries[1].ranges);

    for seed in seed_input {
        let result = almanac.traverse_almanac(seed, "location".to_string());
        println!("Traverse seed {}: {}\n", seed, result);
        location_results.push(result)
    }

    println!("Lowest location value: {}" , location_results.iter().min().unwrap());

    let duration = start.elapsed();
    println!("Time elapsed in seconds: {:?}", duration.as_secs_f64());
}


// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_traverses_seed_2() {



//         assert_eq!(result, 4);
//     }
// }
