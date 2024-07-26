use crate::{try_const, ParseError};

// MSRV: Use `split_at_checked` (1.80.0) and `split_first_chunk` (1.77.0) in this module.
pub const fn take2(input: &[u8]) -> Result<(&[u8], [u8; 2]), ParseError> {
    if let [a, b, rest @ ..] = input {
        Ok((rest, [*a, *b]))
    } else {
        Err(ParseError::UnexpectedEndOfFile)
    }
}

pub const fn take4(input: &[u8]) -> Result<(&[u8], [u8; 4]), ParseError> {
    if let [a, b, c, d, rest @ ..] = input {
        Ok((rest, [*a, *b, *c, *d]))
    } else {
        Err(ParseError::UnexpectedEndOfFile)
    }
}

pub const fn take_slice(input: &[u8], length: usize) -> Result<(&[u8], &[u8]), ParseError> {
    if length <= input.len() {
        let (value, rest) = input.split_at(length);
        Ok((rest, value))
    } else {
        Err(ParseError::UnexpectedEndOfFile)
    }
}

pub const fn le_u16(input: &[u8]) -> Result<(&[u8], u16), ParseError> {
    let (input, value) = try_const!(take2(input));
    Ok((input, u16::from_le_bytes(value)))
}

pub const fn le_u32(input: &[u8]) -> Result<(&[u8], u32), ParseError> {
    let (input, value) = try_const!(take4(input));
    Ok((input, u32::from_le_bytes(value)))
}

pub const fn le_i32(input: &[u8]) -> Result<(&[u8], i32), ParseError> {
    let (input, value) = try_const!(take4(input));
    Ok((input, i32::from_le_bytes(value)))
}
