use std::collections::HashMap;

#[derive(Eq, PartialEq, Hash, Clone, PartialOrd)]
enum Card {
    J,
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
    T,
    Q,
    K,
    A
}

/**
 *  Five of a kind, where all five cards have the same label: AAAAA
    Four of a kind, where four cards have the same label and one card has a different label: AA8AA
    Full house, where three cards have the same label, and the remaining two cards share a different label: 23332
    Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
    Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
    One pair, where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
    High card, where all cards' labels are distinct: 23456
 */
#[derive(PartialEq, Eq, PartialOrd)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind
}

impl Card {
    fn new(label: char) -> Card {
        match label {
            '2' => Card::N2,
            '3' => Card::N3,
            '4' => Card::N4,
            '5' => Card::N5,
            '6' => Card::N6,
            '7' => Card::N7,
            '8' => Card::N8,
            '9' => Card::N9,
            'T' => Card::T,
            'J' => Card::J,
            'Q' => Card::Q,
            'K' => Card::K,
            'A' => Card::A,
            _ => panic!("Unknown card label: {}", label)
        }
    }

}

struct Hand {
    // cards length is 5
    cards: Vec<Card>,
    bid: u32
}

impl Hand {
    fn new(cards_str: &str, bid: u32) -> Hand {
        let cards: Vec<Card> = cards_str.chars().map(|card| Card::new(card)).collect();
        Hand { cards, bid }
    }

    // https://old.reddit.com/r/adventofcode/comments/18cr4xr/2023_day_7_better_example_input_not_a_spoiler/
    fn get_hand_type(&self) -> HandType {

        // the problem is when jokers are the highest count card
        // TODO: don't do joker sorting here. Before this, create a new temp hand and replace the jokers
        // with the best cards.
        // Then pass to this function.
        // remove joker count code etc

        let mut card_counts: HashMap<Card, u32> = HashMap::new();
        for card in self.cards.iter() {
            let count = card_counts.entry(card.clone()).or_insert(0);
            *count += 1;
        }

        let mut counts: Vec<u32> = card_counts.values().map(|count| *count).collect();
        
        counts.sort();

        // J cards are jokers - wildcards that can act like whatever card would make the hand the strongest type possible
        let joker_count = self.cards.iter().filter(|card| **card == Card::J).count() as u32;
        let max_count = counts.iter().max().unwrap();
        let amount_of_twos = counts.iter().filter(|count| **count == 2).count() as u32;

        println!("hand: {:?}, joker_count: {} counts: {:?}", self, joker_count, counts);
        if joker_count > 0 && joker_count < 5 {
            // if joker_count == 2 && amount_of_twos == 1 {
                
            // }
            if let Some(last) = counts.last_mut() {
                *last += joker_count;
            }

            // while total of counts is greater than 5, remove smallest value from counts
            while counts.iter().sum::<u32>() > 5 {
                // remove smallest value from counts
                // first value is always the smallest because it's been sorted
                if let Some(first) = counts.first_mut() {
                    *first -= 1;
                }
                if counts.first() == Some(&0) {
                    counts.remove(0);
                }
            }
            println!("new counts: {:?}", counts);
        }

        println!("hand: {:?}, joker_count: {} counts: {:?}\n", self, joker_count, counts);

        match counts.as_slice() {
            [1, 1, 1, 1, 1] => HandType::HighCard,
            [1, 1, 1, 2] => HandType::OnePair,
            [1, 2, 2] => HandType::TwoPair,
            [1, 1, 3] => HandType::ThreeOfAKind,
            [2, 3] => HandType::FullHouse,
            [1, 4] => HandType::FourOfAKind,
            [5] => HandType::FiveOfAKind,
            _ => panic!("Unknown hand type: {:?}", counts)
        }
    }
}

impl std::fmt::Debug for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cards_str: Vec<String> = self.cards.iter().map(|card| format!("{:?}", card)).collect();
        write!(f, "{} {}", cards_str.join(""), self.bid)
    }
}

impl std::fmt::Debug for HandType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let label = match self {
            HandType::HighCard => "HighCard",
            HandType::OnePair => "OnePair",
            HandType::TwoPair => "TwoPair",
            HandType::ThreeOfAKind => "ThreeOfAKind",
            HandType::FullHouse => "FullHouse",
            HandType::FourOfAKind => "FourOfAKind",
            HandType::FiveOfAKind => "FiveOfAKind",
        };
        write!(f, "{}", label)
    }
}

impl std::fmt::Debug for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let label = match self {
            Card::N2 => "2",
            Card::N3 => "3",
            Card::N4 => "4",
            Card::N5 => "5",
            Card::N6 => "6",
            Card::N7 => "7",
            Card::N8 => "8",
            Card::N9 => "9",
            Card::T => "T",
            Card::J => "J",
            Card::Q => "Q",
            Card::K => "K",
            Card::A => "A",
        };
        write!(f, "{}", label)
    }
}

struct Game {
    hands: Vec<Hand>
}

impl Game {
    fn new() -> Game {
        Game { hands: Vec::new() }
    }

    fn add_hand(&mut self, hand: Hand) {
        self.hands.push(hand);
    }

    /**
     * Hands are primarily ordered based on type; for example, every full house is stronger than any three of a kind.
        If two hands have the same type, a second ordering rule takes effect. Start by comparing the first card in each hand. 
        If these cards are different, the hand with the stronger first card is considered stronger. If the first card in each hand have the same label, however, then move on to considering the second card in each hand. 
        If they differ, the hand with the higher second card wins; otherwise, continue with the third card in each hand, then the fourth, then the fifth.
     */
    fn order_hands(&mut self) {
        self.hands.sort_by(|phand, nhand| {
            let phand_type = phand.get_hand_type();
            let nhand_type = nhand.get_hand_type();
            if phand_type == nhand_type {
                for i in 0..5 {
                    if phand.cards[i] != nhand.cards[i] {
                        return phand.cards[i].partial_cmp(&nhand.cards[i]).unwrap();
                    }
                }
            }
            phand_type.partial_cmp(&nhand_type).unwrap()
        });
    }
}



fn main() {
    let file_input = include_str!("../input.txt");
    let mut game = Game::new();

    for input in file_input.lines() {
        let (hand_str, bid) = input.split_once(" ").unwrap();
        let hand = Hand::new(hand_str, bid.parse::<u32>().unwrap());
        game.add_hand(hand);
    }

    game.order_hands();

    let mut rank = 1;
    let mut total = 0;
    for hand in game.hands {
        println!("-------------------");
        println!("({}) {}: {:?} {:?}\n", hand.bid * rank, rank, hand, hand.get_hand_type());
        total += hand.bid * rank;
        rank += 1;
    }

    println!("total {}", total);

}
