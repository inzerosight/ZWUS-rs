use zwus::Zwus;

#[test]
fn roundtrip_string_base_3_6_8() {
    let text = "secret 🦀 unicode";
    for base in [3u8, 6, 8] {
        let encoded = Zwus::encode_string_with_base(text, base);
        let decoded = Zwus::decode_to_string_with_base(&encoded, base);
        assert_eq!(decoded, text);
    }
}

#[test]
fn roundtrip_number_array_base_3_6_8() {
    let nums = [0u32, 72, 101, 108, 108, 111, 0x10FFFF];
    for base in [3u8, 6, 8] {
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
}
