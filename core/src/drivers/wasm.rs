use std::convert::TryInto;

use wasm_bindgen::prelude::*;

use crate::{drivers::GameCoreDriver, error::GameCoreError};

use super::generic::{GenericDriver, GenericDriverBuilder};

/// Expose the API of `GenericDriverBuilder` to WASM
#[wasm_bindgen]
#[derive(Default)]
pub struct WasmDriverBuilder {
    generic_driver_builder: GenericDriverBuilder,
}

/// Expose the API of `GenericDriver` to WASM
#[wasm_bindgen]
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
impl WasmDriverBuilder {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Default::default()
    }

    #[wasm_bindgen(js_name = setHandSize)]
    pub fn set_hand_size(&mut self, hand_size: i32) -> Result<(), Error> {
        let hand_size = hand_size
            .try_into()
            .map_err(|_| "Hand size cannot be negative.")?;

        self.generic_driver_builder.set_hand_size(hand_size)?;

        Ok(())
    }

    #[wasm_bindgen(js_name = addPlayer)]
    pub fn add_player(&mut self, player_name: &str) -> Result<(), Error> {
        self.generic_driver_builder.add_player(player_name)
    }

    #[wasm_bindgen(js_name = removePlayer)]
    pub fn remove_player(&mut self, player_name: &str) -> Result<(), Error> {
        self.generic_driver_builder.remove_player(player_name)
    }

    #[wasm_bindgen(js_name = removeAllPlayers)]
    pub fn remove_all_players(&mut self) {
        self.generic_driver_builder.remove_all_players()
    }

    #[wasm_bindgen(js_name = addNewQuestions)]
    pub fn add_new_questions(&mut self, questions: JsValue) -> Result<(), Error> {
        let questions: Vec<String> = questions
            .into_serde()
            .map_err(|_| "Supplied questions are not an array of strings.")?;

        self.generic_driver_builder.add_new_questions(questions);

        Ok(())
    }

    #[wasm_bindgen(js_name = clearAllQuestions)]
    pub fn clear_all_questions(&mut self) {
        self.generic_driver_builder.clear_all_questions();
    }

    #[wasm_bindgen(js_name = addNewAnswers)]
    pub fn add_new_answers(&mut self, answers: JsValue) -> Result<(), Error> {
        let answers: Vec<String> = answers
            .into_serde()
            .map_err(|_| "Supplied answers are not an array of strings.")?;

        self.generic_driver_builder.add_new_answers(answers);

        Ok(())
    }

    #[wasm_bindgen(js_name = clearAllAnswers)]
    pub fn clear_all_answers(&mut self) {
        self.generic_driver_builder.clear_all_answers();
    }

    /// - Success: a WasmDriver (game driver)
    /// - Failure: a string (error message)
    #[wasm_bindgen(js_name = build)]
    pub fn build(&self) -> Result<WasmDriver, Error> {
        self.generic_driver_builder
            .build()
            .map(|generic_driver| WasmDriver { generic_driver })
    }
}

// WASM API for GameCoreDriver trait functions in GenericDriver
#[wasm_bindgen]
impl WasmDriver {
    /// Return an array of strings (ordered all player names)
    #[wasm_bindgen(js_name = orderedPlayers)]
    pub fn ordered_players(&self) -> JsValue {
        let ordered_players = self.generic_driver.ordered_players();

        JsValue::from_serde(&ordered_players).unwrap()
    }

    /// Return an object { judge: string, question: string, player_hands: { `name`: \[`answer`: string\] } }
    #[wasm_bindgen(js_name = startRound)]
    pub fn start_round(&mut self) -> JsValue {
        let round_info = self.generic_driver.start_round();

        JsValue::from_serde(&round_info).unwrap()
    }

    /// From JavaScript:
    /// - Input: `player_name`: string, `answer_indices`: \[number >= 0\]
    /// - Success: null | an array of tuples of two strings (player names and their combined answers)
    /// - Failure: a string (error message)
    ///
    /// `answer_indices` correspond to ZERO-based indices of the player's hand.
    #[wasm_bindgen(js_name = submitAnswers)]
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
    /// - Input: `player_names`: an array of strings
    #[wasm_bindgen(js_name = redrawHands)]
    pub fn redraw_hands(&mut self, player_names: JsValue) -> Result<(), GameCoreError> {
        let player_names: Vec<String> = player_names
            .into_serde()
            .map_err(|_| "Supplied player names are not an array of strings.")?;

        self.generic_driver.redraw_hands(player_names)
    }

    /// From JavaScript:
    /// - Input: `chosen_player`: string
    /// - Success: an array of tuples of a string and a number (player names and their awesome points, sorted in descending order of awesome points)
    /// - Failure: a string (error message)
    #[wasm_bindgen(js_name = endRound)]
    pub fn end_round(&mut self, chosen_player: &str) -> Result<JsValue, GameCoreError> {
        self.generic_driver
            .end_round(chosen_player)
            .map(|ranking| JsValue::from_serde(&ranking).unwrap())
    }

    /// Invalidate this driver object.
    ///
    /// No side effects.
    #[wasm_bindgen(js_name = endGame)]
    pub fn end_game(self) {
        self.generic_driver.end_game()
    }
}
