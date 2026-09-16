use zwus::Zwus;

#[test]
fn default_base_is_7() {
    let text = "secret 🦀";
    let numbers = [0, 72, 101];
    let encoded_text = Zwus::encode_string_with_base(text, 7);
    let encoded_numbers = Zwus::encode_number_array_with_base(&numbers, 7);

    assert_eq!(Zwus::encode_string(text), encoded_text);
    assert_eq!(Zwus::decode_to_string(&encoded_text), text);
    assert_eq!(Zwus::encode_number_array(&numbers), encoded_numbers);
    assert_eq!(Zwus::decode_to_number_array(&encoded_numbers), numbers);
}

#[test]
fn roundtrip_string_base_3_6_7() {
    let text = "secret 🦀 unicode";
    for base in [3u8, 6, 7] {
        let encoded = Zwus::encode_string_with_base(text, base);
        let decoded = Zwus::decode_to_string_with_base(&encoded, base);
        assert_eq!(decoded, text);
    }
}

#[test]
fn roundtrip_number_array_base_3_6_7() {
    let nums = [0u32, 72, 101, 108, 108, 111, 0x10FFFF];
    for base in [3u8, 6, 7] {
        let encoded = Zwus::encode_number_array_with_base(&nums, base);
        let decoded = Zwus::decode_to_number_array_with_base(&encoded, base);
        assert_eq!(decoded, nums);
    }
}

#[test]
fn decode_ignores_non_zwus_characters() {
    let hidden = Zwus::encode_string_with_base("secret", 6);
    let mixed = format!("te{hidden}xt");
    assert_eq!(Zwus::decode_to_string_with_base(&mixed, 6), "secret");
    let hidden = Zwus::encode_string_with_base("secret", 7);
    assert_eq!(
        Zwus::decode_to_string_with_base(&format!("te{hidden}xt"), 7),
        "secret"
    );
}

#[test]
fn npm_vectors_decode_to_test123_checkmark() {
    // Paste encoded outputs from your npm package here:
    //   zwus.encodeString("Test123!✅", 3)
    //   zwus.encodeString("Test123!✅", 6)
    //   zwus.encodeString("Test123!✅", 7)
    //
    // Keep these as raw strings to preserve invisible chars exactly.
    let npm_base_3 = r#"​᠎᠎​᠎­​᠎‍᠎‍­​​᠎‍​­​​᠎‍‍­​‍​​­​‍​‍­​‍‍᠎­​᠎‍᠎­​​​‍᠎᠎‍‍‍"#;
    let npm_base_6 = r#"­­‍‌­​‎‌⁠‏‏‌⁠‏­‌‏­‏‌‏­­‌‏­⁠‌‎⁠‌‏‏​‏­‎"#;
    let npm_base_7 = "\u{feff}\u{feff}\u{200c}\u{200f}\u{200c}\u{200f}\u{200d}\u{200c}\u{200d}\u{200c}\u{200e}\u{2060}\u{200c}\u{200e}\u{200b}\u{200c}\u{200e}\u{200e}\u{200c}\u{200b}\u{200f}\u{200c}\u{200b}\u{200f}\u{200d}\u{feff}\u{200d}";

    let expected = "Test123!✅";

    assert!(
        !npm_base_3.is_empty(),
        "Paste base-3 encoded payload into npm_base_3"
    );
    assert!(
        !npm_base_6.is_empty(),
        "Paste base-6 encoded payload into npm_base_6"
    );
    assert!(
        !npm_base_7.is_empty(),
        "Paste base-7 encoded payload into npm_base_7"
    );

    assert_eq!(Zwus::decode_to_string_with_base(npm_base_3, 3), expected);
    assert_eq!(Zwus::decode_to_string_with_base(npm_base_6, 6), expected);
    assert_eq!(Zwus::decode_to_string_with_base(npm_base_7, 7), expected);
    assert_eq!(Zwus::encode_string_with_base(expected, 7), npm_base_7);
}
