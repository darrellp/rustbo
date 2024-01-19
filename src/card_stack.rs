extern crate rand;
use std::str::FromStr;
use rand::prelude::SliceRandom;
use super::card::*;
use rand::thread_rng;

#[derive(Clone)]
pub(crate) struct CardStack {
    pub(crate) cards: Vec<Card>,
}

impl CardStack {
    pub fn new_deck() -> Self {
        let mut ret = vec![];


        for i_rank in 1..=12 {
            let mut rank_cards = vec![Card::Ranked(i_rank as u8);12];
            ret.append(&mut rank_cards);
        }
        let mut wilds = vec![Card::Wild; 18];
        ret.append(&mut wilds);
        let mut rng = thread_rng();
        ret.shuffle(&mut rng);
        Self {
            cards: ret
        }
    }

    pub fn new() -> Self {
        let cards = Vec::<Card>::new();
        Self { cards }
    }

    // Takes string of format "1 2 S 10 12" and produces a stack with the corresponding cards.  The
    // leftmost element of the string is the bottom card and the one on the right is the top card.
    pub fn from_string(input: &str) -> Self {
        let mut cards = vec![];
        let mut card_strings = input.split(' ');
        for card_rep in card_strings.by_ref() {
            if card_rep.starts_with('S') {
                cards.push(Card::Wild);
            }
            else {
                let rank =str::parse::<u8>(card_rep).unwrap();
                cards.push(Card::Ranked(rank));
            }
        }
        Self { cards }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.cards.len()
    }

    #[inline]
    pub fn pop(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    #[inline]
    pub fn peek(&self) -> Option<Card> {
        if self.len() == 0 {
            None
        }
        else {
            Some(self.cards[self.len() - 1])
        }
    }

    // nth(0) returns the same as peek() - i.e., the top card of the deck
    #[inline]
    pub fn nth(&self, n: usize) -> Card {
        if n >= self.len() {
            panic!();
        }
        self.cards[self.len() - n - 1]
    }

    pub fn deal_off(&mut self, n: usize) -> CardStack {
        let cards = self.cards.split_off(self.len() - n);
        CardStack{cards}
    }
}