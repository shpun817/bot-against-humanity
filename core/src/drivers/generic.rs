use std::collections::HashMap;

use serde::Serialize;

use crate::{
    error::GameCoreError,
    game_state::{GameState, GameStateBuilder},
};

use super::GameCoreDriver;

pub struct GenericDriverBuilder {
    game_state_builder: GameStateBuilder,
    hand_size_config: usize,
}

pub struct GenericDriver {
    game_state: GameState,
}

#[derive(Serialize)]
pub struct RoundInformation {
    pub judge: String,
    pub question: String,
    pub player_hands: HashMap<String, Vec<String>>,
}

impl Default for GenericDriverBuilder {
    fn default() -> Self {
        Self {
            game_state_builder: Default::default(),
            hand_size_config: 10,
        }
    }
}

impl GenericDriverBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn set_hand_size(&mut self, hand_size: usize) -> Result<(), GameCoreError> {
        if hand_size == 0 {
            Err(GameCoreError::HandSizeCannotBeZero)
        } else {
            self.hand_size_config = hand_size;
            Ok(())
        }
    }

    pub fn add_player(&mut self, player_name: impl Into<String>) -> Result<(), GameCoreError> {
        self.game_state_builder.add_new_player(player_name)
    }

    pub fn remove_player(&mut self, player_name: impl Into<String>) -> Result<(), GameCoreError> {
        self.game_state_builder.withdraw_player(player_name)
    }

    pub fn remove_all_players(&mut self) {
        self.game_state_builder.withdraw_all_players()
    }

    pub fn add_new_questions(&mut self, questions: impl IntoIterator<Item = impl Into<String>>) {
        self.game_state_builder.add_new_questions(questions);
    }

    pub fn clear_all_questions(&mut self) {
        self.game_state_builder.remove_all_questions();
    }

    pub fn add_new_answers(&mut self, answers: impl IntoIterator<Item = impl Into<String>>) {
        self.game_state_builder.add_new_answers(answers);
    }

    pub fn clear_all_answers(&mut self) {
        self.game_state_builder.remove_all_answers();
    }

    pub fn build(&self) -> Result<GenericDriver, GameCoreError> {
        let game_state = self.game_state_builder.build(self.hand_size_config)?;

        Ok(GenericDriver { game_state })
    }
}

impl GameCoreDriver for GenericDriver {
    type PlayerName = String;
    type Error = GameCoreError;
    type RoundStartInfo = RoundInformation;
    type RoundEndInfo = Vec<(Self::PlayerName, i32)>;

    fn ordered_players(&self) -> Vec<Self::PlayerName> {
        self.game_state.ordered_players()
    }

    fn start_round(&mut self) -> Self::RoundStartInfo {
        let game_state = &mut self.game_state;

        RoundInformation {
            judge: game_state.next_judge(),
            question: game_state.draw_next_question_card(),
            player_hands: game_state.report_hands(),
        }
    }

    /// `answer_indices` correspond to ZERO-based indices of the player's hand.
    fn submit_answers(
        &mut self,
        player_name: impl Into<Self::PlayerName>,
        answer_indices: impl IntoIterator<Item = impl Into<usize>>,
    ) -> Result<Option<Vec<(Self::PlayerName, String)>>, Self::Error> {
        let answer_indices: Vec<usize> = answer_indices.into_iter().map(Into::into).collect();

        self.game_state
            .submit_answers(&player_name.into(), &answer_indices)
    }

    /// Return Err() only when the game is not started or `chosen_player` is unknown.
    fn end_round(
        &mut self,
        chosen_player: impl Into<Self::PlayerName>,
    ) -> Result<Self::RoundEndInfo, Self::Error> {
        self.game_state
            .increment_awesome_points(&chosen_player.into())?;

        Ok(self.game_state.report_awesome_point_ranking())
    }

    /// No side effects.
    fn end_game(self) {}
}

#[allow(dead_code)]
#[cfg(test)]
mod integration_tests {
    use regex::Regex;

    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn new() {
        GenericDriverBuilder::new();
    }

    #[test]
    fn test_set_up_a_game() {
        let mut driver = GenericDriverBuilder::new();

        driver.set_hand_size(6);
        driver.set_hand_size(10);

        for player in players() {
            assert!(
                driver.add_player(player).is_ok(),
                "Should be able to add a non-existent player."
            );
        }
        assert!(
            driver.add_player("Player A").is_err(),
            "Should not be able to add an existent player."
        );
        assert!(
            driver.remove_player("Player C").is_ok(),
            "Should be able to remove an existent player."
        );
        assert!(
            driver.remove_player("Player E").is_err(),
            "Should not be able to remove a non-existent player."
        );
        driver.remove_all_players();
        for player in players() {
            assert!(
                driver.add_player(player).is_ok(),
                "Should be able to add a non-existent player."
            );
        }

        driver.add_new_questions(questions());
        driver.clear_all_questions();
        driver.add_new_questions(questions());

        driver.add_new_answers(answers());
        driver.clear_all_answers();
        driver.add_new_answers(answers());
    }

    fn set_up_a_game(driver: &mut GenericDriverBuilder, add_players: bool) -> GenericDriver {
        driver.set_hand_size(10);

        if add_players {
            for player in players() {
                driver.add_player(player).unwrap();
            }
        }

        driver.add_new_questions(questions());

        driver.add_new_answers(answers());

        driver.build().unwrap()
    }

    #[test]
    fn test_run_a_game() {
        run_a_game(set_up_a_game(&mut GenericDriverBuilder::new(), true));
    }

    fn run_a_game(mut driver: GenericDriver) {
        let winning_awesome_points = 3;
        let mut black_box_votee = 1;
        loop {
            let round_information = driver.start_round();
            let non_judge_players = find_non_judge_players(&round_information.judge);

            // Display round information to users
            println!("The Judge for this round is {}!", round_information.judge);
            println!("Answer this: {}", round_information.question);
            for player in non_judge_players.iter() {
                println!("======================================================");
                println!("{}, Your hand is:", player);
                for (i, card_content) in round_information
                    .player_hands
                    .get(player)
                    .unwrap()
                    .iter()
                    .enumerate()
                {
                    println!("{} - {}", i + 1, card_content);
                }
                println!("======================================================");
            }

            // Simulating users' input of choosing answers
            let correct_num_blanks = Regex::new("_+")
                .unwrap()
                .find_iter(&round_information.question)
                .count();
            let mut submitted_answers = None;
            for player in non_judge_players {
                submitted_answers = driver
                    .submit_answers(player, 0..correct_num_blanks)
                    .unwrap();
            }
            let submitted_answers = submitted_answers.unwrap();

            // Display the submitted answers to everyone
            println!("The following creative answers were collected:");
            for (i, (_, answer)) in submitted_answers.iter().enumerate() {
                println!("{} - {}", i + 1, answer);
            }
            println!("Choose your favorite, Judge {}!", round_information.judge);

            // Simulate the Judge casting a vote
            let chosen_one = submitted_answers[black_box_votee - 1].0.clone();
            black_box_votee = (black_box_votee + 1) % submitted_answers.len() + 1;
            let ranking = driver.end_round(chosen_one).unwrap();
            let highest = ranking[0].clone();

            // Display the ranking
            let mut rank_tracker = 0;
            let mut last_points = highest.1 + 1;
            println!("======================================================");
            for (name, points) in ranking {
                if points != last_points {
                    rank_tracker += 1
                }
                println!("{} {} - {}", rank_tracker, name, points);
                last_points = points;
            }
            println!("======================================================");

            if highest.1 >= winning_awesome_points {
                println!("Congratulations, {}, You Have Won!", highest.0);
                break;
            }
        }

        driver.end_game();
    }

    #[test]
    fn test_run_multiple_times() {
        let mut builder = GenericDriverBuilder::new();
        run_a_game(set_up_a_game(&mut builder, true));

        run_a_game(set_up_a_game(&mut builder, false));

        builder.remove_all_players();
        run_a_game(set_up_a_game(&mut builder, true));
        run_a_game(set_up_a_game(&mut builder, false));
    }

    fn find_non_judge_players(judge_name: &String) -> Vec<String> {
        players().into_iter().filter(|p| p != judge_name).collect()
    }

    fn players() -> Vec<String> {
        vec!["Player A", "Player B", "Player C", "Player D"]
            .into_iter()
            .map(|p| p.to_owned())
            .collect()
    }

    fn questions() -> Vec<String> {
        vec![
            "Who is the smartest person alive?",
            "I only believe in __.",
            "_ and _ are the best things in the world.",
            "I take pride in my _, __, and ___.",
            "_,_,_.",
        ]
        .into_iter()
        .map(|p| p.to_owned())
        .collect()
    }

    fn answers() -> Vec<String> {
        vec![
            "Zombie Apocalypse",
            "Winne the Pooh",
            "video",
            "blood",
            "cute girl",
            "shopping",
            "loud speaker",
            "response",
            "smart method",
            "instance of the class",
            "population",
            "pollution",
            "smelly garbage",
            "argument",
            "tennis ball",
            "hearing",
            "estate",
            "refrigerator",
            "transportation",
            "food",
            "piano",
            "president",
            "fact",
            "poet",
            "permission",
            "ear",
            "scary ugly insect",
            "indication",
            "bad medicine",
            "history",
            "election",
            "requirement",
            "establishment",
            "agreement",
            "grocery",
            "big camera",
            "reputation",
            "solution",
            "role",
            "scary long statement",
            "impression",
            "interaction",
            "winner",
            "photo",
            "republic",
            "yummy pizza",
            "grey computer",
            "profession",
            "paid membership",
            "university",
            "high income",
            "big data",
        ]
        .into_iter()
        .map(|p| p.to_owned())
        .collect()
    }
}
