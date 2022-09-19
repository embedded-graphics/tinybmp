use crate::ParseError;

pub fn take<const N: usize>(input: &[u8]) -> Result<(&[u8], [u8; N]), ParseError> {
    if let (Some(value), Some(rest)) = (input.get(0..N), input.get(N..)) {
        Ok((rest, value.try_into().unwrap()))
    } else {
        Err(ParseError::UnexpectedEndOfFile)
    }
}

pub fn take_slice(input: &[u8], length: usize) -> Result<(&[u8], &[u8]), ParseError> {
    if let (Some(value), Some(rest)) = (input.get(0..length), input.get(length..)) {
        Ok((rest, value))
    } else {
        Err(ParseError::UnexpectedEndOfFile)
    }
}

pub fn le_u16(input: &[u8]) -> Result<(&[u8], u16), ParseError> {
    let (input, value) = take::<2>(input)?;
    Ok((input, u16::from_le_bytes(value)))
}

pub fn le_u32(input: &[u8]) -> Result<(&[u8], u32), ParseError> {
    let (input, value) = take::<4>(input)?;
    Ok((input, u32::from_le_bytes(value)))
}

pub fn le_i32(input: &[u8]) -> Result<(&[u8], i32), ParseError> {
    le_u32(input).map(|(input, value)| (input, value as i32))
}
