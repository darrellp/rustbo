#[cfg(test)]
mod tests {
    use crate::game::Game;
    use super::super::card_stack::CardStack;
    use super::super::card::Card;
    use super::super::player::Player;

    # [test]
    fn deck_test() {
        let deck = CardStack::new_deck();
        assert_eq!(162, deck.len());
        let mut counts = vec![0;13];
        for card in deck.cards {
            if let Card::Ranked(rank) = card {
                counts[rank as usize] += 1;
            }
            else {
                // Skipbo card
                counts[0] += 1;
            }
        }
        assert_eq!(18, counts[0]);
        for i_card in 1..=12 {
            assert_eq!(12, counts[i_card]);
        }
    }

    #[test]
    fn deal_off_test()
    {
        let mut deck = CardStack::new_deck();
        let top_card = deck.peek().unwrap();
        let deal = deck.deal_off(10);
        assert_eq!(152, deck.len());
        assert_eq!(10, deal.len());
        // Top card should be in the dealt off cards
        assert!(deal.cards.contains(&top_card));
    }

    #[test]
    fn from_string_test()
    {
        // The 1 is at the bottom and the 3 is at the top
        let arranged = CardStack::from_string("1 10 S 11 3");
        assert_eq!(arranged.nth(0), Card::Ranked(3));       // Top card
        assert_eq!(arranged.nth(1), Card::Ranked(11));
        assert_eq!(arranged.nth(2), Card::Wild);
        assert_eq!(arranged.nth(3), Card::Ranked(10));
        assert_eq!(arranged.nth(4), Card::Ranked(1));       // Bottom card
    }

    #[test]
    fn make_play_test()
    {
        let hand = CardStack::from_string("5 4 3 2 1");
        let draw_stack = CardStack::from_string("2 1");
        let player = Player {
            hand,
            discards: vec![CardStack::new(); 4],
            draw_stack,
            game_index: 0
        };

        let mut game = Game::empty(1);
        game.set_player(player, 0);

    }
}