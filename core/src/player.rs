use std::collections::HashSet;

use crate::{cards::AnswerCard, errors::GameCoreError};

pub(crate) struct Player {
    awesome_points: i32,
    hand: Vec<AnswerCard>,
}

impl Player {
    pub(crate) fn new() -> Self {
        Self {
            awesome_points: 0,
            hand: vec![],
        }
    }

    pub(crate) fn awesome_points(&self) -> i32 {
        self.awesome_points
    }

    pub(crate) fn increment_awesome_points(&mut self) {
        self.awesome_points += 1;
    }

    pub(crate) fn hand_size(&self) -> usize {
        self.hand.len()
    }

    pub(crate) fn add_card_to_hand(&mut self, card: AnswerCard) {
        self.hand.push(card);
    }

    /// `indices` are ZERO-based indices of the hand.
    pub(crate) fn play_cards(
        &mut self,
        indices: &[usize],
    ) -> Result<Vec<AnswerCard>, GameCoreError> {
        let mut ind_set = HashSet::new();
        for &ind in indices.iter() {
            if ind >= self.hand.len() {
                return Err(GameCoreError::PlayerChoosingCardOutOfHandBound {
                    chosen_ind: ind,
                    hand_bound: self.hand.len(),
                });
            }
            if ind_set.contains(&ind) {
                return Err(GameCoreError::PlayerChoosingTheSameCardMultipleTimes {
                    chosen_ind: ind,
                });
            }
            ind_set.insert(ind);
        }

        let played_cards = indices
            .iter()
            .map(|&ind| self.hand[ind].clone())
            .collect::<Vec<_>>();

        self.hand = self
            .hand
            .iter()
            .enumerate()
            .filter_map(|(i, card)| {
                if ind_set.contains(&i) {
                    None
                } else {
                    Some(card.clone())
                }
            })
            .collect();

        Ok(played_cards)
    }
}

mod tests {
    use super::*;

    #[test]
    fn new() {
        let player = Player::new();

        assert_eq!(player.awesome_points(), 0);
        assert_eq!(player.hand_size(), 0);
    }

    #[test]
    fn increment_awesome_points() {
        let mut player = Player::new();
        player.increment_awesome_points();

        assert_eq!(player.awesome_points(), 1);
    }

    #[test]
    fn add_card_to_hand() {
        let mut player = Player::new();

        player.add_card_to_hand(AnswerCard::new("42"));
        assert_eq!(player.hand_size(), 1);
    }

    #[test]
    fn play_cards() {
        let mut player = Player::new();

        for i in 0..10 {
            player.add_card_to_hand(AnswerCard::new(i.to_string()));
        }

        let played_cards = player.play_cards(&[0, 6, 4]).unwrap();

        assert_eq!(played_cards.len(), 3);
        assert_eq!(played_cards[0].content, "0");
        assert_eq!(played_cards[1].content, "6");
        assert_eq!(played_cards[2].content, "4");

        assert_eq!(player.hand_size(), 7);
    }

    #[test]
    fn play_cards_out_of_bounds() {
        let mut player = Player::new();

        for i in 0..10 {
            player.add_card_to_hand(AnswerCard::new(i.to_string()));
        }

        let play_cards_result = player.play_cards(&[0, 10, 4]);

        assert_eq!(
            play_cards_result.err().unwrap(),
            GameCoreError::PlayerChoosingCardOutOfHandBound {
                chosen_ind: 10,
                hand_bound: 10
            }
        );

        assert_eq!(
            player.hand_size(),
            10,
            "Player's cards are played even though there's an error."
        );
    }

    #[test]
    fn play_same_cards_multiple_times() {
        let mut player = Player::new();

        for i in 0..10 {
            player.add_card_to_hand(AnswerCard::new(i.to_string()));
        }

        let play_cards_result = player.play_cards(&[0, 0, 4]);

        assert_eq!(
            play_cards_result.err().unwrap(),
            GameCoreError::PlayerChoosingTheSameCardMultipleTimes { chosen_ind: 0 }
        );

        assert_eq!(
            player.hand_size(),
            10,
            "Player's cards are played even though there's an error."
        );
    }
}
