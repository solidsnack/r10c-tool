use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct R10c;

#[wasm_bindgen]
impl R10c {
    pub fn prev(n: f64) -> f64 {
        crate::prev(n)
    }

    pub fn near(n: f64) -> f64 {
        crate::near(n)
    }

    pub fn next(n: f64) -> f64 {
        crate::next(n)
    }
}
