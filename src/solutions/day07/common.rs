/* -------------------------------- HandType -------------------------------- */

use std::cmp::Ordering;

enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard
}

impl HandType {
    fn to_value(&self) -> usize {
        match self {
            HandType::FiveOfAKind  => 6,
            HandType::FourOfAKind  => 5,
            HandType::FullHouse    => 4,
            HandType::ThreeOfAKind => 3,
            HandType::TwoPair      => 2,
            HandType::OnePair      => 1,
            HandType::HighCard     => 0
        }
    }
}

/* ----------------------------- HandProperties ----------------------------- */

pub struct HandProperties {
    pub char_to_value: fn(char) -> usize,
    pub count_cards: fn(&[usize]) -> Vec<usize>
}

/* ---------------------------------- Hand ---------------------------------- */

struct Hand {
    cards: Vec<usize>,
    hand_type: HandType
}

impl Hand {
    fn parse(input: &str, properties: &HandProperties) -> Hand {
        let cards: Vec<usize> = input.chars().map(|c| (properties.char_to_value)(c)).collect();
        let hand_type: HandType = Self::determine_type(&cards, properties.count_cards);
        Hand { cards: cards, hand_type: hand_type }
    }

    pub fn compare(a: &Hand, b: &Hand) -> Ordering {
        let a_type_value: usize = a.hand_type.to_value();
        let b_type_value: usize = b.hand_type.to_value();

        match a_type_value.cmp(&b_type_value) {
            Ordering::Less    => { return Ordering::Less    }
            Ordering::Greater => { return Ordering::Greater }
            Ordering::Equal   => ()
        }

        a.cards.iter().zip(b.cards.iter())
            .map(|(a_card, b_card)| a_card.cmp(b_card))
            .find(|ord| *ord != Ordering::Equal)
            .unwrap_or(Ordering::Equal)
    }

    fn determine_type(cards: &Vec<usize>, count_cards: fn(&[usize]) -> Vec<usize>) -> HandType {
        let counts: Vec<usize> = count_cards(cards);
        let first:  usize = counts[0];
        let second: usize = counts[1];
        
        if first == 5 {
            HandType::FiveOfAKind
        } else if first == 4 {
            HandType::FourOfAKind
        } else if first == 3 && second == 2 {
            HandType::FullHouse
        } else if first == 3 {
            HandType::ThreeOfAKind
        } else if first == 2 && second == 2 {
            HandType::TwoPair
        } else if first == 2 {
            HandType::OnePair
        } else {
            HandType::HighCard
        }
    }
}

/* ----------------------------------- Row ---------------------------------- */

pub struct Row {
    hand: Hand,
    bid: i64
}

impl Row {
    pub fn parse(input: &str, properties: &HandProperties) -> Row {
        let (hand_str, bid_str) = input.split_once(' ').unwrap();

        let hand: Hand = Hand::parse(hand_str, properties);
        let bid: i64 = bid_str.parse().unwrap();
        
        Row { hand, bid }
    }

    pub fn compare(a: &Row, b: &Row) -> Ordering {
        Hand::compare(&a.hand, &b.hand)
    }

    pub fn compute_value(&self, index: usize) -> i64 {
        (index as i64 + 1) * self.bid
    }
}
