# Zero Width Unicode Standard (ZWUS)

Zero Width Unicode Steganography — hide text inside invisible characters.

```toml
[dependencies]
zwus = "0.1"
```

## Usage

```rust
use zwus::Zwus;

// Encode & decode strings
let hidden = Zwus::encode_string("secret");
let revealed = Zwus::decode_to_string(&hidden);
assert_eq!(revealed, "secret");

// Encode & decode number arrays
let encoded = Zwus::encode_number_array(&[72, 101, 108]);
let decoded = Zwus::decode_to_number_array(&encoded);
assert_eq!(decoded, vec![72, 101, 108]);
```

### Base

Higher base = shorter output, but more likely visible in some renderers.

```rust
Zwus::encode_string_with_base("hi", 3); // default, safest
Zwus::encode_string_with_base("hi", 6); // compact
Zwus::encode_string_with_base("hi", 8); // most compact
```

Decode must match the encode base:

```rust
Zwus::decode_to_string_with_base(&encoded, 6);
```

### Embedded in visible text

Non-ZWUS characters are automatically ignored during decoding, so hidden payloads survive being mixed into normal text.

```rust
let hidden = Zwus::encode_string("secret");
let carrier = format!("nothing to see here{hidden}, move along");
let extracted = Zwus::decode_to_string(&carrier);
assert_eq!(extracted, "secret");
```

### Full Unicode support

Handles emoji and all of Unicode — anything `char` can represent.

```rust
let encoded = Zwus::encode_string("hello 🦀🔥");
let decoded = Zwus::decode_to_string(&encoded);
assert_eq!(decoded, "hello 🦀🔥");
```

## Interop

Encoded output is byte-identical to the [npm package](https://www.npmjs.com/package/zwus), so you can encode in JS and decode in Rust or vice versa.

## License

[WTFPL](LICENSE)
