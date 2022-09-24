use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[test]
pub fn add_test() {
    assert_eq!(1 + 1, add(1, 1));
}

#[wasm-bindgen]
pub mod cat {
    pub fn meow() {
        crate::dog::woof();
    }
}

#[wasm-bindgen]
pub mod dog {
    pub fn woof() {
        println!("Woof Woof!");
    }
}

#[wasm-bindgen]
pub fn test_one() {
    cat::meow();
}
