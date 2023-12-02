use std::path::Display;

struct Game {
    game_id: u32,
    possible: bool,
    // red_total: u32,
    // blue_total: u32,
    // green_total: u32,
}

impl Game {
    fn new(game_id: u32) -> Game {
        Game {
            game_id,
            possible: true,
            // red_total: 0,
            // blue_total: 0,
            // green_total: 0,
        }
    }

    // fn add_red(&mut self, value: u32) {
    //     self.red_total += value;
    // }

    // fn add_blue(&mut self, value: u32) {
    //     self.blue_total += value;
    // }

    // fn add_green(&mut self, value: u32) {
    //     self.green_total += value;
    // }

    // fn is_possible(&self, red_limit: u32, blue_limit: u32, green_limit: u32) -> bool {
    //     if self.red_total > red_limit {
    //         return false;
    //     }
    //     if self.blue_total > blue_limit {
    //         return false;
    //     }
    //     if self.green_total > green_limit {
    //         return false;
    //     }
    //     return true;
    // }

    // fn get_total(&self) -> u32 {
    //     self.red_total + self.blue_total + self.green_total
    // }
}

impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{game_id: {}}}", self.game_id)
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let mut total: u32 = 0;
    let mut games: Vec<Game> = Vec::new();

    let red_limit = 12;
    let green_limit = 13;
    let blue_limit = 14;

    for game_str in input.lines() {
        // example game input "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
        let mut game_split_input = game_str.split(":");
        let game_id = game_split_input
            .next()
            .unwrap()
            .split(" ")
            .last()
            .unwrap()
            .parse::<u32>()
            .unwrap();

        let mut game = Game::new(game_id);

        let bags = game_split_input.next().unwrap().split(";");

        for bag in bags {
            let bag_split_input = bag.split(",");
            for colour in bag_split_input {
                // colour: " 6 red" (need to trim)
                let mut bag_color = colour.trim().split(" ");
                let bag_count = bag_color.next().unwrap().parse::<u32>().unwrap();
                let bag_color = bag_color.next().unwrap().trim();
                let possible = match bag_color {
                    "red" => bag_count <= red_limit,
                    "blue" => bag_count <= blue_limit,
                    "green" => bag_count <= green_limit,
                    _ => !unreachable!("Invalid bag color: {}", bag_color),
                };
                if !possible {
                    game.possible = false;
                    break;
                }
            }
        }

        if game.possible {
            println!("Possible Game: {}", game);
            games.push(game);
        }
    }

    for game in games {
        total += game.game_id;
    }

    println!("Total: {}", total);
}
