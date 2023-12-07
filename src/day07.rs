use std::cmp::Ordering;
use std::collections::BTreeMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::space1,
    character::complete::u32,
    combinator::map,
    sequence::{separated_pair, tuple},
    IResult,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Hand {
    cards: Vec<Card>,
    bid: u32,
    score: Option<Score>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
enum Score {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.score.cmp(&other.score) {
            Ordering::Equal => self.cards.cmp(&other.cards),
            other => other,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Hand {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            separated_pair(
                tuple((
                    Card::parse,
                    Card::parse,
                    Card::parse,
                    Card::parse,
                    Card::parse,
                )),
                space1,
                u32,
            ),
            |((first, second, third, fourth, fifth), bid)| Self {
                cards: vec![first, second, third, fourth, fifth],
                score: None,
                bid,
            },
        )(input)
    }

    fn score(&mut self) -> Score {
        type ScoreFn = for<'a, 'b> fn(&'a Hand, &'b [(Card, usize)]) -> Option<Score>;

        const SCORE_FNS: [ScoreFn; 7] = [
            Hand::five_of_a_kind,
            Hand::four_of_a_kind,
            Hand::full_house,
            Hand::three_of_a_kind,
            Hand::two_pair,
            Hand::one_pair,
            Hand::high_card,
        ];

        let counts = self.card_counts();

        self.score = SCORE_FNS.iter().find_map(|f| f(self, &counts));
        self.score.unwrap()
    }

    fn five_of_a_kind(&self, counts: &[(Card, usize)]) -> Option<Score> {
        counts
            .iter()
            .find(|(_, n)| *n == 5)
            .and(Some(Score::FiveOfAKind))
    }

    fn four_of_a_kind(&self, counts: &[(Card, usize)]) -> Option<Score> {
        counts
            .iter()
            .find(|(_, n)| *n == 4)
            .and(Some(Score::FourOfAKind))
    }

    fn full_house(&self, counts: &[(Card, usize)]) -> Option<Score> {
        let three = counts.iter().find(|(_, n)| *n == 3);
        let two = counts.iter().find(|(_, n)| *n == 2);

        (three.is_some() && two.is_some()).then_some(Score::FullHouse)
    }

    fn three_of_a_kind(&self, counts: &[(Card, usize)]) -> Option<Score> {
        counts
            .iter()
            .find(|(_, n)| *n == 3)
            .and(Some(Score::ThreeOfAKind))
    }

    fn two_pair(&self, counts: &[(Card, usize)]) -> Option<Score> {
        let mut owned = counts.to_owned();

        owned.retain(|&(_, n)| n == 2);

        (owned.len() == 2).then_some(Score::TwoPair)
    }

    fn one_pair(&self, counts: &[(Card, usize)]) -> Option<Score> {
        counts
            .iter()
            .find(|(_, n)| *n == 2)
            .and(Some(Score::OnePair))
    }

    fn high_card(&self, _counts: &[(Card, usize)]) -> Option<Score> {
        Some(Score::HighCard)
    }

    fn card_counts(&self) -> Vec<(Card, usize)> {
        let mut map = BTreeMap::new();

        self.cards
            .iter()
            .for_each(|card| *map.entry(*card).or_default() += 1);

        map.into_iter().collect()
    }

    fn upgrade(&mut self) {
        let orig_cards = self.cards.clone();
        self.cards.retain(|card| *card != Card::Joker);

        let n_jokers = 5 - self.cards.len();

        if n_jokers == 5 {
            self.score = Some(Score::FiveOfAKind);
            self.cards = orig_cards;
            return;
        }

        let jokerless_score = self.score();

        if n_jokers == 0 {
            return;
        }

        let new_score = match jokerless_score {
            Score::HighCard => match n_jokers {
                1 => Score::OnePair,
                2 => Score::ThreeOfAKind,
                3 => Score::FourOfAKind,
                4 => Score::FiveOfAKind,
                _ => unreachable!(),
            },
            Score::OnePair => match n_jokers {
                1 => Score::ThreeOfAKind,
                2 => Score::FourOfAKind,
                3 => Score::FiveOfAKind,
                _ => unreachable!(),
            },
            Score::TwoPair => Score::FullHouse,
            Score::ThreeOfAKind => match n_jokers {
                1 => Score::FourOfAKind,
                2 => Score::FiveOfAKind,
                _ => unreachable!(),
            },
            Score::FullHouse => {
                unreachable!()
            }
            Score::FourOfAKind => match n_jokers {
                1 => Score::FiveOfAKind,
                _ => unreachable!(),
            },
            Score::FiveOfAKind => {
                unreachable!()
            }
        };

        self.score = Some(new_score);
        self.cards = orig_cards;
    }
}

impl Card {
    fn parse(input: &str) -> IResult<&str, Self> {
        let j = if *JOKERS.read().unwrap() {
            Self::Joker
        } else {
            Self::Jack
        };

        alt((
            map(tag("A"), |_| Self::Ace),
            map(tag("K"), |_| Self::King),
            map(tag("Q"), |_| Self::Queen),
            map(tag("J"), move |_| j),
            map(tag("T"), |_| Self::Ten),
            map(tag("9"), |_| Self::Nine),
            map(tag("8"), |_| Self::Eight),
            map(tag("7"), |_| Self::Seven),
            map(tag("6"), |_| Self::Six),
            map(tag("5"), |_| Self::Five),
            map(tag("4"), |_| Self::Four),
            map(tag("3"), |_| Self::Three),
            map(tag("2"), |_| Self::Two),
        ))(input)
    }
}

static JOKERS: std::sync::RwLock<bool> = std::sync::RwLock::new(false);

fn parse(input: &str) -> Vec<Hand> {
    input
        .lines()
        .map(|line| Hand::parse(line).unwrap().1)
        .collect()
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &str) -> u32 {
    let mut input = parse(input);
    input.iter_mut().for_each(|hand| {
        hand.score();
    });
    input.sort();

    input
        .iter()
        .enumerate()
        .map(|(i, hand)| hand.bid * (i as u32 + 1))
        .sum()
}

#[aoc(day7, part2)]
pub fn solve_part2(input: &str) -> u32 {
    *JOKERS.write().unwrap() = true;

    let mut input = parse(input);

    input.iter_mut().for_each(|hand| hand.upgrade());
    input.sort();

    input
        .iter()
        .enumerate()
        .map(|(i, hand)| hand.bid * (i as u32 + 1))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "32T3K 765\n\
			   T55J5 684\n\
			   KK677 28\n\
			   KTJJT 220\n\
			   QQQJA 483";

    #[test]
    fn examples_part1() {
        assert_eq!(solve_part1(EXAMPLE), 6440);
    }

    #[test]
    fn examples_part2() {
        assert_eq!(solve_part2(EXAMPLE), 5905);
    }
}
