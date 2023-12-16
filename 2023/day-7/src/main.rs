use std::{cmp::Ordering, collections::BTreeMap};

fn main() {
    let input = include_str!("./input.txt");

    let game = Game::parse(input);

    let part_1 = game.calculate_total_winnings_idx_order_style();
    dbg!(part_1);
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Card {
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

impl Card {
    pub fn parse(input: char) -> Result<Self, String> {
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

    pub fn value(&self) -> usize {
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
pub struct Hand {
    pub cards: Vec<Card>,
    pub kind: HandKind,
    pub kind_order: Vec<Card>,
    pub bid: usize,
}

impl Hand {
    pub fn parse(input: &str) -> Self {
        let mut input_parts = input.split_ascii_whitespace();

        let cards: Vec<Card> = input_parts
            .next()
            .unwrap()
            .chars()
            .map(|char| Card::parse(char).unwrap())
            .collect();

        let card_map: BTreeMap<Card, usize> =
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

#[derive(Clone, Debug, PartialEq)]
pub struct Game {
    pub hands: Vec<Hand>,
}

impl Game {
    pub fn parse(input: &str) -> Self {
        let hands = input.lines().map(Hand::parse).collect();
        Self { hands }
    }

    pub fn rank_hands_poker_style(&self) -> Vec<Hand> {
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

    pub fn calculate_total_winnings_poker_style(&self) -> usize {
        let ranked_hands = self.rank_hands_poker_style();
        // dbg!(&ranked_hands);
        ranked_hands
            .iter()
            .enumerate()
            .fold(0, |acc, (idx, hand)| {
                println!("idx: {}, bid: {}, cards: {:?}, type: {:?}, order: {:?}", idx + 1, hand.bid, hand.cards, hand.kind, hand.kind_order);
                println!("{}", acc);
                acc + ((idx + 1) * hand.bid)
            })
    }

    pub fn rank_hands_idx_order_style(&self) -> Vec<Hand> {
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
        let ranked_hands = self.rank_hands_idx_order_style();
        // dbg!(&ranked_hands);
        ranked_hands
            .iter()
            .enumerate()
            .fold(0, |acc, (idx, hand)| {
                println!("idx: {}, bid: {}, cards: {:?}, type: {:?}, order: {:?}", idx + 1, hand.bid, hand.cards, hand.kind, hand.kind_order);
                println!("{}", acc);
                acc + ((idx + 1) * hand.bid)
            })
    }
}

mod tests {
    static EXAMPLE1: &str = "32T3K 765\nT55J5 684\nKK677 28\nKTJJT 220\nQQQJA 483";

    #[test]
    fn should_parse_card() {
        assert_eq!(crate::Card::parse('T').unwrap(), crate::Card::Ten)
    }

    #[test]
    fn should_parse_hand() {
        assert_eq!(
            crate::Hand::parse("32T3K 765"),
            crate::Hand {
                cards: vec![
                    crate::Card::Three,
                    crate::Card::Two,
                    crate::Card::Ten,
                    crate::Card::Three,
                    crate::Card::King,
                ],
                kind: crate::HandKind::OnePair,
                kind_order: vec![
                    crate::Card::Three,
                    crate::Card::King,
                    crate::Card::Ten,
                    crate::Card::Two
                ],
                bid: 765
            },
        )
    }

    #[test]
    fn should_parse_game() {
        assert_eq!(
            crate::Game::parse(EXAMPLE1),
            crate::Game {
                hands: vec![
                    crate::Hand {
                        cards: vec![
                            crate::Card::Three,
                            crate::Card::Two,
                            crate::Card::Ten,
                            crate::Card::Three,
                            crate::Card::King,
                        ],
                        kind: crate::HandKind::OnePair,
                        kind_order: vec![
                            crate::Card::Three,
                            crate::Card::King,
                            crate::Card::Ten,
                            crate::Card::Two
                        ],
                        bid: 765
                    },
                    crate::Hand {
                        cards: vec![
                            crate::Card::Ten,
                            crate::Card::Five,
                            crate::Card::Five,
                            crate::Card::Jack,
                            crate::Card::Five,
                        ],
                        kind: crate::HandKind::ThreeOfKind,
                        kind_order: vec![crate::Card::Five, crate::Card::Jack, crate::Card::Ten],
                        bid: 684
                    },
                    crate::Hand {
                        cards: vec![
                            crate::Card::King,
                            crate::Card::King,
                            crate::Card::Six,
                            crate::Card::Seven,
                            crate::Card::Seven,
                        ],
                        kind: crate::HandKind::TwoPair,
                        kind_order: vec![crate::Card::King, crate::Card::Seven, crate::Card::Six],
                        bid: 28
                    },
                    crate::Hand {
                        cards: vec![
                            crate::Card::King,
                            crate::Card::Ten,
                            crate::Card::Jack,
                            crate::Card::Jack,
                            crate::Card::Ten,
                        ],
                        kind: crate::HandKind::TwoPair,
                        kind_order: vec![crate::Card::Jack, crate::Card::Ten, crate::Card::King],
                        bid: 220
                    },
                    crate::Hand {
                        cards: vec![
                            crate::Card::Queen,
                            crate::Card::Queen,
                            crate::Card::Queen,
                            crate::Card::Jack,
                            crate::Card::Ace,
                        ],
                        kind: crate::HandKind::ThreeOfKind,
                        kind_order: vec![crate::Card::Queen, crate::Card::Ace, crate::Card::Jack],
                        bid: 483
                    },
                ]
            }
        );
    }

    #[test]
    fn should_rank_hands() {
        assert_eq!(
            crate::Game::parse(EXAMPLE1).rank_hands_poker_style(),
            vec![
                crate::Hand {
                    cards: vec![
                        crate::Card::Three,
                        crate::Card::Two,
                        crate::Card::Ten,
                        crate::Card::Three,
                        crate::Card::King,
                    ],
                    kind: crate::HandKind::OnePair,
                    kind_order: vec![
                        crate::Card::Three,
                        crate::Card::King,
                        crate::Card::Ten,
                        crate::Card::Two
                    ],
                    bid: 765
                },
                crate::Hand {
                    cards: vec![
                        crate::Card::King,
                        crate::Card::Ten,
                        crate::Card::Jack,
                        crate::Card::Jack,
                        crate::Card::Ten,
                    ],
                    kind: crate::HandKind::TwoPair,
                    kind_order: vec![crate::Card::Jack, crate::Card::Ten, crate::Card::King],
                    bid: 220
                },
                crate::Hand {
                    cards: vec![
                        crate::Card::King,
                        crate::Card::King,
                        crate::Card::Six,
                        crate::Card::Seven,
                        crate::Card::Seven,
                    ],
                    kind: crate::HandKind::TwoPair,
                    kind_order: vec![crate::Card::King, crate::Card::Seven, crate::Card::Six],
                    bid: 28
                },
                crate::Hand {
                    cards: vec![
                        crate::Card::Ten,
                        crate::Card::Five,
                        crate::Card::Five,
                        crate::Card::Jack,
                        crate::Card::Five,
                    ],
                    kind: crate::HandKind::ThreeOfKind,
                    kind_order: vec![crate::Card::Five, crate::Card::Jack, crate::Card::Ten],
                    bid: 684
                },
                crate::Hand {
                    cards: vec![
                        crate::Card::Queen,
                        crate::Card::Queen,
                        crate::Card::Queen,
                        crate::Card::Jack,
                        crate::Card::Ace,
                    ],
                    kind: crate::HandKind::ThreeOfKind,
                    kind_order: vec![crate::Card::Queen, crate::Card::Ace, crate::Card::Jack],
                    bid: 483
                },
            ]
        );
    }

    #[test]
    fn should_calculate_total_winnings() {
        assert_eq!(
            crate::Game::parse(EXAMPLE1).calculate_total_winnings_poker_style(),
            6440
        );
    }
}
