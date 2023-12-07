use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    part1(input);
    part2(input);
}

fn part1(input : &str) {
    let hands = input.split("\n").map(str::trim).map(|line| Hand::parse(line, false)).collect::<Vec<_>>();
    let mut ranked : Vec<Hand> = vec![];

    for hand in hands {
        insert_naive(hand.clone(), &mut ranked);
    }

    let sum = ranked.iter()
        .enumerate()
        .map(|(index, value)| {
            (index as u32 + 1) * value.bid
        })
        .sum::<u32>();

    println!("PART 1: {}", sum);
}

fn part2(input : &str) {
    let hands = input.split("\n").map(str::trim).map(|line| Hand::parse(line, true)).collect::<Vec<_>>();
    let mut ranked : Vec<Hand> = vec![];

    for hand in hands {
        insert_naive(hand.clone(), &mut ranked);
    }

    let sum = ranked.iter()
        .enumerate()
        .map(|(index, value)| {
            (index as u32 + 1) * value.bid
        })
        .sum::<u32>();

    println!("PART 2: {}", sum);
}

fn insert_naive(hand : Hand, ranked : &mut Vec<Hand>) {
    let mut index = 0;

    while index < ranked.len() && hand.is_stronger_than(&ranked[index]) {
        index += 1;
    }

    ranked.insert(index, hand);
}

#[derive(Clone)]
struct Hand {
    cards : Vec<u32>,
    bid : u32,
    hand_type : HandType,
}

impl Hand {
    pub fn parse(input : &str, jokers : bool) -> Hand {
        let parts = input.split(" ").collect::<Vec<_>>();

        let cards = parts[0].chars().map(|c| {
            match c {
                '2'..='9' => {
                    c.to_digit(10).unwrap()
                }
                'T' => 10,
                'J' => match jokers {
                    true => 1,
                    false => 11,
                },
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                _ => {
                    panic!("Unexpected char: {}", c)
                }
            }
        }).collect::<Vec<_>>();

        let hand_type = match jokers {
            false => HandType::assess_p1(&cards),
            true => HandType::assess_p2(&cards),
        };

        Hand { cards: cards, bid: parts[1].parse::<u32>().unwrap(), hand_type: hand_type }
    }

    pub fn is_stronger_than(&self, other : &Hand) -> bool {
        if self.hand_type != other.hand_type {
            return self.hand_type.value() > other.hand_type.value();
        }

        for i in 0..5 {
            if self.cards[i] != other.cards[i] {
                return self.cards[i] > other.cards[i];
            }
        }

        panic!("Hands are identical?")
    }
}

#[derive(Debug, PartialEq, Clone)]
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
    pub fn value(&self) -> u32 {
        match self {
            HandType::FiveOfAKind => 7,
            HandType::FourOfAKind => 6,
            HandType::FullHouse => 5,
            HandType::ThreeOfAKind => 4,
            HandType::TwoPair => 3,
            HandType::OnePair => 2,
            HandType::HighCard => 1,
        }
    }

    pub fn assess_p2(cards : &Vec<u32>) -> HandType {
        let new_cards = cards.iter().filter(|card| **card != 1).map(|card| *card).collect::<Vec<_>>();
        let joker_count = cards.iter().filter(|card| **card == 1).count();

        if joker_count == 5 {
            return HandType::FiveOfAKind
        }

        let mut hand_type = HandType::assess_p1(&new_cards);

        for _ in 0..joker_count {
            hand_type = hand_type.joker_upgrade();
        }

        hand_type
    }

    pub fn assess_p1(cards : &Vec<u32>) -> HandType {
        let mut counts : HashMap<u32, usize> = HashMap::new();

        for card in cards {
            if !counts.contains_key(&card) {
                counts.insert(*card, 1);
            } else {
                counts.insert(*card, counts[card] + 1);
            }
        }

        if counts.iter().any(|(_, count)| *count == 5) {
            return HandType::FiveOfAKind
        }

        if counts.iter().any(|(_, count)| *count == 4) {
            return HandType::FourOfAKind
        }

        // Since we know it's not 4 of a kind, it must be 2/3 or 3/2, so fullhouse
        if counts.len() == 2 && cards.len() == 5 {
            return HandType::FullHouse
        } 

        if counts.iter().any(|(_, count)| *count == 3) {
            return HandType::ThreeOfAKind
        }

        // Since we know it's not 3 of a kind, it must be 2/2/1, 2/1/2, or 1/2/2, so two pair
        if counts.iter().filter(|(_, count)| **count == 2).count() == 2 {
            return HandType::TwoPair
        }

        if counts.iter().any(|(_, count)| *count == 2) {
            return HandType::OnePair
        }

        HandType::HighCard
    }

    pub fn joker_upgrade(&self) -> HandType {
        match self {
            HandType::FourOfAKind => Self::FiveOfAKind,
            HandType::ThreeOfAKind => Self::FourOfAKind,
            HandType::TwoPair => Self::FullHouse,
            HandType::OnePair => Self::ThreeOfAKind,
            HandType::HighCard => Self::OnePair,
            
            // This shouldn't happen
            _ => panic!("Shouldn't upgrade five card hand"),
        }
    }
}