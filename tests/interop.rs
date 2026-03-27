use zwus::Zwus;

/// Torture-test corpus — every Unicode edge case.
/// ><  on the last line is where npm-encoded "test" (base 6) gets pasted.
const CORPUS: &str = "\
ASCII: Hello, World! 0123456789 ~!@#$%^&*()_+-=[]{}|;:'\",.<>?/\n\
CJK Unified: 你好世界测试汉字\n\
CJK Extension B: 𠀀𠀁𠀂\n\
Korean: 한국어 테스트\n\
Japanese: こんにちは カタカナ テスト\n\
Emoji single: ✅❌🔥💀🎉🦀⭐🌍\n\
Surrogate-heavy: 𐐷𐑌𐑀 𝄞\n\
Math: ∑∏∫∂∇ ℵ ∞ ≠ ≤ ≥\n\
Newlines:\n\
Line2\n\
Line3\n\
Tabs:\tcol1\tcol2\tcol3\n\
RTL Arabic: مرحبا بالعالم\n\
RTL Hebrew: שלום עולם\n\
Mixed: Hello مرحبا World שלום Fin\n\
Skin tones: 👋🏻👋🏼👋🏽👋🏾👋🏿\n\
Flags: 🇺🇸🇯🇵🇩🇪\n\
Cuneiform: 𒀀𒀁𒀂\n\
Egyptian: 𓀀𓀁𓀂\n\
Musical: 𝄞𝄢\n\
Repeated: 😀😀😀😀😀\n\
Single: x\n\
Zero: 0\n\
Spaces:   multiple   spaces   here\n\
Nested ZW: >⁠‏­‌­​‎‌⁠‏‏‌⁠‏­<";

// ── Paste encoded output from npm here ─────────────────────────────
// In Node:
//   import zwus from 'zwus';
//   console.log(JSON.stringify(zwus.encodeString(CORPUS, 3)));
//   console.log(JSON.stringify(zwus.encodeString(CORPUS, 6)));
//   console.log(JSON.stringify(zwus.encodeString(CORPUS, 8)));
const NPM_BASE_3: &str = "PASTE_HERE";
const NPM_BASE_6: &str = "PASTE_HERE";
const NPM_BASE_8: &str = "PASTE_HERE";

// ── Encoding match: Rust encode == npm encode ──────────────────────

#[test]
fn rust_encode_matches_npm_base3() {
    assert_ne!(NPM_BASE_3, "PASTE_HERE", "Paste npm base-3 encoded output");
    let rust_encoded = Zwus::encode_string_with_base(CORPUS, 3);
    assert_eq!(rust_encoded, NPM_BASE_3, "Base-3 encoding differs from npm");
}

#[test]
fn rust_encode_matches_npm_base6() {
    assert_ne!(NPM_BASE_6, "PASTE_HERE", "Paste npm base-6 encoded output");
    let rust_encoded = Zwus::encode_string_with_base(CORPUS, 6);
    assert_eq!(rust_encoded, NPM_BASE_6, "Base-6 encoding differs from npm");
}

#[test]
fn rust_encode_matches_npm_base8() {
    assert_ne!(NPM_BASE_8, "PASTE_HERE", "Paste npm base-8 encoded output");
    let rust_encoded = Zwus::encode_string_with_base(CORPUS, 8);
    assert_eq!(rust_encoded, NPM_BASE_8, "Base-8 encoding differs from npm");
}

// ── Decoding match: Rust decodes npm output back to CORPUS ─────────

#[test]
fn rust_decodes_npm_base3() {
    assert_ne!(NPM_BASE_3, "PASTE_HERE", "Paste npm base-3 encoded output");
    let decoded = Zwus::decode_to_string_with_base(NPM_BASE_3, 3);
    assert_eq!(decoded, CORPUS);
}

#[test]
fn rust_decodes_npm_base6() {
    assert_ne!(NPM_BASE_6, "PASTE_HERE", "Paste npm base-6 encoded output");
    let decoded = Zwus::decode_to_string_with_base(NPM_BASE_6, 6);
    assert_eq!(decoded, CORPUS);
}

#[test]
fn rust_decodes_npm_base8() {
    assert_ne!(NPM_BASE_8, "PASTE_HERE", "Paste npm base-8 encoded output");
    let decoded = Zwus::decode_to_string_with_base(NPM_BASE_8, 8);
    assert_eq!(decoded, CORPUS);
}

// ── Rust self-roundtrip ────────────────────────────────────────────

#[test]
fn rust_roundtrip_corpus_all_bases() {
    for base in [3u8, 6, 8] {
        let encoded = Zwus::encode_string_with_base(CORPUS, base);
        let decoded = Zwus::decode_to_string_with_base(&encoded, base);
        assert_eq!(decoded, CORPUS, "Roundtrip failed for base {base}");
    }
}

// ── Nested zero-width: encode the whole corpus, then double-decode ─
// The CORPUS itself contains base-6 encoded "test" between > and <.
// First decode peels the outer layer → reveals visible text + inner ZW.
// Second decode (base 6) on that inner ZW region → "test".

#[test]
fn nested_zero_width_double_decode() {
    assert_ne!(NPM_BASE_3, "PASTE_HERE", "Paste npm base-3 encoded output");

    // Outer layer: decode the full corpus from base 3
    let outer_decoded = Zwus::decode_to_string_with_base(NPM_BASE_3, 3);
    assert_eq!(outer_decoded, CORPUS);

    // Find the nested ZW payload between > and <
    let start = outer_decoded.find('>').expect("missing >") + 1;
    let end = outer_decoded.rfind('<').expect("missing <");
    let inner_payload = &outer_decoded[start..end];

    // Inner layer: that region contains base-6 encoded "test"
    // (pasted by Master into CORPUS between > and <)
    assert!(
        !inner_payload.is_empty() && inner_payload != "PASTE_ZW_HERE",
        "Paste base-6 encoded 'test' between >< in CORPUS"
    );
    let inner_decoded = Zwus::decode_to_string_with_base(inner_payload, 6);
    assert_eq!(inner_decoded, "test", "Inner nested ZW did not decode to 'test'");
}

// ── Surrogate safety ──────────────────────────────────────────────

#[test]
fn no_surrogates_in_codepoint_iteration() {
    let codepoints: Vec<u32> = CORPUS.chars().map(|c| c as u32).collect();
    for &cp in &codepoints {
        assert!(
            !(0xD800..=0xDFFF).contains(&cp),
            "Surrogate in Rust char iteration: U+{cp:04X}"
        );
        assert!(cp <= 0x10FFFF, "Out of Unicode range: U+{cp:04X}");
    }
}

// ── Number array roundtrip ────────────────────────────────────────

#[test]
fn number_array_roundtrip_all_bases() {
    let codepoints: Vec<u32> = CORPUS.chars().map(|c| c as u32).collect();
    for base in [3u8, 6, 8] {
        let encoded = Zwus::encode_number_array_with_base(&codepoints, base);
        let decoded = Zwus::decode_to_number_array_with_base(&encoded, base);
        assert_eq!(decoded, codepoints, "Number array roundtrip failed base {base}");
    }
}
