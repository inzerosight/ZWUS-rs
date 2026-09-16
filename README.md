# Zero Width Unicode Standard (ZWUS)

Zero Width Unicode Steganography — hide text inside invisible characters.

```toml
[dependencies]
zwus = "0.3"
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

Base 7 ranks printable ASCII so common English characters use one or two digits. Base 6 keeps Unicode code points as numbers. Some platforms strip particular zero-width characters.

```rust
use zwus::Zwus;

Zwus::encode_string_with_base("hi", 3); // safest
Zwus::encode_string_with_base("hi", 6); // code point encoding
Zwus::encode_string_with_base("hi", 7); // default, compact for ordinary English text
```

Decode must match the encode base:

```rust
use zwus::Zwus;

let encoded = Zwus::encode_string_with_base("hi", 6);
Zwus::decode_to_string_with_base(&encoded, 6);
```

Number arrays use ordinary base digits in every standard. Base 7's frequency ranking applies only to strings. Decoding ignores visible text mixed into a payload.

### Embedded in visible text

Non-ZWUS characters are automatically ignored during decoding, so hidden payloads survive being mixed into normal text.

```rust
use zwus::Zwus;

let hidden = Zwus::encode_string("secret");
let carrier = format!("nothing to see here{hidden}, move along");
let extracted = Zwus::decode_to_string(&carrier);
assert_eq!(extracted, "secret");
```

### Full Unicode support

Handles emoji and all of Unicode — anything `char` can represent.

```rust
use zwus::Zwus;

let encoded = Zwus::encode_string("hello 🦀🔥");
let decoded = Zwus::decode_to_string(&encoded);
assert_eq!(decoded, "hello 🦀🔥");
```

## Interop

Encoded output is byte-identical to the [npm package](https://www.npmjs.com/package/zwus), so you can encode in JS and decode in Rust or vice versa.

## License

[WTFPL](LICENSE)

## Browser Extension

You can also use ZWUS in your browser with [inØsight](https://github.com/inzerosight/inzerosight): [Firefox Add-on](https://addons.mozilla.org/en-US/firefox/addon/in0sight/) · [Chrome Web Store](https://chromewebstore.google.com/detail/acnmohbphjmnbaboacmecidopeplkhog)
