# cifers

Small Rust crate implementing classic ciphers (Caesar, Vigenère, Beaufort,
Affine, Railfence, Redefence) and custom-alphabet variants.

## Features

- `Cipher` trait with `encipher` / `decipher` methods
- Canonical implementations: Caesar, Vigenère, Beaufort, Affine, Railfence
- Custom-alphabet variants for Caesar, Vigenère, Beaufort, Affine
- Small helpers for alphabet validation

## Quick examples

Basic Caesar usage:

```rust
use cifers::Caeser;

let c = Caeser::new().set_shift(3);
let encrypted = c.encipher("exampletext");
let decrypted = c.decipher(&encrypted);
assert_eq!(decrypted, "exampletext");
```

Custom-alphabet Caesar usage:

```rust
use cifers::custom_alphabet_ciphers::custom_caesar::CustomCaesar;

let c = CustomCaesar::new(String::from("abcdefghijklmnopqrstuvwxyz")).set_shift(3);
assert_eq!(c.encipher("abc"), "def");
```
Adding A Custom Alphabet To Regular Cipher:
```rust
use cifers::custom_alphabet_ciphers::custom_caesar::CustomCaesar;

let c = Caesar::new()set_alphabet(String::from("abcdefghijklmnopqrstuvwxyz")).set_shift(3);
assert_eq!(c.encipher("abc"), "def");
```

Vigenère example:

```rust
use cifers::Vigenere;
let c = Vigenere::new().set_code(String::from("acrylic"));
assert_eq!(c.encipher("exampletext"), "ezrkatgtgor");
```

See the module docs for more examples and API details.

## Build & test

Build the crate:

```bash
cargo build
```

Run unit tests:

```bash
cargo test
```

Generate documentation:

```bash
cargo doc --no-deps
# open the docs locally (macOS)
open target/doc/cifers/index.html
```

## Contributing

Contributions welcome. Please add tests for behavior you change and keep
public APIs stable.

## License

MIT / Apache-2.0
