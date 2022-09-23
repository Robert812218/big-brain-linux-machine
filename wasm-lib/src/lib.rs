//pub fn add(left: usize, right: usize) -> usize {
//    left + right
//}
//
//[cfg(test)]
//mod tests {
//    use super::*;
//
//    #[test]
//    fn it_works() {
//        let result = add(2, 2);
//        assert_eq!(result, 4);
//    }
//}


use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[test]
pub fn add_test() {
    assert_eq!(1 + 1, add(1, 1));
}
