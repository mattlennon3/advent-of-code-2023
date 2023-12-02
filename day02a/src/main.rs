struct Game {
    game_id: u32,
    red_minimum: u32,
    blue_minimum: u32,
    green_minimum: u32,
}

impl Game {
    fn new(game_id: u32) -> Game {
        Game {
            game_id,
            red_minimum: 0,
            blue_minimum: 0,
            green_minimum: 0,
        }
    }

    fn set_min_red(&mut self, value: u32) {
        if value > self.red_minimum || self.red_minimum == 0 {
            self.red_minimum = value;
        }
    }

    fn set_min_blue(&mut self, value: u32) {
        if value > self.blue_minimum || self.blue_minimum == 0 {
            self.blue_minimum = value;
        }
    }

    fn set_min_green(&mut self, value: u32) {
        if value > self.green_minimum || self.green_minimum == 0 {
            self.green_minimum = value;
        }
    }

    fn get_power(&self) -> u32 {
        self.red_minimum * self.blue_minimum * self.green_minimum
    }
}

impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // write!(f, "{{game_id: {}}}", self.game_id)
        write!(
            f,
            "{{game_id: {}, red_minimum: {}, blue_minimum: {}, green_minimum: {}}}",
            self.game_id, self.red_minimum, self.blue_minimum, self.green_minimum
        )
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let mut total: u32 = 0;
    let mut games: Vec<Game> = Vec::new();

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
                match bag_color {
                    "red" => game.set_min_red(bag_count),
                    "blue" => game.set_min_blue(bag_count),
                    "green" => game.set_min_green(bag_count),
                    _ => !unreachable!("Invalid bag color: {}", bag_color),
                }
            }
        }
        println!("{}", game);
        games.push(game);
    }

    for game in games {
        total += game.get_power();
    }

    println!("Total: {}", total);
}
