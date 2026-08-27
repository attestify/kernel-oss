//! Checked RFC 3492 Bootstring and Punycode primitives.

#[cfg(test)]
mod tests;

use super::Error;

const BASE: u32 = 36;
const TMIN: u32 = 1;
const TMAX: u32 = 26;
const SKEW: u32 = 38;
const DAMP: u32 = 700;
const INITIAL_BIAS: u32 = 72;
const INITIAL_N: u32 = 128;

/// Encodes one Unicode label without its `xn--` prefix.
pub(super) fn encode(input: &str) -> Result<String, Error> {
    let mut output: String = input
        .chars()
        .filter(|character| character.is_ascii())
        .collect();
    let basic_count = u32::try_from(output.len()).map_err(|_| Error::Overflow)?;
    let input_count = u32::try_from(input.chars().count()).map_err(|_| Error::Overflow)?;
    let mut handled = basic_count;
    if basic_count > 0 {
        output.push('-');
    }
    let mut n = INITIAL_N;
    let mut delta = 0_u32;
    let mut bias = INITIAL_BIAS;
    while handled < input_count {
        let next = input
            .chars()
            .map(u32::from)
            .filter(|code_point| *code_point >= n)
            .min()
            .ok_or(Error::Invalid)?;
        delta = delta
            .checked_add(
                next.checked_sub(n)
                    .ok_or(Error::Overflow)?
                    .checked_mul(handled.checked_add(1).ok_or(Error::Overflow)?)
                    .ok_or(Error::Overflow)?,
            )
            .ok_or(Error::Overflow)?;
        n = next;
        for character in input.chars() {
            let code_point = u32::from(character);
            if code_point < n {
                delta = delta.checked_add(1).ok_or(Error::Overflow)?;
            }
            if code_point != n {
                continue;
            }
            let mut quotient = delta;
            let mut k = BASE;
            loop {
                let threshold = threshold(k, bias)?;
                if quotient < threshold {
                    break;
                }
                output.push(encode_digit(
                    threshold + (quotient - threshold) % (BASE - threshold),
                )?);
                quotient = (quotient - threshold) / (BASE - threshold);
                k = k.checked_add(BASE).ok_or(Error::Overflow)?;
            }
            output.push(encode_digit(quotient)?);
            bias = adapt(
                delta,
                handled.checked_add(1).ok_or(Error::Overflow)?,
                handled == basic_count,
            )?;
            delta = 0;
            handled = handled.checked_add(1).ok_or(Error::Overflow)?;
        }
        delta = delta.checked_add(1).ok_or(Error::Overflow)?;
        n = n.checked_add(1).ok_or(Error::Overflow)?;
    }
    Ok(output)
}

/// Decodes one ASCII Punycode payload without its `xn--` prefix.
pub(super) fn decode(input: &str) -> Result<String, Error> {
    if input.is_empty() || !input.is_ascii() {
        return Err(Error::Invalid);
    }
    let (basic, encoded) = input.rsplit_once('-').unwrap_or(("", input));
    let mut output: Vec<u32> = basic.bytes().map(u32::from).collect();
    let mut n = INITIAL_N;
    let mut index = 0_u32;
    let mut bias = INITIAL_BIAS;
    let bytes = encoded.as_bytes();
    let mut position = 0_usize;
    while position < bytes.len() {
        let old_index = index;
        let mut weight = 1_u32;
        let mut k = BASE;
        loop {
            let byte = *bytes.get(position).ok_or(Error::Invalid)?;
            position = position.checked_add(1).ok_or(Error::Overflow)?;
            let digit = decode_digit(byte)?;
            index = index
                .checked_add(digit.checked_mul(weight).ok_or(Error::Overflow)?)
                .ok_or(Error::Overflow)?;
            let threshold = threshold(k, bias)?;
            if digit < threshold {
                break;
            }
            weight = weight
                .checked_mul(BASE - threshold)
                .ok_or(Error::Overflow)?;
            k = k.checked_add(BASE).ok_or(Error::Overflow)?;
        }
        let output_len = u32::try_from(output.len()).map_err(|_| Error::Overflow)?;
        bias = adapt(
            index.checked_sub(old_index).ok_or(Error::Overflow)?,
            output_len.checked_add(1).ok_or(Error::Overflow)?,
            old_index == 0,
        )?;
        n = n
            .checked_add(index / output_len.checked_add(1).ok_or(Error::Overflow)?)
            .ok_or(Error::Overflow)?;
        if char::from_u32(n).is_none() {
            return Err(Error::Invalid);
        }
        index %= output_len.checked_add(1).ok_or(Error::Overflow)?;
        output.insert(usize::try_from(index).map_err(|_| Error::Overflow)?, n);
        index = index.checked_add(1).ok_or(Error::Overflow)?;
    }
    output
        .into_iter()
        .map(|code_point| char::from_u32(code_point).ok_or(Error::Invalid))
        .collect()
}

fn threshold(k: u32, bias: u32) -> Result<u32, Error> {
    if k <= bias {
        Ok(TMIN)
    } else if k >= bias.checked_add(TMAX).ok_or(Error::Overflow)? {
        Ok(TMAX)
    } else {
        k.checked_sub(bias).ok_or(Error::Overflow)
    }
}

fn adapt(mut delta: u32, num_points: u32, first_time: bool) -> Result<u32, Error> {
    if num_points == 0 {
        return Err(Error::Invalid);
    }
    delta = if first_time { delta / DAMP } else { delta / 2 };
    delta = delta
        .checked_add(delta / num_points)
        .ok_or(Error::Overflow)?;
    let mut k = 0_u32;
    while delta > ((BASE - TMIN) * TMAX) / 2 {
        delta /= BASE - TMIN;
        k = k.checked_add(BASE).ok_or(Error::Overflow)?;
    }
    k.checked_add(
        (BASE - TMIN + 1)
            .checked_mul(delta)
            .ok_or(Error::Overflow)?
            / delta.checked_add(SKEW).ok_or(Error::Overflow)?,
    )
    .ok_or(Error::Overflow)
}

fn encode_digit(value: u32) -> Result<char, Error> {
    match value {
        0..=25 => char::from_u32(u32::from(b'a') + value).ok_or(Error::Invalid),
        26..=35 => char::from_u32(u32::from(b'0') + value - 26).ok_or(Error::Invalid),
        _ => Err(Error::Invalid),
    }
}
fn decode_digit(byte: u8) -> Result<u32, Error> {
    match byte {
        b'a'..=b'z' => Ok(u32::from(byte - b'a')),
        b'A'..=b'Z' => Ok(u32::from(byte - b'A')),
        b'0'..=b'9' => Ok(u32::from(byte - b'0') + 26),
        _ => Err(Error::Invalid),
    }
}
