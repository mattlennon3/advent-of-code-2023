use std::{collections::btree_map::Range, time::Instant};

use regex::Regex;

#[derive(Clone)]
struct Card {
    id: u32,
    checked: bool,
    picked: Vec<u32>,
    winners: Vec<u32>,
}

#[derive(Clone)]
struct WinResult {
    id: u32,
    total: u32,
    matched: Vec<u32>,
}

struct CardDuplicationSet {
    cards: Vec<Card>,
}

impl CardDuplicationSet {
    fn new(cards: Vec<Card>) -> CardDuplicationSet {
        CardDuplicationSet { cards }
    }

    fn get_first_unchecked_card(&self) -> Option<&Card> {
        self.cards.iter().find(|card| !card.checked)
    }
    fn get_first_unchecked_card_mut(&mut self) -> Option<&mut Card> {
        self.cards.iter_mut().find(|card| !card.checked)
    }

    fn get_first_card_by_id(&self, id: u32) -> &Card {
        self.cards.iter().find(|card| card.id == id).unwrap()
    }

    fn get_next_card_by_id(&self, range: std::ops::Range<u32>) -> Vec<&Card> {
        let mut cards: Vec<&Card> = Vec::new();
        for id in range {
            cards.push(self.get_first_card_by_id(id));
        }
        cards
    }

    fn get_duplicated_cards(&self, card: &Card) -> Option<Vec<Card>> {
        let mut new_duplicated_cards: Vec<Card> = Vec::new();

        if let Some(ahead) = card.get_ahead_card_count() {
            let next_id = card.id + 1;
            for next_card in self.get_next_card_by_id(next_id..next_id + ahead) {
                new_duplicated_cards.push(next_card.clone())
            }
        }

        if new_duplicated_cards.len() == 0 {
            return None;
        }
        Some(new_duplicated_cards)
    }

    fn insert_duplicated_cards(&mut self, cards: Vec<Card>) {
        // Merge the cards into the existing set, in order of their ID

        // Find the index of the card with an id matching the first card in the new set
        for new_card in cards {
            let index = self
                .cards
                .iter()
                .position(|card| card.id == new_card.id)
                .unwrap();
            self.cards.insert(index + 1, new_card);
        }

    }

    // fn check_next_card(&mut self) -> Option<()> {
    //     let mut new_cards: Vec<Card> = Vec::new();
    //     if let Some(card) = self.get_first_unchecked_card() {
    //         if let Some(ahead) = card.get_ahead_card_count() {
    //             for next_card in self.get_next_card_by_id(card.id..card.id + ahead) {
    //                 new_cards.push(next_card.clone())
    //             }
    //             // get next card by id
    //             //
    //         }
    //         card.checked = true;
    //         ()
    //     }
    //     None
    // }
}

impl Card {
    fn new(id: u32) -> Card {
        Card {
            id,
            checked: false,
            picked: Vec::new(),
            winners: Vec::new(),
        }
    }

    fn get_ahead_card_count(&self) -> Option<u32> {
        if let Some(winner) = self.get_winning_points() {
            return Some(winner.matched.len() as u32);
        }
        None
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

fn duplicate_card(card: &Card) -> Card {
    card.clone()
}

impl std::fmt::Debug for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let winners: Vec<u32> = self.winners.iter().map(|winner| *winner).collect();
        let picked: Vec<u32> = self.picked.iter().map(|picked| *picked).collect();
        write!(
            f,
            "Card {{ id: {}, winners: {:?}, picked: {:?} }}",
            self.id, winners, picked
        )
    }
}

impl std::fmt::Debug for WinResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "WinResult {{ id: {}, total: {}, matched: {:?} }}",
            self.id, self.total, self.matched
        )
    }
}

fn main() {
    let start = Instant::now();

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

        // println!("winners_str {}", winners_str);
        // println!("picked_str {}", picked_str);

        for mat in digits_re.find_iter(winners_str) {
            card.winners
                .push(winners_str[mat.start()..mat.end()].parse::<u32>().unwrap());
        }

        for mat in digits_re.find_iter(picked_str) {
            card.picked
                .push(picked_str[mat.start()..mat.end()].parse::<u32>().unwrap());
        }

        cards.push(card);
        // println!("card {:?}", card);
        // println!("card {:?}", card.get_winning_points());
    }

    let mut dup_set = CardDuplicationSet::new(cards);

    let mut loader = "-";

    loop {
        if let Some(card) = dup_set.get_first_unchecked_card() {
            println!("{} Processing Card {}", loader, card.id);
            let new_cards = dup_set.get_duplicated_cards(card);
            if let Some(new_cards) = new_cards {
                // println!("NEW CARDS! {} | {:?}", card.id, new_cards);
                dup_set.insert_duplicated_cards(new_cards);
            }
        }

        if let Some(card) = dup_set.get_first_unchecked_card_mut() {
            card.checked = true;

            if loader == "-" {
                loader = "\\"
            } else if loader == "\\" {
                loader = "|"
            } else if loader == "|" {
                loader = "/"
            } else if loader == "/" {
                loader = "-"
            }
        } else {
            break;
        }
    }

    println!("Total cards in set: {}", dup_set.cards.len());
    
    let duration = start.elapsed();
    println!("Time elapsed was: {:?}", duration);

    // loop over cards
    // if

    // call get_winning_points on all cards, filter out None results
    // let results: Vec<WinResult> = cards
    //     .iter()
    //     .map(|card| card.get_winning_points())
    //     .filter(|result| result.is_some())
    //     .map(|result| result.unwrap()).collect();

    // println!("Winning total: {}", winning_points)
}
