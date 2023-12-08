advent_of_code::solution!(7);

use std::{cmp::Ordering, collections::HashSet, hash::Hash};

#[derive(Debug, Clone)]
enum Rule {
    Default,
    WithJoker,
}

#[derive(Debug, PartialEq, Clone, Eq, PartialOrd, Ord)]
enum Type<T> {
    FiveOfAKind(T),
    FourOfAKind(T),
    FullHouse(T),
    ThreeOfAKind(T),
    TwoPair(T, T),
    OnePair(T),
    HighCard,
}

impl<T> Type<T> {
    pub fn cmp(&self, other: &Type<T>) -> std::cmp::Ordering {
        match self {
            Type::FiveOfAKind(_) => match other {
                Type::FiveOfAKind(_) => Ordering::Equal,
                _ => Ordering::Greater,
            },
            Type::FourOfAKind(_) => match other {
                Type::FiveOfAKind(_) => Ordering::Less,
                Type::FourOfAKind(_) => Ordering::Equal,
                _ => Ordering::Greater,
            },
            Type::FullHouse(_) => match other {
                Type::FiveOfAKind(_) => Ordering::Less,
                Type::FourOfAKind(_) => Ordering::Less,
                Type::FullHouse(_) => Ordering::Equal,
                _ => Ordering::Greater,
            },
            Type::ThreeOfAKind(_) => match other {
                Type::FiveOfAKind(_) => Ordering::Less,
                Type::FourOfAKind(_) => Ordering::Less,
                Type::FullHouse(_) => Ordering::Less,
                Type::ThreeOfAKind(_) => Ordering::Equal,
                _ => Ordering::Greater,
            },
            Type::TwoPair(_, _) => match other {
                Type::FiveOfAKind(_) => Ordering::Less,
                Type::FourOfAKind(_) => Ordering::Less,
                Type::FullHouse(_) => Ordering::Less,
                Type::ThreeOfAKind(_) => Ordering::Less,
                Type::TwoPair(_, _) => Ordering::Equal,
                _ => Ordering::Greater,
            },
            Type::OnePair(_) => match other {
                Type::FiveOfAKind(_) => Ordering::Less,
                Type::FourOfAKind(_) => Ordering::Less,
                Type::FullHouse(_) => Ordering::Less,
                Type::ThreeOfAKind(_) => Ordering::Less,
                Type::TwoPair(_, _) => Ordering::Less,
                Type::OnePair(_) => Ordering::Equal,
                _ => Ordering::Greater,
            },
            Type::HighCard => match other {
                Type::HighCard => Ordering::Equal,
                _ => Ordering::Less,
            },
        }
    }
}

trait CardType {
    fn from_char(c: char) -> Option<Self>
    where
        Self: Sized;
    fn value(&self) -> u32;
    fn rule(&self) -> Rule;
    fn compare(&self, other: &Self) -> std::cmp::Ordering {
        self.value().cmp(&other.value())
    }
}

#[derive(Debug, PartialEq, Clone, Eq, PartialOrd, Ord, Hash)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    J,
    Q,
    K,
    A,
}

impl CardType for Card {
    fn from_char(s: char) -> Option<Card> {
        match s {
            '2' => Some(Card::Two),
            '3' => Some(Card::Three),
            '4' => Some(Card::Four),
            '5' => Some(Card::Five),
            '6' => Some(Card::Six),
            '7' => Some(Card::Seven),
            '8' => Some(Card::Eight),
            '9' => Some(Card::Nine),
            'T' => Some(Card::T),
            'J' => Some(Card::J),
            'Q' => Some(Card::Q),
            'K' => Some(Card::K),
            'A' => Some(Card::A),
            _ => None,
        }
    }

    fn value(&self) -> u32 {
        match self {
            Card::Two => 2,
            Card::Three => 3,
            Card::Four => 4,
            Card::Five => 5,
            Card::Six => 6,
            Card::Seven => 7,
            Card::Eight => 8,
            Card::Nine => 9,
            Card::T => 10,
            Card::J => 11,
            Card::Q => 12,
            Card::K => 13,
            Card::A => 14,
        }
    }

    fn rule(&self) -> Rule {
        Rule::Default
    }
}

#[derive(Debug, PartialEq, Clone, Eq, PartialOrd, Ord, Hash)]
enum CardWithJoker {
    J,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    Q,
    K,
    A,
}

impl CardType for CardWithJoker {
    fn value(&self) -> u32 {
        match self {
            CardWithJoker::J => 1,
            CardWithJoker::Two => 2,
            CardWithJoker::Three => 3,
            CardWithJoker::Four => 4,
            CardWithJoker::Five => 5,
            CardWithJoker::Six => 6,
            CardWithJoker::Seven => 7,
            CardWithJoker::Eight => 8,
            CardWithJoker::Nine => 9,
            CardWithJoker::T => 10,
            CardWithJoker::Q => 11,
            CardWithJoker::K => 12,
            CardWithJoker::A => 13,
        }
    }

    fn from_char(c: char) -> Option<CardWithJoker> {
        match c {
            '2' => Some(CardWithJoker::Two),
            '3' => Some(CardWithJoker::Three),
            '4' => Some(CardWithJoker::Four),
            '5' => Some(CardWithJoker::Five),
            '6' => Some(CardWithJoker::Six),
            '7' => Some(CardWithJoker::Seven),
            '8' => Some(CardWithJoker::Eight),
            '9' => Some(CardWithJoker::Nine),
            'T' => Some(CardWithJoker::T),
            'J' => Some(CardWithJoker::J),
            'Q' => Some(CardWithJoker::Q),
            'K' => Some(CardWithJoker::K),
            'A' => Some(CardWithJoker::A),
            _ => None,
        }
    }

    fn rule(&self) -> Rule {
        Rule::WithJoker
    }
}

#[derive(Debug, PartialEq, Clone, Eq, PartialOrd, Ord)]
struct Hand<T> {
    cards: Vec<T>,
    bid: u32,
    hand_type: Type<T>,
}

impl<T: Eq + Clone + CardType + Hash + Ord> Hand<T> {
    fn new(cards: Vec<T>, bid: u32) -> Hand<T> {
        let hand_type = get_type(cards.clone());

        Hand {
            cards,
            bid,
            hand_type,
        }
    }

    fn cmp(&self, other: &Self) -> Ordering {
        let ordering = self.hand_type.cmp(&other.hand_type);

        match ordering {
            Ordering::Equal => {
                for i in 0..self.cards.len() {
                    if self.cards[i] == other.cards[i] {
                        continue;
                    }
                    return self.cards[i].compare(&other.cards[i]);
                }

                panic!("Hands are equal ?")
            }
            _ => ordering,
        }
    }
}

fn get_type<T: PartialEq + Eq + Hash + Clone + CardType>(cards: Vec<T>) -> Type<T> {
    let counts = cards
        .iter()
        .map(|card| ((*card).clone(), cards.iter().filter(|c| *c == card).count()))
        .collect::<HashSet<(T, usize)>>();

    match cards[0].rule() {
        Rule::Default => {
            if counts.iter().any(|(_, count)| *count == 5) {
                Type::FiveOfAKind(cards[0].clone())
            } else if counts.iter().any(|(_, count)| *count == 4) {
                Type::FourOfAKind(cards[0].clone())
            } else if counts.iter().any(|(_, count)| *count == 3)
                && counts.iter().any(|(_, count)| *count == 2)
            {
                Type::FullHouse(cards[0].clone())
            } else if counts.iter().any(|(_, count)| *count == 3) {
                Type::ThreeOfAKind(cards[0].clone())
            } else if counts.iter().filter(|(_, count)| *count == 2).count() == 2 {
                let mut pairs = counts
                    .iter()
                    .filter(|(_, count)| *count == 2)
                    .map(|(card, _)| (*card).clone())
                    .collect::<Vec<T>>();

                pairs.sort_by(|a, b| b.value().cmp(&a.value()));

                Type::TwoPair(pairs[0].clone(), pairs[1].clone())
            } else if counts.iter().any(|(_, count)| *count == 2) {
                Type::OnePair(cards[0].clone())
            } else {
                Type::HighCard
            }
        }
        Rule::WithJoker => {
            let joker_occurences = counts
                .iter()
                .find(|(card, _)| *card == T::from_char('J').unwrap())
                .unwrap_or(&(T::from_char('J').unwrap(), 0))
                .1;

            if counts.iter().any(|(_, count)| *count == 5) {
                Type::FiveOfAKind(cards[0].clone())
            } else if counts
                .iter()
                .any(|(card, count)| *count == 4 && *card != T::from_char('J').unwrap())
            {
                match joker_occurences {
                    1 => Type::FiveOfAKind(cards[0].clone()),
                    _ => Type::FourOfAKind(cards[0].clone()),
                }
            } else if counts
                .iter()
                .any(|(card, count)| *count == 3 && *card != T::from_char('J').unwrap())
            {
                match joker_occurences {
                    2 => Type::FiveOfAKind(cards[0].clone()),
                    1 => Type::FourOfAKind(cards[0].clone()),
                    _ => {
                        if counts
                            .iter()
                            .any(|(card, count)| *count == 2 && *card != T::from_char('J').unwrap())
                        {
                            Type::FullHouse(cards[0].clone())
                        } else {
                            Type::ThreeOfAKind(cards[0].clone())
                        }
                    }
                }
            } else if counts
                .iter()
                .any(|(card, count)| *count == 2 && *card != T::from_char('J').unwrap())
            {
                match joker_occurences {
                    3 => Type::FiveOfAKind(cards[0].clone()),
                    2 => Type::FourOfAKind(cards[0].clone()),
                    1 => {
                        if counts.iter().filter(|(_, count)| *count == 2).count() == 2 {
                            Type::FullHouse(cards[0].clone())
                        } else {
                            Type::ThreeOfAKind(cards[0].clone())
                        }
                    }
                    _ => {
                        if counts.iter().filter(|(_, count)| *count == 2).count() == 2 {
                            let mut pairs = counts
                                .iter()
                                .filter(|(_, count)| *count == 2)
                                .map(|(card, _)| (*card).clone())
                                .collect::<Vec<T>>();

                            pairs.sort_by(|a, b| b.value().cmp(&a.value()));

                            Type::TwoPair(pairs[0].clone(), pairs[1].clone())
                        } else {
                            Type::OnePair(cards[0].clone())
                        }
                    }
                }
            } else {
                match joker_occurences {
                    4 => Type::FiveOfAKind(cards[0].clone()),
                    3 => Type::FourOfAKind(cards[0].clone()),
                    2 => Type::ThreeOfAKind(cards[0].clone()),
                    1 => Type::OnePair(cards[0].clone()),
                    _ => Type::HighCard,
                }
            }
        }
    }
}

fn sort_hands<T: PartialEq + Eq + Hash + Clone + CardType + Ord>(hands: &mut Vec<Hand<T>>) {
    hands.sort_by(|a, b| a.cmp(&b));
}

fn parse_input<T: CardType + Eq + Hash + Clone + Ord>(input: &str) -> Vec<Hand<T>> {
    input
        .lines()
        .map(|line| {
            let parsed = line.split_whitespace().collect::<Vec<&str>>();

            let cards: Vec<T> = parsed[0]
                .chars()
                .map(|c| T::from_char(c).unwrap())
                .collect();

            let bid = parsed[1].parse().unwrap();

            Hand::new(cards, bid)
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut hands: Vec<Hand<Card>> = parse_input(input);

    sort_hands(&mut hands);

    Some(
        hands
            .iter()
            .enumerate()
            .map(|(i, hand)| hand.bid * (i as u32 + 1))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut hands: Vec<Hand<CardWithJoker>> = parse_input(input);

    sort_hands(&mut hands);

    Some(
        hands
            .iter()
            .enumerate()
            .map(|(i, hand)| hand.bid * (i as u32 + 1))
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6592));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6839));
    }

    #[test]
    fn test_get_type_default() {
        let hand = Hand::new(
            vec![Card::Two, Card::Two, Card::Two, Card::Two, Card::Two],
            1,
        );

        assert_eq!(get_type(hand.cards), Type::FiveOfAKind(Card::Two));

        let hand = Hand::new(
            vec![Card::Two, Card::Two, Card::Two, Card::Two, Card::Three],
            1,
        );

        assert_eq!(get_type(hand.cards), Type::FourOfAKind(Card::Two));

        let hand = Hand::new(
            vec![Card::Two, Card::Two, Card::Two, Card::Three, Card::Three],
            1,
        );

        assert_eq!(get_type(hand.cards), Type::FullHouse(Card::Two));

        let hand = Hand::new(
            vec![Card::Two, Card::Two, Card::Two, Card::Three, Card::Four],
            1,
        );

        assert_eq!(get_type(hand.cards), Type::ThreeOfAKind(Card::Two));

        let hand = Hand::new(
            vec![Card::Two, Card::Two, Card::Three, Card::Three, Card::Four],
            1,
        );

        assert_eq!(get_type(hand.cards), Type::TwoPair(Card::Three, Card::Two));

        let hand = Hand::new(
            vec![Card::Two, Card::Two, Card::Three, Card::Four, Card::Five],
            1,
        );

        assert_eq!(get_type(hand.cards), Type::OnePair(Card::Two));

        let hand = Hand::new(
            vec![Card::Two, Card::Three, Card::Four, Card::Five, Card::Six],
            1,
        );

        assert_eq!(get_type(hand.cards), Type::HighCard);
    }
}
