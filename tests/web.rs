//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use password_strength_rs::Calculator;
use wasm_bindgen::JsValue;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn entropy_test() {
    let mock_input_value = JsValue::from_str("password123");

    let pw_strength = Calculator::new(mock_input_value);

    assert_eq!(JsValue::from_f64(28f64), pw_strength.get_entropy());
}

#[wasm_bindgen_test]
fn strength_test() {
    let mock_input_value = JsValue::from_str("password123");

    let pw_strength = Calculator::new(mock_input_value);

    assert_eq!(JsValue::from_str("very-weak"), pw_strength.get_strength());
}
