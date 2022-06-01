use wasm_bindgen::prelude::*;

use super::generic::GenericDriver;

/// Expose the API of `GenericDriver` to WASM
#[wasm_bindgen]
pub struct WasmDriver {
    generic_driver: GenericDriver,
}

pub type Error = String;

#[allow(clippy::new_without_default)]
#[wasm_bindgen]
impl WasmDriver {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            generic_driver: GenericDriver::new(),
        }
    }
}
