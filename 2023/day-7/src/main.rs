use std::{cmp::Ordering, collections::BTreeMap};

fn main() {
    let input = include_str!("./input.txt");

    let game = Game::<StandardVariant>::parse(input);

    let part_1 = game.calculate_total_winnings_idx_order_style();
    dbg!(part_1);

    let game = Game::<JokerVariant>::parse(input);

    let part_2 = game.calculate_total_winnings_idx_order_style();
    dbg!(part_2);
}

pub trait Card {
    fn parse(input: char) -> Result<Self, String>
    where
        Self: Sized;

    fn value(&self) -> usize;
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum StandardVariant {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}

impl Card for StandardVariant {
    fn parse(input: char) -> Result<Self, String>
    where
        Self: Sized,
    {
        match input {
            'A' => Ok(Self::Ace),
            'K' => Ok(Self::King),
            'Q' => Ok(Self::Queen),
            'J' => Ok(Self::Jack),
            'T' => Ok(Self::Ten),
            '9' => Ok(Self::Nine),
            '8' => Ok(Self::Eight),
            '7' => Ok(Self::Seven),
            '6' => Ok(Self::Six),
            '5' => Ok(Self::Five),
            '4' => Ok(Self::Four),
            '3' => Ok(Self::Three),
            '2' => Ok(Self::Two),
            _ => Err("Failed to parse".to_string()),
        }
    }

    fn value(&self) -> usize {
        match self {
            Self::Ace => 13,
            Self::King => 12,
            Self::Queen => 11,
            Self::Jack => 10,
            Self::Ten => 9,
            Self::Nine => 8,
            Self::Eight => 7,
            Self::Seven => 6,
            Self::Six => 5,
            Self::Five => 4,
            Self::Four => 3,
            Self::Three => 2,
            Self::Two => 1,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum JokerVariant {
    Ace,
    King,
    Queen,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    Joker,
}

impl Card for JokerVariant {
    fn parse(input: char) -> Result<Self, String> {
        match input {
            'A' => Ok(Self::Ace),
            'K' => Ok(Self::King),
            'Q' => Ok(Self::Queen),
            'T' => Ok(Self::Ten),
            '9' => Ok(Self::Nine),
            '8' => Ok(Self::Eight),
            '7' => Ok(Self::Seven),
            '6' => Ok(Self::Six),
            '5' => Ok(Self::Five),
            '4' => Ok(Self::Four),
            '3' => Ok(Self::Three),
            '2' => Ok(Self::Two),
            'J' => Ok(Self::Joker),
            _ => Err("Failed to parse".to_string()),
        }
    }

    fn value(&self) -> usize {
        match self {
            Self::Ace => 13,
            Self::King => 12,
            Self::Queen => 11,
            Self::Ten => 10,
            Self::Nine => 9,
            Self::Eight => 8,
            Self::Seven => 7,
            Self::Six => 6,
            Self::Five => 5,
            Self::Four => 4,
            Self::Three => 3,
            Self::Two => 2,
            Self::Joker => 1,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandKind {
    FiveOfKind,
    FourOfKind,
    FullHouse,
    ThreeOfKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandKind {
    pub fn value(&self) -> usize {
        match self {
            Self::FiveOfKind => 7,
            Self::FourOfKind => 6,
            Self::FullHouse => 5,
            Self::ThreeOfKind => 4,
            Self::TwoPair => 3,
            Self::OnePair => 2,
            Self::HighCard => 1,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Hand<T> {
    pub cards: Vec<T>,
    pub kind: HandKind,
    pub kind_order: Vec<T>,
    pub bid: usize,
}

impl Hand<StandardVariant> {
    pub fn parse(input: &str) -> Self {
        let mut input_parts = input.split_ascii_whitespace();

        let cards: Vec<StandardVariant> = input_parts
            .next()
            .unwrap()
            .chars()
            .map(|char| StandardVariant::parse(char).unwrap())
            .collect();

        let card_map: BTreeMap<StandardVariant, usize> =
            cards.iter().fold(BTreeMap::new(), |mut acc, card| {
                if let Some(value) = acc.get_mut(card) {
                    *value += 1;
                } else {
                    acc.insert(*card, 1);
                }
                acc
            });

        let mut counted_cards = Vec::from_iter(card_map);

        counted_cards.sort_by(|&(a_card, a_count), &(b_card, b_count)| {
            if b_card == a_card {
                b_card.cmp(&a_card)
            } else {
                b_count.cmp(&a_count)
            }
        });

        let (kind_order, ordered_count): (Vec<_>, Vec<_>) = counted_cards.into_iter().unzip();

        let kind = match ordered_count[0] {
            5 => HandKind::FiveOfKind,
            4 => HandKind::FourOfKind,
            3 => {
                if ordered_count[1] == 1 {
                    HandKind::ThreeOfKind
                } else {
                    HandKind::FullHouse
                }
            }
            2 => {
                if ordered_count[1] == 1 {
                    HandKind::OnePair
                } else {
                    HandKind::TwoPair
                }
            }
            1 => HandKind::HighCard,
            _ => unreachable!(),
        };

        let bid = input_parts.next().unwrap().parse::<usize>().unwrap();

        Self {
            cards,
            kind,
            kind_order,
            bid,
        }
    }
}

impl Hand<JokerVariant> {
    pub fn parse(input: &str) -> Self {
        let mut input_parts = input.split_ascii_whitespace();

        let cards: Vec<JokerVariant> = input_parts
            .next()
            .unwrap()
            .chars()
            .map(|char| JokerVariant::parse(char).unwrap())
            .collect();

        let mut card_map: BTreeMap<JokerVariant, usize> =
            cards.iter().fold(BTreeMap::new(), |mut acc, card| {
                if let Some(value) = acc.get_mut(card) {
                    *value += 1;
                } else {
                    acc.insert(*card, 1);
                }
                acc
            });
        
        let mut counted_cards = Vec::from_iter(card_map.clone());
        
        counted_cards.sort_by(|&(a_card, a_count), &(b_card, b_count)| {
            if b_card == a_card {
                b_card.cmp(&a_card)
            } else {
                b_count.cmp(&a_count)
            }
        });
        
        let stuff = if let Some(joker_count) = card_map.get(&JokerVariant::Joker) {
            if let Some((card, _count)) = counted_cards.iter().find(|(card, _count)| *card != JokerVariant::Joker) {
                Some((*card, *joker_count))
            } else {
                None
            }
        } else {
            None
        };
        
        if let Some((highest_card, count)) = stuff {
            if let Some(t) = card_map.get_mut(&highest_card) {
                *t += count;
            }
            card_map.remove(&JokerVariant::Joker);
        }
        
        let mut counted_cards = Vec::from_iter(card_map.clone());
        
        counted_cards.sort_by(|&(a_card, a_count), &(b_card, b_count)| {
            if b_card == a_card {
                b_card.cmp(&a_card)
            } else {
                b_count.cmp(&a_count)
            }
        });
        
        let (kind_order, ordered_count): (Vec<_>, Vec<_>) = counted_cards.into_iter().unzip();
        
        let kind = match ordered_count[0] {
            5 => HandKind::FiveOfKind,
            4 => HandKind::FourOfKind,
            3 => {
                if ordered_count[1] == 1 {
                    HandKind::ThreeOfKind
                } else {
                    HandKind::FullHouse
                }
            }
            2 => {
                if ordered_count[1] == 1 {
                    HandKind::OnePair
                } else {
                    HandKind::TwoPair
                }
            }
            1 => HandKind::HighCard,
            _ => unreachable!(),
        };

        let bid = input_parts.next().unwrap().parse::<usize>().unwrap();

        Self {
            cards,
            kind,
            kind_order,
            bid,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Game<T> {
    pub hands: Vec<Hand<T>>,
}

impl Game<StandardVariant> {
    pub fn parse(input: &str) -> Self {
        let hands = input.lines().map(Hand::<StandardVariant>::parse).collect();
        Self { hands }
    }

    pub fn rank_hands_poker_style(&self) -> Vec<Hand<StandardVariant>> {
        let mut hands = self.hands.clone();

        hands.sort_by(|a, b| {
            if a.kind == b.kind {
                let mut ord = Ordering::Equal;

                for (idx, card) in b.kind_order.iter().enumerate() {
                    let other = a.kind_order[idx];
                    let cmp = card.cmp(&other);
                    if cmp != Ordering::Equal {
                        ord = cmp;
                        break;
                    }
                }

                ord
            } else {
                a.kind.value().cmp(&b.kind.value())
            }
        });

        hands
    }

    pub fn rank_hands_idx_order_style(&self) -> Vec<Hand<StandardVariant>> {
        let mut hands = self.hands.clone();

        hands.sort_by(|a, b| {
            if a.kind == b.kind {
                let mut ord = Ordering::Equal;

                for (idx, card) in b.cards.iter().enumerate() {
                    let other = a.cards[idx];
                    let cmp = card.cmp(&other);
                    if cmp != Ordering::Equal {
                        ord = cmp;
                        break;
                    }
                }

                ord
            } else {
                a.kind.value().cmp(&b.kind.value())
            }
        });

        hands
    }

    pub fn calculate_total_winnings_idx_order_style(&self) -> usize {
        self.rank_hands_idx_order_style()
            .iter()
            .enumerate()
            .fold(0, |acc, (idx, hand)| acc + ((idx + 1) * hand.bid))
    }
}

impl Game<JokerVariant> {
    pub fn parse(input: &str) -> Self {
        let hands = input.lines().map(Hand::<JokerVariant>::parse).collect();
        Self { hands }
    }

    pub fn rank_hands_poker_style(&self) -> Vec<Hand<JokerVariant>> {
        let mut hands = self.hands.clone();

        hands.sort_by(|a, b| {
            if a.kind == b.kind {
                let mut ord = Ordering::Equal;

                for (idx, card) in b.kind_order.iter().enumerate() {
                    let other = a.kind_order[idx];
                    let cmp = card.cmp(&other);
                    if cmp != Ordering::Equal {
                        ord = cmp;
                        break;
                    }
                }

                ord
            } else {
                a.kind.value().cmp(&b.kind.value())
            }
        });

        hands
    }

    pub fn rank_hands_idx_order_style(&self) -> Vec<Hand<JokerVariant>> {
        let mut hands = self.hands.clone();

        hands.sort_by(|a, b| {
            if a.kind == b.kind {
                let mut ord = Ordering::Equal;

                for (idx, card) in b.cards.iter().enumerate() {
                    let other = a.cards[idx];
                    let cmp = card.cmp(&other);
                    if cmp != Ordering::Equal {
                        ord = cmp;
                        break;
                    }
                }

                ord
            } else {
                a.kind.value().cmp(&b.kind.value())
            }
        });

        hands
    }

    pub fn calculate_total_winnings_idx_order_style(&self) -> usize {
        self.rank_hands_idx_order_style()
            .iter()
            .enumerate()
            .fold(0, |acc, (idx, hand)| acc + ((idx + 1) * hand.bid))
    }
}

mod tests {
    use crate::Card;

    static EXAMPLE1: &str = "32T3K 765\nT55J5 684\nKK677 28\nKTJJT 220\nQQQJA 483";

    #[test]
    fn should_parse_card() {
        assert_eq!(
            crate::StandardVariant::parse('T').unwrap(),
            crate::StandardVariant::Ten
        )
    }

    #[test]
    fn should_parse_hand() {
        assert_eq!(
            crate::Hand::<crate::StandardVariant>::parse("32T3K 765"),
            crate::Hand {
                cards: vec![
                    crate::StandardVariant::Three,
                    crate::StandardVariant::Two,
                    crate::StandardVariant::Ten,
                    crate::StandardVariant::Three,
                    crate::StandardVariant::King,
                ],
                kind: crate::HandKind::OnePair,
                kind_order: vec![
                    crate::StandardVariant::Three,
                    crate::StandardVariant::King,
                    crate::StandardVariant::Ten,
                    crate::StandardVariant::Two
                ],
                bid: 765
            },
        )
    }

    #[test]
    fn should_parse_game() {
        assert_eq!(
            crate::Game::<crate::StandardVariant>::parse(EXAMPLE1),
            crate::Game {
                hands: vec![
                    crate::Hand {
                        cards: vec![
                            crate::StandardVariant::Three,
                            crate::StandardVariant::Two,
                            crate::StandardVariant::Ten,
                            crate::StandardVariant::Three,
                            crate::StandardVariant::King,
                        ],
                        kind: crate::HandKind::OnePair,
                        kind_order: vec![
                            crate::StandardVariant::Three,
                            crate::StandardVariant::King,
                            crate::StandardVariant::Ten,
                            crate::StandardVariant::Two
                        ],
                        bid: 765
                    },
                    crate::Hand {
                        cards: vec![
                            crate::StandardVariant::Ten,
                            crate::StandardVariant::Five,
                            crate::StandardVariant::Five,
                            crate::StandardVariant::Jack,
                            crate::StandardVariant::Five,
                        ],
                        kind: crate::HandKind::ThreeOfKind,
                        kind_order: vec![
                            crate::StandardVariant::Five,
                            crate::StandardVariant::Jack,
                            crate::StandardVariant::Ten
                        ],
                        bid: 684
                    },
                    crate::Hand {
                        cards: vec![
                            crate::StandardVariant::King,
                            crate::StandardVariant::King,
                            crate::StandardVariant::Six,
                            crate::StandardVariant::Seven,
                            crate::StandardVariant::Seven,
                        ],
                        kind: crate::HandKind::TwoPair,
                        kind_order: vec![
                            crate::StandardVariant::King,
                            crate::StandardVariant::Seven,
                            crate::StandardVariant::Six
                        ],
                        bid: 28
                    },
                    crate::Hand {
                        cards: vec![
                            crate::StandardVariant::King,
                            crate::StandardVariant::Ten,
                            crate::StandardVariant::Jack,
                            crate::StandardVariant::Jack,
                            crate::StandardVariant::Ten,
                        ],
                        kind: crate::HandKind::TwoPair,
                        kind_order: vec![
                            crate::StandardVariant::Jack,
                            crate::StandardVariant::Ten,
                            crate::StandardVariant::King
                        ],
                        bid: 220
                    },
                    crate::Hand {
                        cards: vec![
                            crate::StandardVariant::Queen,
                            crate::StandardVariant::Queen,
                            crate::StandardVariant::Queen,
                            crate::StandardVariant::Jack,
                            crate::StandardVariant::Ace,
                        ],
                        kind: crate::HandKind::ThreeOfKind,
                        kind_order: vec![
                            crate::StandardVariant::Queen,
                            crate::StandardVariant::Ace,
                            crate::StandardVariant::Jack
                        ],
                        bid: 483
                    },
                ]
            }
        );
    }

    #[test]
    fn should_rank_hands() {
        assert_eq!(
            crate::Game::<crate::StandardVariant>::parse(EXAMPLE1).rank_hands_poker_style(),
            vec![
                crate::Hand {
                    cards: vec![
                        crate::StandardVariant::Three,
                        crate::StandardVariant::Two,
                        crate::StandardVariant::Ten,
                        crate::StandardVariant::Three,
                        crate::StandardVariant::King,
                    ],
                    kind: crate::HandKind::OnePair,
                    kind_order: vec![
                        crate::StandardVariant::Three,
                        crate::StandardVariant::King,
                        crate::StandardVariant::Ten,
                        crate::StandardVariant::Two
                    ],
                    bid: 765
                },
                crate::Hand {
                    cards: vec![
                        crate::StandardVariant::King,
                        crate::StandardVariant::Ten,
                        crate::StandardVariant::Jack,
                        crate::StandardVariant::Jack,
                        crate::StandardVariant::Ten,
                    ],
                    kind: crate::HandKind::TwoPair,
                    kind_order: vec![
                        crate::StandardVariant::Jack,
                        crate::StandardVariant::Ten,
                        crate::StandardVariant::King
                    ],
                    bid: 220
                },
                crate::Hand {
                    cards: vec![
                        crate::StandardVariant::King,
                        crate::StandardVariant::King,
                        crate::StandardVariant::Six,
                        crate::StandardVariant::Seven,
                        crate::StandardVariant::Seven,
                    ],
                    kind: crate::HandKind::TwoPair,
                    kind_order: vec![
                        crate::StandardVariant::King,
                        crate::StandardVariant::Seven,
                        crate::StandardVariant::Six
                    ],
                    bid: 28
                },
                crate::Hand {
                    cards: vec![
                        crate::StandardVariant::Ten,
                        crate::StandardVariant::Five,
                        crate::StandardVariant::Five,
                        crate::StandardVariant::Jack,
                        crate::StandardVariant::Five,
                    ],
                    kind: crate::HandKind::ThreeOfKind,
                    kind_order: vec![
                        crate::StandardVariant::Five,
                        crate::StandardVariant::Jack,
                        crate::StandardVariant::Ten
                    ],
                    bid: 684
                },
                crate::Hand {
                    cards: vec![
                        crate::StandardVariant::Queen,
                        crate::StandardVariant::Queen,
                        crate::StandardVariant::Queen,
                        crate::StandardVariant::Jack,
                        crate::StandardVariant::Ace,
                    ],
                    kind: crate::HandKind::ThreeOfKind,
                    kind_order: vec![
                        crate::StandardVariant::Queen,
                        crate::StandardVariant::Ace,
                        crate::StandardVariant::Jack
                    ],
                    bid: 483
                },
            ]
        );
    }

    #[test]
    fn should_calculate_total_winnings_for_standard_variant() {
        assert_eq!(
            crate::Game::<crate::StandardVariant>::parse(EXAMPLE1)
                .calculate_total_winnings_idx_order_style(),
            6440
        );
    }

    #[test]
    fn should_calculate_total_winnings_for_joker_variant() {
        assert_eq!(
            crate::Game::<crate::JokerVariant>::parse(EXAMPLE1)
                .calculate_total_winnings_idx_order_style(),
            5905
        );
    }
}
