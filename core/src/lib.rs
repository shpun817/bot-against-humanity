mod cards;
pub mod drivers;
mod errors;
pub mod game_state;
mod player;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet() -> String {
    "Hello, core".into()
}
