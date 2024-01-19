use super::card_stack::CardStack;
use super::player::Player;
pub(crate) struct Game {
    pub(crate) deck: CardStack,
    players: Vec<Player>,
    foundations: Vec<CardStack>
}

impl Game {
    pub fn player_count(&self) -> usize {
        self.players.len()
    }

    pub fn new(player_count: usize) -> Self {
        let mut deck = CardStack::new_deck();
        let foundations = vec![CardStack::new(); 4];
        let mut players = vec![];
        for i_player in 0..player_count {
            players.push(Player::deal(&mut deck, player_count, i_player))
        }

        Self {
            deck,
            players,
            foundations,
        }
    }
}