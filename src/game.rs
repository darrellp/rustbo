use super::card_stack::CardStack;
use super::player::Player;

pub(crate) struct Game {
    pub(crate) deck: CardStack,
    players: Vec<Player>,
    foundations: Vec<CardStack>,
    tops: Vec<u8>,
    waste: CardStack,
    cur_player: u8,
}

impl Game {
    pub fn player_count(&self) -> usize {
        self.players.len()
    }

    pub fn empty(player_count: usize) -> Self {
        let mut deck = CardStack::new_deck();
        let waste = CardStack::new();
        let foundations = vec![CardStack::new(); 4];
        let mut players = vec![];
        for i_player in 0..player_count {
            players.push(Player::deal(&mut deck, player_count, i_player))
        }

        Self {
            deck,
            players,
            foundations,
            waste,
            tops: vec![0; 4],
            cur_player: 0,
        }
    }

    pub fn empty_game(player_count: u8) -> Self {
        Self {
            deck: CardStack::new(),
            players: vec![Player::empty(); player_count as usize],
            foundations:vec![CardStack::new(); 4],
            waste: CardStack::new(),
            tops: vec![0, 0, 0, 0],
            cur_player: 0,
        }
    }

    pub fn make_play(&mut self) {
        let player = &mut self.players[self.cur_player as usize];
        player.make_play(self);
    }

    pub fn set_player(&mut self, player: Player, player_index: usize) {
        self.players[player_index] = player;
    }

    pub fn set_tops(&mut self, f1: u8, f2: u8, f3: u8, f4: u8) {
        self.tops[0] = f1;
        self.tops[1] = f2;
        self.tops[2] = f3;
        self.tops[3] = f4;
    }

    pub fn set_foundations(&mut self, f1: CardStack, f2: CardStack, f3: CardStack, f4: CardStack) {
        self.tops[0] = f1.len() as u8;
        self.foundations[0] = f1;
        self.tops[1] = f2.len() as u8;
        self.foundations[1] = f2;
        self.tops[2] = f3.len() as u8;
        self.foundations[2] = f3;
        self.tops[3] = f4.len() as u8;
        self.foundations[3] = f4;
    }

    pub fn set_deck(&mut self, deck: CardStack) {
        self.deck = deck;
    }
}