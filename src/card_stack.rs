extern crate rand;

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

    pub fn deal_off(&mut self, n: usize) -> CardStack {
        let cards = self.cards.split_off(self.len() - n);
        CardStack{cards}
    }
}