use wasm_bindgen::prelude::*;

use crate::{drivers::GameCoreDriver, error::GameCoreError};

use super::generic::GenericDriver;

/// Expose the API of `GenericDriver` to WASM
#[wasm_bindgen]
#[derive(Default)]
pub struct WasmDriver {
    generic_driver: GenericDriver,
}

pub type Error = GameCoreError;

impl From<Error> for JsValue {
    fn from(val: Error) -> Self {
        val.to_string().into()
    }
}

#[wasm_bindgen]
impl WasmDriver {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Default::default()
    }

    pub fn set_hand_size(&mut self, hand_size: usize) {
        self.generic_driver.set_hand_size(hand_size);
    }

    pub fn add_player(&mut self, player_name: &str) -> Result<(), Error> {
        self.generic_driver.add_player(player_name)
    }

    pub fn remove_player(&mut self, player_name: &str) -> Result<(), Error> {
        self.generic_driver.remove_player(player_name)
    }

    pub fn remove_all_players(&mut self) {
        self.generic_driver.remove_all_players()
    }

    pub fn add_new_questions(&mut self, questions: JsValue) -> Result<(), Error> {
        let questions: Vec<String> = questions
            .into_serde()
            .map_err(|_| "Supplied questions are not an array of strings.")?;

        self.generic_driver.add_new_questions(questions);

        Ok(())
    }

    pub fn clear_all_questions(&mut self) {
        self.generic_driver.clear_all_questions();
    }

    pub fn add_new_answers(&mut self, answers: JsValue) -> Result<(), Error> {
        let answers: Vec<String> = answers
            .into_serde()
            .map_err(|_| "Supplied answers are not an array of strings.")?;

        self.generic_driver.add_new_answers(answers);

        Ok(())
    }

    pub fn clear_all_answers(&mut self) {
        self.generic_driver.clear_all_answers();
    }
}

// WASM API for GameCoreDriver trait functions in GenericDriver
#[wasm_bindgen]
impl WasmDriver {
    /// From JavaScript:
    /// - Success: an array of strings (player names)
    /// - Failure: a string (error message)
    pub fn start_game(&mut self) -> Result<JsValue, GameCoreError> {
        self.generic_driver
            .start_game()
            .map(|player_names| JsValue::from_serde(&player_names).unwrap())
    }

    /// From JavaScript:
    /// - Success: an object { judge: string, question: string, player_hands: { `name`: \[`answer`: string\] } }
    /// - Failure: a string (error message)
    pub fn start_round(&mut self) -> Result<JsValue, GameCoreError> {
        self.generic_driver
            .start_round()
            .map(|round_info| JsValue::from_serde(&round_info).unwrap())
    }

    /// From JavaScript:
    /// - Input: `player_name`: string, `answer_indices`: \[number >= 0\]
    /// - Success: null | an array of tuples of two strings (player names and their combined answers)
    /// - Failure: a string (error message)
    pub fn submit_answers(
        &mut self,
        player_name: &str,
        answer_indices: &[usize],
    ) -> Result<JsValue, GameCoreError> {
        self.generic_driver
            .submit_answers(player_name, answer_indices.to_owned())
            .map(|submitted_answers_option| JsValue::from_serde(&submitted_answers_option).unwrap())
    }

    /// From JavaScript:
    /// - Input: `chosen_player`: string
    /// - Success: an array of tuples of a string and a number (player names and their awesome points, sorted in descending order of awesome points)
    /// - Failure: a string (error message)
    pub fn end_round(&mut self, chosen_player: &str) -> Result<JsValue, GameCoreError> {
        self.generic_driver
            .end_round(chosen_player)
            .map(|ranking| JsValue::from_serde(&ranking).unwrap())
    }

    /// From JavaScript:
    /// - Success: void
    /// - Failure: a string (error message)
    pub fn end_game(&mut self) -> Result<(), GameCoreError> {
        self.generic_driver.end_game()
    }
}
