use bottomify::bottom::{decode_string, encode_string};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn encode(input: &str) -> String {
    encode_string(&input)
}

#[wasm_bindgen]
pub fn decode(input: String) -> Option<String> {
    decode_string(&input).ok()
}
