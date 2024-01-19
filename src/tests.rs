#[cfg(test)]
mod tests {
    use super::super::card_stack::CardStack;
    use super::super::card::Card;

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
}