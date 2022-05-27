use rand::{prelude::SliceRandom, thread_rng};

use super::card::Card;

pub(crate) struct CardStorage<C>
where
    C: Card,
{
    deck: Vec<C>,
    discard_pile: Vec<C>,
}

impl<C> CardStorage<C>
where
    C: Card,
{
    pub(crate) fn new() -> Self {
        Self {
            deck: vec![],
            discard_pile: vec![],
        }
    }

    pub(crate) fn num_cards_total(&self) -> usize {
        self.deck.len() + self.discard_pile.len()
    }

    pub(crate) fn add_card_to_deck(&mut self, card: C) {
        self.deck.push(card);
    }

    pub(crate) fn discard_card(&mut self, card: C) {
        self.discard_pile.push(card);
    }

    pub(crate) fn draw_card_from_deck(&mut self) -> Option<C> {
        self.deck.pop()
    }

    pub(crate) fn shuffle_deck(&mut self) {
        self.deck.shuffle(&mut thread_rng());
    }

    pub(crate) fn refill_deck_and_shuffle(&mut self) {
        while !self.discard_pile.is_empty() {
            let card = self.discard_pile.pop().unwrap();
            self.add_card_to_deck(card);
        }
        self.shuffle_deck()
    }
}

mod tests {
    use super::*;

    #[derive(Default)]
    struct DummyCard {
        id: usize,
    }
    impl Card for DummyCard {}

    #[test]
    fn new_card_storage() {
        let card_storage = CardStorage::<DummyCard>::new();

        assert_eq!(card_storage.num_cards_total(), 0);
    }

    #[test]
    fn card_storage_of_string_doesnt_work() {
        // The line below does not compile.
        // let card_storage = CardStorage::<String>::new();
    }

    #[test]
    fn add_card_to_deck() {
        let mut card_storage = CardStorage::new();
        let card = DummyCard::default();

        card_storage.add_card_to_deck(card);
        assert_eq!(card_storage.num_cards_total(), 1);
    }

    #[test]
    fn discard_a_card() {
        let mut card_storage = CardStorage::new();
        let card = DummyCard::default();

        card_storage.discard_card(card);
        assert_eq!(card_storage.num_cards_total(), 1);
    }

    #[test]
    fn draw_a_card() {
        let mut card_storage = CardStorage::new();
        card_storage.add_card_to_deck(DummyCard { id: 0 });
        card_storage.add_card_to_deck(DummyCard { id: 1 });

        let drawn_card = card_storage.draw_card_from_deck().unwrap();
        assert_eq!(drawn_card.id, 1);
    }

    #[test]
    fn draw_a_card_when_deck_is_empty() {
        let mut card_storage = CardStorage::<DummyCard>::new();

        let draw_result = card_storage.draw_card_from_deck();
        assert!(draw_result.is_none());
    }

    #[test]
    fn shuffle_deck() {
        let mut card_storage = CardStorage::new();
        let range = 0..=1000;
        range.clone().for_each(|id| {
            let card = DummyCard { id };
            card_storage.add_card_to_deck(card);
        });

        card_storage.shuffle_deck();

        for i in range.rev() {
            let drawn_card = card_storage.draw_card_from_deck().unwrap();
            if drawn_card.id != i {
                return;
            }
        }

        panic!("1000 cards are not shuffled");
    }

    #[test]
    fn refill_deck() {
        let mut card_storage = CardStorage::new();
        (0..=1000).for_each(|id| {
            let card = DummyCard { id };
            card_storage.discard_card(card);
        });

        let drawn_card = card_storage.draw_card_from_deck();
        assert!(drawn_card.is_none());

        card_storage.refill_deck_and_shuffle();

        let drawn_card = card_storage.draw_card_from_deck();
        assert!(drawn_card.is_some());
    }
}