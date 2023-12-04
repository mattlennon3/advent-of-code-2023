use regex::Regex;

struct Card {
    id: u32,
    picked: Vec<u32>,
    winners: Vec<u32>,
}

struct WinResult {
    id: u32,
    total: u32,
    matched: Vec<u32>,
}

impl Card {
    fn new(id: u32) -> Card {
        Card {
            id,
            picked: Vec::new(),
            winners: Vec::new(),
        }
    }

    fn get_winning_points(&self) -> Option<WinResult> {
        let mut result = WinResult {
            id: self.id,
            total: 0,
            matched: Vec::new(),
        };
        for winner in &self.winners {
            if let Some(found) = &self.picked.iter().find(|picked| picked == &winner) {
                result.matched.push(**found)
            }
        }

        if result.matched.len() == 0 {
            return None;
        }
        // Double the points for each matched number
        result.total = result.matched.iter().fold(0, |acc, _| {
            if acc == 0 {
                return 1;
            }
            acc * 2
        });

        return Some(result);
    }
}

impl std::fmt::Debug for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let winners: Vec<u32> = self.winners.iter().map(|winner| *winner).collect();
        let picked: Vec<u32> = self.picked.iter().map(|picked| *picked).collect();
        write!(f, "Card {{ id: {}, winners: {:?}, picked: {:?} }}", self.id, winners, picked)
    }
}

impl std::fmt::Debug for WinResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "WinResult {{ id: {}, total: {}, matched: {:?} }}", self.id, self.total, self.matched)
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let mut cards: Vec<Card> = Vec::new();

    let digits_re = Regex::new(r"\d+").unwrap();

    for card_str in input.lines() {
        // Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53

        let mut card_split_input = card_str.split(":");
        // todo find out how to avoid this chaining
        let card_id = card_split_input
            .next()
            .unwrap()
            .split(" ")
            .last()
            .unwrap()
            .parse::<u32>()
            .unwrap();
        let mut card = Card::new(card_id);

        let mut card_split_input = card_split_input.next().unwrap().split("|");
        let winners_str = card_split_input.next().unwrap();
        let picked_str = card_split_input.next().unwrap();

        println!("winners_str {}", winners_str);
        println!("picked_str {}", picked_str);

        for mat in digits_re.find_iter(winners_str) {
            card.winners.push(winners_str[mat.start()..mat.end()].parse::<u32>().unwrap());
        }

        for mat in digits_re.find_iter(picked_str) {
            card.picked.push(picked_str[mat.start()..mat.end()].parse::<u32>().unwrap());
        }

        cards.push(card);
        // println!("card {:?}", card);
        // println!("card {:?}", card.get_winning_points());
    }

    // call get_winning_points on all cards, filter out None results
    let winning_points: u32 = cards
        .iter()
        .map(|card| card.get_winning_points())
        .filter(|result| result.is_some())
        .map(|result| result.unwrap().total)
        .fold(0, |acc, next| acc + next);

    println!("Winning total: {}", winning_points)    

}
