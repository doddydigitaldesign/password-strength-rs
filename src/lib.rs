mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Calculator {
    password: JsValue,
}
#[wasm_bindgen]
impl Calculator {
    pub fn new(pw: JsValue) -> Self {
        Self { password: pw }
    }

    /// Get an entropy estimate of the password.
    pub fn get_entropy(&self) -> JsValue {
        let password_string = self.password.as_string().unwrap();
        let password_length = password_string.len() as f64;
        // 26 possible chars
        let contains_lowercase = password_string.chars().any(|c| c.is_ascii_lowercase());
        // 26 possible chars
        let contains_uppercase = password_string.chars().any(|c| c.is_ascii_uppercase());
        // 10 possible
        let contains_digit = password_string.chars().any(|c| c.is_ascii_digit());
        // 31 possible
        let contains_punctuation = password_string.chars().any(|c| c.is_ascii_punctuation());

        let mut possible_symbols_count = 0f64;
        if contains_lowercase {
            possible_symbols_count += 26f64;
        }
        if contains_uppercase {
            possible_symbols_count += 26f64;
        }
        if contains_digit {
            possible_symbols_count += 10f64;
        }
        if contains_punctuation {
            possible_symbols_count += 31f64;
        }

        // This assumes the password is randomly generated, which is rarely the case.
        fn calculate_entropy(length: f64, number_of_symbols: f64) -> f64 {
            ((length * number_of_symbols.log2()) / 2f64).round()
        }
        let entropy = calculate_entropy(password_length, possible_symbols_count);

        JsValue::from_f64(entropy)
    }

    /// Get a strength estimate of the password.
    pub fn get_strength(&self) -> JsValue {
        let entropy = self.get_entropy();
        let entropy_value = entropy.as_f64().unwrap();
        if entropy_value <= 28f64 {
            return JsValue::from_str("very-weak");
        }
        if entropy_value <= 59f64 {
            return JsValue::from_str("weak");
        }
        if entropy_value <= 127f64 {
            return JsValue::from_str("strong");
        }
        // 128 bits of entropy is considered very strong
        return JsValue::from_str("very-strong");
    }
}
