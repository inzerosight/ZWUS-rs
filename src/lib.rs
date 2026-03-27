#![doc = include_str!("../README.md")]

pub const DEFAULT_BASE: u8 = 3;
pub const SUPPORTED_BASES: [u8; 3] = [3, 6, 8];

#[derive(Clone, Copy)]
struct Alphabet {
    unifier: char,
    digits: &'static [char],
}

const BASE_3: Alphabet = Alphabet {
    unifier: '\u{00AD}',
    digits: &['\u{180E}', '\u{200B}', '\u{200D}'],
};

const BASE_6: Alphabet = Alphabet {
    unifier: '\u{200C}',
    digits: &[
        '\u{200D}', '\u{200F}', '\u{00AD}', '\u{2060}', '\u{200B}', '\u{200E}',
    ],
};

const BASE_8: Alphabet = Alphabet {
    unifier: '\u{200C}',
    digits: &[
        '\u{200D}', '\u{200F}', '\u{00AD}', '\u{2060}', '\u{200B}', '\u{200E}', '\u{180E}',
        '\u{FEFF}',
    ],
};

#[inline]
fn alphabet(base: u8) -> Alphabet {
    match base {
        3 => BASE_3,
        6 => BASE_6,
        8 => BASE_8,
        _ => panic!("Unsupported base {base}. Use 3, 6, or 8."),
    }
}

#[inline]
fn to_base_digits(mut n: u32, base: u32) -> Vec<usize> {
    if n == 0 {
        return vec![0];
    }
    let mut out = Vec::new();
    while n > 0 {
        out.push((n % base) as usize);
        n /= base;
    }
    out.reverse();
    out
}

fn encode_numbers<I>(numbers: I, base: u8) -> String
where
    I: IntoIterator<Item = u32>,
{
    let abc = alphabet(base);
    let mut out = String::new();

    for (i, n) in numbers.into_iter().enumerate() {
        if i > 0 {
            out.push(abc.unifier);
        }
        for d in to_base_digits(n, base as u32) {
            out.push(abc.digits[d]);
        }
    }

    out
}

#[inline]
fn retain_zwus_chars(text: &str, abc: Alphabet) -> String {
    text.chars()
        .filter(|c| *c == abc.unifier || abc.digits.contains(c))
        .collect()
}

#[inline]
fn decode_chunk(chunk: &str, abc: Alphabet, base: u8) -> Option<u32> {
    chunk.chars().try_fold(0u32, |acc, ch| {
        let digit = abc.digits.iter().position(|&x| x == ch)? as u32;
        acc.checked_mul(base as u32)?.checked_add(digit)
    })
}

fn decode_numbers(text: &str, base: u8) -> Vec<u32> {
    let abc = alphabet(base);
    let filtered = retain_zwus_chars(text, abc);

    filtered
        .split(abc.unifier)
        .filter(|chunk| !chunk.is_empty())
        .filter_map(|chunk| decode_chunk(chunk, abc, base))
        .collect()
}

/// Zero Width Unicode Standard (ZWUS)
pub struct Zwus;

impl Zwus {
    /// Encode a string using base 3 (default/safest).
    pub fn encode_string(text: &str) -> String {
        Self::encode_string_with_base(text, DEFAULT_BASE)
    }

    /// Encode a string using base 3, 6, or 8.
    pub fn encode_string_with_base(text: &str, base: u8) -> String {
        encode_numbers(text.chars().map(|c| c as u32), base)
    }

    /// Encode numbers using base 3 (default/safest).
    pub fn encode_number_array(numbers: &[u32]) -> String {
        Self::encode_number_array_with_base(numbers, DEFAULT_BASE)
    }

    /// Encode numbers using base 3, 6, or 8.
    pub fn encode_number_array_with_base(numbers: &[u32], base: u8) -> String {
        encode_numbers(numbers.iter().copied(), base)
    }

    /// Decode to string using base 3 (default/safest).
    /// Non-ZWUS chars are ignored automatically.
    pub fn decode_to_string(text: &str) -> String {
        Self::decode_to_string_with_base(text, DEFAULT_BASE)
    }

    /// Decode to string using base 3, 6, or 8.
    /// Non-ZWUS chars are ignored automatically.
    pub fn decode_to_string_with_base(text: &str, base: u8) -> String {
        decode_numbers(text, base)
            .into_iter()
            .filter_map(char::from_u32)
            .collect()
    }

    /// Decode to numbers using base 3 (default/safest).
    /// Non-ZWUS chars are ignored automatically.
    pub fn decode_to_number_array(text: &str) -> Vec<u32> {
        Self::decode_to_number_array_with_base(text, DEFAULT_BASE)
    }

    /// Decode to numbers using base 3, 6, or 8.
    /// Non-ZWUS chars are ignored automatically.
    pub fn decode_to_number_array_with_base(text: &str, base: u8) -> Vec<u32> {
        decode_numbers(text, base)
    }
}
