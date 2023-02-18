// The wasm-pack uses wasm-bindgen to build and generate JavaScript binding file.
// Import the wasm-bindgen crate.
use wasm_bindgen::prelude::*;

// This exports an add function
// It takes two 32-bits integer values
// And returns a 32-bits integer value

#[wasm_bindgen]
pub fn call_me_from_javascript(a: i32, b: i32) -> i32 {
    add_integer_with_constant(a, b)
}

// An NOT exported constant
// Rust does not support exporting constants
// for Wasm (that I known of)
const ADD_CONSTANT: i32 = 23;

// An Not exported function

fn add_integer_with_constant(a: i32, b: i32) -> i32 {
    a + b + ADD_CONSTANT
}
