/*
    Appellation: utils <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:... Summary...
*/
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add_one(data: usize) -> usize {
    data + 1
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::wasm_bindgen_test;

    #[wasm_bindgen_test]
    fn test_add_one() {
        assert_eq!(add_one(10), 11)
    }
}