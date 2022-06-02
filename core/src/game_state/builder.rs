use std::{
    collections::{HashMap, HashSet},
    iter::FromIterator,
};

use crate::{cards::CardStorage, error::GameCoreError, player::Player};

use super::{GameState, PlayerName};

#[derive(Default)]
pub struct GameStateBuilder<PN = String>
where
    PN: PlayerName,
{
    players: HashSet<PN>,
    questions: HashSet<String>,
    answers: HashSet<String>,
}

impl<PN> GameStateBuilder<PN>
where
    PN: PlayerName,
{
    pub fn new() -> Self {
        Default::default()
    }

    pub fn num_players(&self) -> usize {
        self.players.len()
    }

    pub fn num_questions(&self) -> usize {
        self.questions.len()
    }

    pub fn num_answer_cards_in_storage(&self) -> usize {
        self.answers.len()
    }

    #[allow(clippy::map_entry)]
    pub fn add_new_player(&mut self, player_name: impl Into<PN>) -> Result<(), GameCoreError> {
        let player_name = player_name.into();

        if self.players.contains(&player_name) {
            Err(GameCoreError::PlayerAlreadyExists {
                name: player_name.to_string(),
            })
        } else {
            self.players.insert(player_name);
            Ok(())
        }
    }

    pub fn withdraw_player(&mut self, player_name: impl Into<PN>) -> Result<(), GameCoreError> {
        let player_name = player_name.into();

        if !self.players.remove(&player_name) {
            Err(GameCoreError::PlayerDoesNotExist {
                name: player_name.to_string(),
            })
        } else {
            Ok(())
        }
    }

    pub fn withdraw_all_players(&mut self) {
        self.players = HashSet::new();
    }

    pub fn remove_all_questions(&mut self) {
        self.questions = HashSet::new();
    }

    pub fn remove_all_answers(&mut self) {
        self.answers = HashSet::new();
    }

    pub fn add_new_question(&mut self, question: impl Into<String>) {
        self.questions.insert(question.into());
    }

    pub fn add_new_questions(&mut self, questions: impl IntoIterator<Item = impl Into<String>>) {
        questions.into_iter().for_each(|q| self.add_new_question(q));
    }

    pub fn add_new_answer(&mut self, answer: impl Into<String>) {
        self.answers.insert(answer.into());
    }

    pub fn add_new_answers(&mut self, answers: impl IntoIterator<Item = impl Into<String>>) {
        answers.into_iter().for_each(|a| self.add_new_answer(a));
    }

    pub fn build(&self, hand_size: usize) -> Result<GameState<PN>, GameCoreError> {
        let num_players = self.players.len();
        if num_players < 3 {
            return Err(GameCoreError::NotEnoughPlayers { num_players });
        }

        if self.num_questions() == 0 {
            return Err(GameCoreError::NoQuestionCards);
        }

        let num_answer_cards_in_storage = self.num_answer_cards_in_storage();
        if num_players * hand_size > num_answer_cards_in_storage {
            return Err(GameCoreError::InsufficientAnswerCardsToDeal {
                num_players,
                each_deal: hand_size,
                num_answer_cards: num_answer_cards_in_storage,
            });
        }

        let mut players = HashMap::from_iter(
            self.players
                .iter()
                .map(|player_name| (player_name.clone(), Player::new())),
        );
        let mut question_card_storage = CardStorage::from_cards(self.questions.clone());
        let mut answer_card_storage = CardStorage::from_cards(self.answers.clone());

        answer_card_storage.shuffle_deck();
        question_card_storage.shuffle_deck();

        for player in players.values_mut() {
            for _ in 0..hand_size {
                player.add_card_to_hand(answer_card_storage.draw_card_from_deck().unwrap());
            }
        }

        Ok(GameState::new(
            players,
            question_card_storage,
            answer_card_storage,
        ))
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn new() {
        let game_state_builder: GameStateBuilder = GameStateBuilder::new();

        assert_eq!(game_state_builder.num_players(), 0);
        assert_eq!(game_state_builder.num_questions(), 0);
        assert_eq!(game_state_builder.num_answer_cards_in_storage(), 0);
    }

    #[test]
    fn add_player() {
        let mut game_state_builder: GameStateBuilder = GameStateBuilder::new();

        assert!(game_state_builder.add_new_player("A").is_ok());
        assert_eq!(game_state_builder.num_players(), 1);
        assert!(game_state_builder.add_new_player("B").is_ok());
        assert_eq!(game_state_builder.num_players(), 2);
    }

    #[test]
    fn withdraw_player() {
        let mut game_state_builder: GameStateBuilder = GameStateBuilder::new();
        game_state_builder.add_new_player("A").ok().unwrap();

        assert!(game_state_builder.withdraw_player("A").is_ok());
        assert_eq!(game_state_builder.num_players(), 0);
    }

    #[test]
    fn withdraw_player_not_exist() {
        let mut game_state_builder: GameStateBuilder = GameStateBuilder::new();
        game_state_builder.add_new_player("A").ok().unwrap();

        assert_eq!(
            game_state_builder.withdraw_player("B").err().unwrap(),
            GameCoreError::PlayerDoesNotExist {
                name: "B".to_owned()
            }
        );
        assert_eq!(game_state_builder.num_players(), 1);
    }

    #[test]
    fn withdraw_all_players() {
        let mut game_state_builder: GameStateBuilder = GameStateBuilder::new();
        game_state_builder.add_new_player("A").ok().unwrap();
        game_state_builder.add_new_player("B").ok().unwrap();
        game_state_builder.add_new_player("C").ok().unwrap();

        game_state_builder.withdraw_all_players();

        assert_eq!(game_state_builder.num_players(), 0);
    }

    #[test]
    fn add_player_already_exists() {
        let mut game_state_builder: GameStateBuilder = GameStateBuilder::new();
        game_state_builder.add_new_player("A").ok().unwrap();

        assert_eq!(
            game_state_builder.add_new_player("A").err().unwrap(),
            GameCoreError::PlayerAlreadyExists {
                name: "A".to_owned()
            }
        );
    }

    #[test]
    fn add_question_to_storage() {
        let mut game_state_builder: GameStateBuilder = GameStateBuilder::new();

        game_state_builder.add_new_question("Where are you?");

        assert_eq!(game_state_builder.num_questions(), 1);
        assert_eq!(game_state_builder.num_answer_cards_in_storage(), 0);
    }

    #[test]
    fn add_questions_to_storage() {
        let mut game_state_builder: GameStateBuilder = GameStateBuilder::new();

        game_state_builder.add_new_questions(["A?", "B?", "C?"]);

        assert_eq!(game_state_builder.num_questions(), 3);
        assert_eq!(game_state_builder.num_answer_cards_in_storage(), 0);
    }

    #[test]
    fn add_answer_to_storage() {
        let mut game_state_builder: GameStateBuilder = GameStateBuilder::new();

        game_state_builder.add_new_answer("Paradise");

        assert_eq!(game_state_builder.num_questions(), 0);
        assert_eq!(game_state_builder.num_answer_cards_in_storage(), 1);
    }

    #[test]
    fn add_answers_to_storage() {
        let mut game_state_builder: GameStateBuilder = GameStateBuilder::new();

        game_state_builder.add_new_answers(["A", "B", "C", "D"]);

        assert_eq!(game_state_builder.num_questions(), 0);
        assert_eq!(game_state_builder.num_answer_cards_in_storage(), 4);
    }

    #[test]
    fn build_game() {
        let mut game_state_builder: GameStateBuilder = GameStateBuilder::new();
        game_state_builder.add_new_player("A").ok().unwrap();
        game_state_builder.add_new_player("B").ok().unwrap();
        game_state_builder.add_new_player("C").ok().unwrap();
        for i in 0..30 {
            game_state_builder.add_new_answer(i.to_string());
        }
        game_state_builder.add_new_question("Are you okay?");

        let game_state = game_state_builder.build(10).ok().unwrap();

        assert_eq!(game_state.players.len(), 3);
        assert_eq!(game_state.players.get("A").unwrap().hand_size(), 10);
        assert_eq!(game_state.players.get("B").unwrap().hand_size(), 10);
        assert_eq!(game_state.players.get("C").unwrap().hand_size(), 10);
        assert_eq!(game_state.answer_card_storage.num_cards_total(), 0);
        assert_eq!(game_state.question_card_storage.num_cards_total(), 1);

        assert_eq!(game_state_builder.num_answer_cards_in_storage(), 30);
        assert_eq!(game_state_builder.num_questions(), 1);
        assert_eq!(
            game_state_builder.num_players(),
            3,
            "Player map in builder should not be reset."
        );
    }

    #[test]
    fn build_game_not_enough_answer_cards() {
        let mut game_state_builder: GameStateBuilder = GameStateBuilder::new();
        game_state_builder.add_new_player("A").ok().unwrap();
        game_state_builder.add_new_player("B").ok().unwrap();
        game_state_builder.add_new_player("C").ok().unwrap();
        for i in 0..29 {
            game_state_builder.add_new_answer(i.to_string());
        }
        game_state_builder.add_new_question("Are you okay?");

        assert_eq!(
            game_state_builder.build(10).err().unwrap(),
            GameCoreError::InsufficientAnswerCardsToDeal {
                num_players: 3,
                each_deal: 10,
                num_answer_cards: 29
            }
        );
        assert_eq!(game_state_builder.num_answer_cards_in_storage(), 29);
    }

    #[test]
    fn build_game_not_enough_players() {
        let mut game_state_builder: GameStateBuilder = GameStateBuilder::new();
        game_state_builder.add_new_player("A").ok().unwrap();
        game_state_builder.add_new_player("B").ok().unwrap();
        for i in 0..30 {
            game_state_builder.add_new_answer(i.to_string());
        }
        game_state_builder.add_new_question("Are you okay?");

        assert_eq!(
            game_state_builder.build(10).err().unwrap(),
            GameCoreError::NotEnoughPlayers { num_players: 2 }
        );
        assert_eq!(game_state_builder.num_answer_cards_in_storage(), 30);
    }

    #[test]
    fn build_game_no_question_card() {
        let mut game_state_builder: GameStateBuilder = GameStateBuilder::new();
        game_state_builder.add_new_player("A").ok().unwrap();
        game_state_builder.add_new_player("B").ok().unwrap();
        game_state_builder.add_new_player("C").ok().unwrap();
        for i in 0..30 {
            game_state_builder.add_new_answer(i.to_string());
        }

        assert_eq!(
            game_state_builder.build(10).err().unwrap(),
            GameCoreError::NoQuestionCards
        );
        assert_eq!(game_state_builder.num_answer_cards_in_storage(), 30);
    }
}
