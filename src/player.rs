use crate::game::Game;
use super::card_stack::CardStack;

#[derive(Clone)]
pub(crate) struct Player {
    pub(crate) hand: CardStack,
    pub(crate) discards: Vec<CardStack>,
    pub(crate) draw_stack: CardStack,
    pub(crate) game_index: usize,
}


impl Player {
    pub fn deal(deck: &mut CardStack, player_count: usize, index: usize) -> Self {
        let draw_stack_size = if player_count <= 4_usize { 30_usize } else { 20_usize };
        let draw_stack = deck.deal_off(draw_stack_size);
        let game_index = index;
        let discards = vec![CardStack::new(); 4];
        let hand = CardStack::new();
        Self {
            hand,
            discards,
            draw_stack,
            game_index
        }
    }

    pub fn empty() -> Self {
        Self {
            hand: CardStack::new(),
            discards: vec![CardStack::new();4],
            draw_stack: CardStack::new(),
            game_index: 0
        }
    }

    pub fn make_play(&mut self, game: &mut Game) {

    }
}