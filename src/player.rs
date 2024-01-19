use super::card_stack::CardStack;

pub(crate) struct Player {
    hand: CardStack,
    discards: Vec<CardStack>,
    draw_stack: CardStack,
    game_index: usize,
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
}