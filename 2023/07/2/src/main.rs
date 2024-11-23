use std::{
    cmp::Ordering,
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
    string::ParseError,
};

const CARD_ORDER: [char; 13] = [
    'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
];

const JOKER: Card = Card { order: 0 };

fn main() {
    let file = File::open("input.txt").expect("opening input file");

    let mut card_hands: Vec<CardHand> = BufReader::new(file)
        .lines()
        .map_while(Result::ok)
        .map(|line| line.parse().unwrap())
        .collect();

    card_hands.sort();

    let result = card_hands
        .iter()
        .enumerate()
        .map(|(i, card_hand)| (i + 1) as u32 * card_hand.bid)
        .reduce(|acc, e| acc + e)
        .unwrap_or_default();

    println!("{result}");
}

struct CardHand {
    cards: Vec<Card>,
    bid: u32,
    kind: CardHandKind,
}

impl FromStr for CardHand {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hand: Vec<&str> = s.split_whitespace().collect();
        let cards: Vec<Card> = hand[0].chars().map(Card::from).collect();
        let kind = CardHandKind::from(&cards);

        Ok(Self {
            cards,
            bid: hand[1].parse().unwrap(),
            kind,
        })
    }
}

impl PartialOrd for CardHand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CardHand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.kind.cmp(&other.kind) {
            Ordering::Equal => self.cards.cmp(&other.cards),
            ordering => ordering,
        }
    }
}

impl PartialEq for CardHand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl Eq for CardHand {}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
struct Card {
    order: usize,
}

impl Card {
    fn from(c: char) -> Self {
        Self {
            order: CARD_ORDER.iter().position(|&r| r == c).unwrap(),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum CardHandKind {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl CardHandKind {
    fn from(cards: &[Card]) -> Self {
        let mut map = cards
            .iter()
            .fold(HashMap::new(), |mut map: HashMap<&Card, usize>, x| {
                *map.entry(x).or_default() += 1;
                map
            });

        if let Some(number_of_jokers) = map.remove(&JOKER) {
            if number_of_jokers == 5 {
                map.insert(&JOKER, number_of_jokers);
            } else {
                let max_number_card = map
                    .iter()
                    .max_by(|(_, a_value), (_, b_value)| a_value.cmp(b_value))
                    .map(|(key, _)| *key)
                    .unwrap();

                *map.entry(max_number_card).or_default() += number_of_jokers;
            }
        }

        match map.len() {
            1 => Self::FiveOfAKind,
            2 => {
                if map.values().any(|count| *count == 4) {
                    Self::FourOfAKind
                } else {
                    Self::FullHouse
                }
            }
            3 => {
                if map.values().any(|count| *count == 3) {
                    Self::ThreeOfAKind
                } else {
                    Self::TwoPair
                }
            }
            4 => Self::OnePair,
            _ => Self::HighCard,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::CardHandKind;

    #[test]
    fn test_kind_order() {
        assert!(CardHandKind::FiveOfAKind > CardHandKind::FourOfAKind);
    }

    #[test]
    fn test_kind_equality() {
        assert!(CardHandKind::TwoPair == CardHandKind::TwoPair);
    }
}
