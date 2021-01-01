# password-strength-rs
*A password strength calculator written in Rust and compiled to WASM.*

## Installation
```sh
npm install password-strength-rs
```

## Usage
The `get_strength` method converts the estimated password entropy into a human-readable format:
- `"very-weak"`: 0-28 bits
- `"weak"`: 28-59 bits
- `"strong"`: 59-127 bits
- `"very-strong"`: 128 bits or more
```ts
import { Calculator } from 'password-strength-rs';

const calc = Calculator.new("password123");

const entropy = calc.get_entropy();

// Get a human-readable strength estimate
const strength = calc.get_strength();
```

## Related guide
See [GUIDE.md](https://github.com/doddydigitaldesign/password-strength-rs/blob/main/GUIDE.md) or visit [www.doddy.se/guides](https://www.doddy.se/guides).
