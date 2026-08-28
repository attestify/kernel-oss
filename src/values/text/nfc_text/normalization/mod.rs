// Portions adapted from unicode-normalization 0.1.25,
// commit 7ae3eb01d3c30da0d3c319225b306d8315ceb134.
// Copyright 2012-2015 The Rust Project Developers.
// Licensed under MIT; see third-party/licenses/unicode-normalization-MIT.txt.
// Modified by Attestify in 2026 for a private Unicode 17.0 NFC-only engine.
// Relocated to Kernel OSS without algorithmic rewrite in 2026; see
// docs/nfc-text.md and THIRD_PARTY_NOTICES.md.
//
// This owned implementation keeps only the Unicode 17.0 NFC algorithmic
// portions: canonical decomposition, canonical ordering, and composition.

#[cfg(test)]
mod conformance_tests;

#[cfg(test)]
mod tests;

mod tables;

const S_BASE: u32 = 0xAC00;
const L_BASE: u32 = 0x1100;
const V_BASE: u32 = 0x1161;
const T_BASE: u32 = 0x11A7;
const L_COUNT: u32 = 19;
const V_COUNT: u32 = 21;
const T_COUNT: u32 = 28;
const N_COUNT: u32 = V_COUNT * T_COUNT;
const S_COUNT: u32 = L_COUNT * N_COUNT;
const S_LAST: u32 = S_BASE + S_COUNT - 1;
const L_LAST: u32 = L_BASE + L_COUNT - 1;
const V_LAST: u32 = V_BASE + V_COUNT - 1;
const T_FIRST: u32 = T_BASE + 1;
const T_LAST: u32 = T_BASE + T_COUNT - 1;

pub(crate) fn normalize_nfc(text: &str) -> String {
    if text.is_ascii() {
        return text.to_owned();
    }

    let mut decomposed = Vec::with_capacity(text.len());
    for character in text.chars() {
        decompose_scalar(character as u32, &mut decomposed);
    }
    canonical_order(&mut decomposed);
    compose(&decomposed).into_iter().collect()
}

pub(crate) fn is_nfc(text: &str) -> bool {
    normalize_nfc(text).as_str() == text
}

/// Returns whether the scalar has canonical combining class Virama (9).
pub(crate) fn is_virama(character: char) -> bool {
    canonical_combining_class(character as u32) == 9
}

fn decompose_scalar(scalar: u32, output: &mut Vec<char>) {
    if is_hangul_syllable(scalar) {
        decompose_hangul(scalar, output);
        return;
    }

    if let Some(mapping) = canonical_decomposition(scalar) {
        for mapped_scalar in mapping {
            decompose_scalar(*mapped_scalar, output);
        }
        return;
    }

    if let Some(character) = char::from_u32(scalar) {
        output.push(character);
    }
}

fn canonical_order(characters: &mut [char]) {
    let mut pending_start = 0;
    for index in 0..characters.len() {
        if canonical_combining_class(characters[index] as u32) == 0 {
            characters[pending_start..index]
                .sort_by_key(|character| canonical_combining_class(*character as u32));
            pending_start = index + 1;
        }
    }
    characters[pending_start..]
        .sort_by_key(|character| canonical_combining_class(*character as u32));
}

fn compose(characters: &[char]) -> Vec<char> {
    let mut output = Vec::with_capacity(characters.len());
    let mut starter_index = None;
    let mut last_combining_class = 0;

    for character in characters {
        let combining_class = canonical_combining_class(*character as u32);
        let composed = starter_index.and_then(|index| {
            let starter = output[index] as u32;
            if last_combining_class == 0 || last_combining_class < combining_class {
                compose_pair(starter, *character as u32)
            } else {
                None
            }
        });

        if let Some(composed_scalar) = composed
            && let Some(composed_character) = char::from_u32(composed_scalar)
            && let Some(index) = starter_index
        {
            output[index] = composed_character;
            continue;
        }

        output.push(*character);
        if combining_class == 0 {
            starter_index = Some(output.len() - 1);
        }
        last_combining_class = combining_class;
    }

    output
}

fn canonical_combining_class(scalar: u32) -> u8 {
    tables::CANONICAL_COMBINING_CLASSES
        .binary_search_by_key(&scalar, |entry| entry.0)
        .map_or(0, |index| tables::CANONICAL_COMBINING_CLASSES[index].1)
}

fn canonical_decomposition(scalar: u32) -> Option<&'static [u32]> {
    tables::CANONICAL_DECOMPOSITIONS
        .binary_search_by_key(&scalar, |entry| entry.0)
        .ok()
        .map(|index| tables::CANONICAL_DECOMPOSITIONS[index].1)
}

fn compose_pair(first: u32, second: u32) -> Option<u32> {
    compose_hangul(first, second).or_else(|| {
        tables::CANONICAL_COMPOSITIONS
            .binary_search_by_key(&(first, second), |entry| (entry.0, entry.1))
            .ok()
            .map(|index| tables::CANONICAL_COMPOSITIONS[index].2)
    })
}

fn is_hangul_syllable(scalar: u32) -> bool {
    (S_BASE..=S_LAST).contains(&scalar)
}

fn decompose_hangul(scalar: u32, output: &mut Vec<char>) {
    let syllable_index = scalar - S_BASE;
    let leading = L_BASE + syllable_index / N_COUNT;
    let vowel = V_BASE + (syllable_index % N_COUNT) / T_COUNT;
    let trailing_index = syllable_index % T_COUNT;

    for jamo in [leading, vowel] {
        if let Some(character) = char::from_u32(jamo) {
            output.push(character);
        }
    }
    if trailing_index > 0
        && let Some(character) = char::from_u32(T_BASE + trailing_index)
    {
        output.push(character);
    }
}

fn compose_hangul(first: u32, second: u32) -> Option<u32> {
    match (first, second) {
        (L_BASE..=L_LAST, V_BASE..=V_LAST) => {
            Some(S_BASE + (first - L_BASE) * N_COUNT + (second - V_BASE) * T_COUNT)
        }
        (S_BASE..=S_LAST, T_FIRST..=T_LAST) if (first - S_BASE).is_multiple_of(T_COUNT) => {
            Some(first + second - T_BASE)
        }
        _ => None,
    }
}
