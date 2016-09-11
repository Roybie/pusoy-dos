use cards::card::Card;
use cards::types::*;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Copy, PartialOrd)]
/// Type of hand that can be played
pub enum Move{
    /// No cards
    Pass,
    /// One card
    Single(Card),
    /// A pair of matching cards
    Pair(Card, Card),
    /// 3 of a kind
    Prial(Card, Card, Card),
    /// 5 card trick
    FiveCardTrick(Trick)
}

#[derive(Clone, Debug, PartialEq, Copy, PartialOrd)]
/// Type of 5 card trick
pub enum Trick{
    /// sequence
    Straight(Card, Card, Card, Card, Card),
    /// same suit
    Flush(Card, Card, Card, Card, Card),
    /// 3 over 2
    FullHouse(Card, Card, Card, Card, Card),
    /// 4 of same, 1 different
    FourOfAKind(Card, Card, Card, Card, Card),
    /// sequence of same suit
    StraightFlush(Card, Card, Card, Card, Card),
    /// 5 of same
    FiveOfAKind(Card, Card, Card, Card, Card)
}

/// builds a move from a Vec of cards
pub fn build_move(cards: Vec<Card>) -> Option<Move> {

    match cards.len() {
        0 => Some(Move::Pass),
        1 => Some(Move::Single(cards[0])),
        2 => check_valid_pair(cards),
        3 => check_valid_prial(cards),
        5 => check_valid_fct(cards),
        _ => None
    }
}

fn check_valid_pair(cards: Vec<Card>) -> Option<Move> {

    if cards.len() == 2 && get_counts(cards.clone()).len() == 1 {
        Some(Move::Pair(cards[0], cards[1]))
    } else {
        None
    }
}

fn check_valid_prial(cards: Vec<Card>) -> Option<Move> {

    if cards.len() == 3 && get_counts(cards.clone()).len() == 1 {
        Some(Move::Prial(cards[0], cards[1], cards[2]))
    } else {
        None
    }
}

fn check_valid_fct(cards: Vec<Card>) -> Option<Move> {

    let rank_count = get_counts(cards.clone());
    match rank_count.len() {
        1 => build_five_of_a_kind(cards),
        2 => {
           match *rank_count.values().last().unwrap() {
                3 | 2   => build_full_house(cards),
                4 | 1   => build_four_of_a_kind(cards),
                _       => None

           }
        },
        5 => {
            //flush or straight or straight flush
            let straight = cards.iter().enumerate().all(|(i, &card)| i == 0 || card.previous_rank().is_some() && cards[i-1].rank == card.previous_rank().unwrap());
            let flush = cards.iter().all(|&card| card.suit == cards[0].suit);
            match (straight, flush) {
                (true, true)    => build_straight_flush(cards),
                (true, _)       => build_straight(cards),
                (_, true)       => build_flush(cards),
                _               => None
            }
        },
        _ => None
    }
}

fn get_counts(cards: Vec<Card>) -> HashMap<Rank, usize> {
    cards.iter().fold(HashMap::new(), |mut acc, &card| {
        *acc.entry(card.rank).or_insert(0) += 1;
        acc
    })
}

fn build_four_of_a_kind(cards: Vec<Card>) -> Option<Move> {
    match cards.len() {
        5 => Some(Move::FiveCardTrick(
                    Trick::FourOfAKind(
                            cards[0],
                            cards[1],
                            cards[2],
                            cards[3],
                            cards[4]))),
        _ => None
    }
}

fn build_five_of_a_kind(cards: Vec<Card>) -> Option<Move> {
    match cards.len() {
        5 => Some(Move::FiveCardTrick(
                    Trick::FiveOfAKind(
                            cards[0],
                            cards[1],
                            cards[2],
                            cards[3],
                            cards[4]))),
        _ => None
    }
}

fn build_full_house(cards: Vec<Card>) -> Option<Move> {
    match cards.len() {
        5 => Some(Move::FiveCardTrick(
                    Trick::FullHouse(
                            cards[0],
                            cards[1],
                            cards[2],
                            cards[3],
                            cards[4]))),
        _ => None
    }
}

fn build_flush(cards: Vec<Card>) -> Option<Move> {
    match cards.len() {
        5 => Some(Move::FiveCardTrick(
                    Trick::Flush(
                            cards[0],
                            cards[1],
                            cards[2],
                            cards[3],
                            cards[4]))),
        _ => None
    }
}

fn build_straight(cards: Vec<Card>) -> Option<Move> {
    match cards.len() {
        5 => Some(Move::FiveCardTrick(
                    Trick::Straight(
                            cards[0],
                            cards[1],
                            cards[2],
                            cards[3],
                            cards[4]))),
        _ => None
    }
}

fn build_straight_flush(cards: Vec<Card>) -> Option<Move> {
    match cards.len() {
        5 => Some(Move::FiveCardTrick(
                    Trick::StraightFlush(
                            cards[0],
                            cards[1],
                            cards[2],
                            cards[3],
                            cards[4]))),
        _ => None
    }
}

