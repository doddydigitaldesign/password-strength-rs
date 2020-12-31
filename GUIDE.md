# Creating a WebAssembly library with Rust in 2020
In this guide we will learn how to create a WebAssembly library using Rust 
and publish it to [NPM](https://www.npmjs.com) as a module that can be used
with our JS projects.

See the [live demo](https://doddydigitaldesign.github.io/password-strength-rs/)
for a preview of what we're building.

## Requirements
This guide assumes that we have the following installed:
- a text editor such as [Visual Studio Code](https://code.visualstudio.com/) for i.e. syntax high-lighting
- the [Rust](https://www.rust-lang.org/) programming language with the included Cargo package manager for managing Rust crates
- [NodeJS](https://nodejs.org/en/) with NPM for the JS parts
- [cargo-generate](https://crates.io/crates/cargo-generate) for setting up the project using a template
- [wasm-pack](https://github.com/rustwasm/wasm-pack/releases) for handling the Rust-to-WebAssembly steps

## Project setup
In order to focus on the interesting parts we'll be using `cargo-generate` and a [github repo](https://github.com/rustwasm/wasm-pack-template.git) to bootstrap our project.
Open a terminal and type the following command:
```sh
cargo generate --git https://github.com/rustwasm/wasm-pack-template.git
```
We'll then be asked to provide a name for our project.
```sh
 Project Name: password-strength-rs
 Creating project called `password-strength-rs`...
 Done! New project created E:/projects/password-strength-rs
```
Great, let's open our editor in the new directory. I'll be using VSCode:
```sh
cd password-strength-rs
code E:/projects/password-strength-rs
```

Let's check that it compiles correctly:
```sh
wasm-pack build
```
And wait for it to finish:
```
[INFO]: Checking for the Wasm target...
[INFO]: Compiling to Wasm...
   Compiling proc-macro2 v1.0.24
   Compiling unicode-xid v0.2.1
   Compiling log v0.4.11
   Compiling syn v1.0.56
   Compiling wasm-bindgen-shared v0.2.69
   Compiling cfg-if v0.1.10
   Compiling lazy_static v1.4.0
   Compiling bumpalo v3.4.0
   Compiling wasm-bindgen v0.2.69
   Compiling cfg-if v1.0.0
   Compiling quote v1.0.8
   Compiling wasm-bindgen-backend v0.2.69
   Compiling wasm-bindgen-macro-support v0.2.69
   Compiling wasm-bindgen-macro v0.2.69
   Compiling console_error_panic_hook v0.1.6
   Compiling password-strength-rs v0.1.0 (E:/projects/password-strength-rs)
warning: function is never used: `set_panic_hook`
 --> src\utils.rs:1:8
  |
1 | pub fn set_panic_hook() {
  |        ^^^^^^^^^^^^^^
  |
  = note: `#[warn(dead_code)]` on by default

warning: 1 warning emitted

    Finished release [optimized] target(s) in 11.66s
[INFO]: Installing wasm-bindgen...
[INFO]: Optimizing wasm binaries with `wasm-opt`...
[INFO]: Optional fields missing from Cargo.toml: 'description', 'repository', and 'license'. These are not necessary, but recommended
[INFO]: :-) Done in 13.38s
[INFO]: :-) Your wasm pkg is ready to publish at E:/projects/password-strength-rs.
```
The compiled module can be found in the `pkg` directory of our project.
Awesome! Now we can move on to more interesting stuff. 🎉

## Exposing Rust to JS
Our goal is to have our NPM module expose functionality written in Rust
to the users of our library. They'll simply add our module to their `package.json`
and import it in their code, for example in a `React` component:
```tsx
import pws from 'password-strength-rs';
import React, { useState } from 'react';

export const PasswordInput = () => {
  const [value, setValue] = useState<string>("");
  const [strength, setStrength] = useState<number|null>(null);

  const handleChange = (event: React.ChangeEvent<HTMLInputElement>) => {
      setValue(event.target.value);
      setStrength(pws.validate(event.target.value));
  };

  const Indicator = () => <span role="image">
    {strength >= pws.defaultThreshold ? "💪" : "👎"}
  </span>;

  return (
    <div className="password-strength">
      <input
        onChange={handleChange}
        value={value}
      />
      <p>Password strength: <Indicator /></p>
    </div>
  );
}
```

Let's open `src/lib.rs`, which should look something like this:
```rust
mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, password-strength-rs!");
}

```
If the above code is unfamiliar to you, I'd recommend checking out [the official Rust guide](https://doc.rust-lang.org/book/) and learn the basics of the Rust programming language.

Currently our library exposes a function, `greet`, which calls the browsers' `alert` function with
`"Hello, password-strength-rs!"`. The conversion from Rust `str` to a JS `string` is handled
by the `wasm_bindgen` macro. 

Let's create a new function that takes a JS `string` and returns a JS `number` (64-bit float):
```rust
mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn validate(input: JsValue) -> JsValue {
    let password_string = input.as_string().unwrap();
    let password_length = password_string.len() as f64;

    JsValue::from_f64(password_length)
}
```

To test our new function, we'll open `tests/web.rs` and write a simple test:
```rust
//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use password_strength_rs::validate;
use wasm_bindgen::JsValue;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    let mock_input_value = JsValue::from_str("password123");

    assert_eq!(JsValue::from_f64(11.0), validate(mock_input_value));
}

```

Let's test it in a headless Chrome browser
```sh
wasm-pack test --headless --chrome
```
and wait for it to finish:
```
...
test web::pass ... ok

test result: ok. 1 passed; 0 failed; 0 ignored
```

Great, we've learned how to test our code and make it available to JS the browser! 💪

## Estimating the strength of a password
In order to estimate the strength of a password we'll need to define what we mean by "strength"
in this context. Currently our estimated strength value is the number of characters of the password.

According to [NIST](https://pages.nist.gov/800-63-3/sp800-63b.html#appA), the length is a 
primary factor of characterizing password strength. Simply put, it takes more time to guess
a password the more characters it contains and how many possible characters each character can be. [Kaspersky](https://password.kaspersky.com/) illustrates this nicely in their `Password Check` app by estimating the time it would take a home computer to bruteforce a password. For example, the password `"abcd1234"` would take approximately 3 seconds to bruteforce while `"abcdefghiklmnopqrstuvwxyz1234567890"` would take approximately 13 days.

So, we have the password length. Let's try to improve our estimate by including another factor: the size of the set of used characters:
```
Sizes:
- 0-9: 10
- a-z: 26
- a-z + A-Z: 52
- a-z + A-Z + 0-9: 62
- ASCII: 94

Source: https://en.wikipedia.org/wiki/Password_strength
```

Let's open up `src/lib.rs` again and calculate entropy of the password:
```rust
mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn validate(input: JsValue) -> JsValue {
    let password_string = input.as_string().unwrap();
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
    fn get_entropy(length: f64, number_of_symbols: f64) -> f64 {
        ((length * number_of_symbols.log2())/2f64).round()
    }
    let entropy = get_entropy(password_length, possible_symbols_count);

    JsValue::from_f64(entropy)
}

```
Note: This is entropy estimation assumes that the password is randomly generated so the actual
entropy of user selected passwords is likely much lower, hence I simply divide it by 2. 
I'll leave to the reader to find a more accurate estimate.

## Strength estimates
We should add a function that converts the calculated password entropy into something more user-friendly.

Let's collect the functionality of our library in a struct and expose that as a JS class:
```rust
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

```

We should update our tests too, let's edit `tests/web.rs`:
```rust
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

```