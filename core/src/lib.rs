pub mod drivers;
pub mod game_state;

mod cards;
mod error;
mod player;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet() -> String {
    "Hello, core".into()
}
