use std::{collections::HashMap, fmt::Display, hash::Hash};

use rand::{prelude::SliceRandom, thread_rng};

use crate::{
    cards::{AnswerCard, CardStorage, QuestionCard},
    errors::GameCoreError,
    player::Player,
};

pub trait PlayerName: Clone + Display + Eq + Hash + PartialEq {}
impl PlayerName for String {}

pub mod builder;

pub struct GameState<PN = String>
where
    PN: PlayerName,
{
    // Assets
    players: HashMap<PN, Player>,
    question_card_storage: CardStorage<QuestionCard>,
    answer_card_storage: CardStorage<AnswerCard>,

    // Constants
    num_players: usize,
    max_hand_size: usize,
    ordered_players: Vec<PN>,

    // Variables
    current_judge: usize,
    current_question: Option<QuestionCard>,
    submitted_answers_display: SubmittedCombinedAnswersDisplay<PN>,
}

pub type SubmittedCombinedAnswersDisplay<PN> = HashMap<PN, String>;

impl<PN> GameState<PN>
where
    PN: PlayerName,
{
    /// Change the Judge to the next player and return the player's name
    pub fn next_judge(&mut self) -> PN {
        self.current_judge = (self.current_judge + 1) % self.num_players;
        self.current_judge_name()
    }

    pub fn draw_next_question_card(&mut self) -> String {
        if let Some(current_card) = self.current_question.take() {
            self.question_card_storage.discard_card(current_card);
        }

        self.current_question = Some(
            if let Some(next_card) = self.question_card_storage.draw_card_from_deck() {
                next_card
            } else {
                self.question_card_storage.refill_deck_and_shuffle();

                // unwrap() with the assumption that there is at least 1 QuestionCard in the game.
                self.question_card_storage.draw_card_from_deck().unwrap()
            },
        );

        self.current_question.as_ref().unwrap().to_string()
    }

    pub fn submit_answers(
        &mut self,
        player_name: &PN,
        indices: &[usize],
    ) -> Result<Option<SubmittedCombinedAnswersDisplay<PN>>, GameCoreError> {
        let question = if let Some(q) = self.current_question.as_ref() {
            q
        } else {
            return Err(GameCoreError::NoActiveQuestionCard);
        };

        if *player_name == self.current_judge_name() {
            return Err(GameCoreError::JudgeTryingToSubmitAnswers {
                judge_name: self.current_judge_name().to_string(),
            });
        }

        if self.submitted_answers_display.contains_key(player_name) {
            return Err(GameCoreError::PlayerAlreadySubmittedAnswers {
                player_name: player_name.to_string(),
            });
        }

        if let Some(player) = self.players.get_mut(player_name) {
            let played_cards = player.play_cards(indices)?;

            let combined_answer = question.combine_with_answer_cards(&played_cards)?;
            self.submitted_answers_display
                .insert(player_name.clone(), combined_answer);

            played_cards
                .into_iter()
                .for_each(|c| self.answer_card_storage.discard_card(c));

            if self.submitted_answers_display.len() == self.num_players - 1 {
                self.refill_player_hands();

                return Ok(Some(std::mem::take(&mut self.submitted_answers_display)));
            }
        } else {
            return Err(GameCoreError::PlayerDoesNotExist {
                name: player_name.to_string(),
            });
        }

        Ok(None)
    }

    pub fn increment_awesome_points(&mut self, player_name: &PN) -> Result<i32, GameCoreError> {
        if let Some(player) = self.players.get_mut(player_name) {
            Ok(player.increment_awesome_points())
        } else {
            Err(GameCoreError::PlayerDoesNotExist {
                name: player_name.to_string(),
            })
        }
    }

    /// Requirements:
    /// - The player names (keys of `players`) must not change throughout the game.
    /// - `players` are expected to have a filled hand already, each having the same hand size.
    /// - `players` has at least 3 entries.
    /// - `question_card_storage` must not be empty.
    pub(super) fn new(
        players: HashMap<PN, Player>,
        question_card_storage: CardStorage<QuestionCard>,
        answer_card_storage: CardStorage<AnswerCard>,
    ) -> Self {
        let mut ordered_players: Vec<PN> = players.keys().into_iter().cloned().collect();
        ordered_players.shuffle(&mut thread_rng());

        let max_hand_size = players.values().next().unwrap().hand_size();

        Self {
            players,
            question_card_storage,
            answer_card_storage,

            num_players: ordered_players.len(),
            max_hand_size,
            ordered_players,

            current_judge: 0,
            current_question: None,
            submitted_answers_display: HashMap::new(),
        }
    }

    fn current_judge_name(&self) -> PN {
        self.ordered_players[self.current_judge].clone()
    }

    fn refill_player_hands(&mut self) {
        for player in self.players.values_mut() {
            while player.hand_size() < self.max_hand_size {
                player.add_card_to_hand(self.answer_card_storage.draw_card_from_deck().unwrap());
            }
        }
    }
}

mod tests {
    use super::{builder::GameStateBuilder, *};

    #[allow(dead_code)]
    fn get_built_game_state() -> GameState {
        let mut game_state_builder: GameStateBuilder = GameStateBuilder::new();
        game_state_builder.add_new_player("A").ok().unwrap();
        game_state_builder.add_new_player("B").ok().unwrap();
        game_state_builder.add_new_player("C").ok().unwrap();
        for i in 1..=30 {
            game_state_builder.add_new_answer("A".to_owned() + &i.to_string());
        }
        for i in 1..=10 {
            game_state_builder.add_new_question("Q".to_owned() + &i.to_string());
        }

        game_state_builder.build(10).ok().unwrap()
    }

    #[test]
    fn has_a_judge() {
        let game_state = get_built_game_state();

        let current_judge_name = game_state.current_judge_name();

        dbg!(&current_judge_name);
        assert!(
            current_judge_name == "A" || current_judge_name == "B" || current_judge_name == "C"
        );
    }

    #[test]
    fn next_judge() {
        let mut game_state = get_built_game_state();
        let current_judge_name = game_state.current_judge_name();

        game_state.next_judge();

        assert_ne!(game_state.current_judge_name(), current_judge_name);
    }

    #[test]
    fn does_not_have_a_question_at_start() {
        let game_state = get_built_game_state();

        assert!(game_state.current_question.is_none());
    }

    #[test]
    fn draw_next_question_card() {
        let mut game_state = get_built_game_state();

        let current_question = game_state.draw_next_question_card();

        assert!(current_question.starts_with('Q'));
        assert_ne!(game_state.draw_next_question_card(), current_question);
    }

    #[test]
    fn auto_refill_question_cards() {
        let mut game_state = get_built_game_state();

        for _ in 0..100 {
            game_state.draw_next_question_card();
        }
        let current_question = game_state.draw_next_question_card();

        assert!(current_question.starts_with('Q'));
    }

    #[test]
    fn submit_answers_unfinished() {
        let mut game_state = get_built_game_state();
        game_state.draw_next_question_card();
        let current_judge_name = game_state.current_judge_name();
        let answer_submitter = if current_judge_name != "A" { "A" } else { "B" };

        assert_eq!(
            game_state.submit_answers(&answer_submitter.to_owned(), &[7]),
            Ok(None)
        );
    }

    #[test]
    fn submit_answers_finished() {
        let mut game_state = get_built_game_state();
        game_state.draw_next_question_card();
        let current_judge_name = game_state.current_judge_name();
        let answer_submitters = ["A", "B", "C"]
            .iter()
            .filter_map(|&name| {
                if name != current_judge_name {
                    Some(name.to_owned())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        game_state
            .submit_answers(&answer_submitters[0], &[8])
            .ok()
            .unwrap();

        let submit_answers_result = game_state.submit_answers(&answer_submitters[1], &[8]);
        assert_eq!(submit_answers_result.ok().unwrap().unwrap().len(), 2);
    }

    #[test]
    fn submit_answers_before_question_is_drawn() {
        let mut game_state = get_built_game_state();
        let current_judge_name = game_state.current_judge_name();
        let answer_submitter = if current_judge_name != "A" { "A" } else { "B" };

        assert_eq!(
            game_state.submit_answers(&answer_submitter.to_owned(), &[7]),
            Err(GameCoreError::NoActiveQuestionCard)
        );
    }

    #[test]
    fn submit_answers_unknown_player() {
        let mut game_state = get_built_game_state();
        game_state.draw_next_question_card();
        let answer_submitter = "X";

        assert_eq!(
            game_state.submit_answers(&answer_submitter.to_owned(), &[7]),
            Err(GameCoreError::PlayerDoesNotExist {
                name: answer_submitter.to_owned()
            })
        );
    }

    #[test]
    fn submit_answers_but_is_judge() {
        let mut game_state = get_built_game_state();
        game_state.draw_next_question_card();

        assert_eq!(
            game_state.submit_answers(&game_state.current_judge_name(), &[7]),
            Err(GameCoreError::JudgeTryingToSubmitAnswers {
                judge_name: game_state.current_judge_name(),
            })
        );
    }

    #[test]
    fn submit_answers_but_already_submitted_before_finish() {
        let mut game_state = get_built_game_state();
        game_state.draw_next_question_card();
        let current_judge_name = game_state.current_judge_name();
        let answer_submitter = if current_judge_name != "A" { "A" } else { "B" };
        game_state
            .submit_answers(&answer_submitter.to_owned(), &[7])
            .ok()
            .unwrap();

        assert_eq!(
            game_state.submit_answers(&answer_submitter.to_owned(), &[7]),
            Err(GameCoreError::PlayerAlreadySubmittedAnswers {
                player_name: answer_submitter.to_owned()
            })
        );
    }

    #[test]
    fn submit_answers_but_already_submitted_after_finish() {
        let mut game_state = get_built_game_state();
        game_state.draw_next_question_card();
        let current_judge_name = game_state.current_judge_name();
        let answer_submitters = ["A", "B", "C"]
            .iter()
            .filter_map(|&name| {
                if name != current_judge_name {
                    Some(name.to_owned())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        game_state
            .submit_answers(&answer_submitters[0], &[8])
            .ok()
            .unwrap();

        game_state
            .submit_answers(&answer_submitters[1], &[8])
            .ok()
            .unwrap();

        let current_judge_name = game_state.next_judge();
        game_state.draw_next_question_card();
        let answer_submitters = ["A", "B", "C"]
            .iter()
            .filter_map(|&name| {
                if name != current_judge_name {
                    Some(name.to_owned())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        assert_eq!(
            game_state.submit_answers(&answer_submitters[0], &[7]),
            Ok(None)
        );
    }

    #[test]
    fn refill_player_hands() {
        let mut game_state = get_built_game_state();
        game_state.draw_next_question_card();
        let current_judge_name = game_state.current_judge_name();
        let answer_submitter = if current_judge_name != "A" { "A" } else { "B" };
        game_state
            .submit_answers(&answer_submitter.to_owned(), &[7])
            .ok()
            .unwrap();

        game_state.refill_player_hands();
    }

    #[test]
    fn increment_awesome_points() {
        let mut game_state = get_built_game_state();

        assert_eq!(
            game_state
                .increment_awesome_points(&"A".to_owned())
                .ok()
                .unwrap(),
            1
        );
        assert_eq!(
            game_state
                .increment_awesome_points(&"B".to_owned())
                .ok()
                .unwrap(),
            1
        );
        assert_eq!(
            game_state
                .increment_awesome_points(&"A".to_owned())
                .ok()
                .unwrap(),
            2
        );
    }

    #[test]
    fn increment_awesome_points_unknown_player() {
        let mut game_state = get_built_game_state();

        assert_eq!(
            game_state.increment_awesome_points(&"D".to_owned()),
            Err(GameCoreError::PlayerDoesNotExist {
                name: "D".to_owned()
            })
        )
    }
}
