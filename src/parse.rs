use core::convert::TryInto;

pub fn le_uleb128(i: &[u8]) -> Result<(&[u8], u128), ParseError> {
    // I hate VLUs, I hate it, I hate it so much
    let mut res = 0u128;
    let mut shif = 0;
    let mut i = i;
    loop {
        let (j, b) = ne_u8(i)?;
        i = j;
        res |= (b as u128 & 0b01111111) << shif;
        if (b & 0b1000_0000) == 0 {
            break;
        }
        shif += 7;
    }

    Ok((i, res))
}

pub fn le_f64(i: &[u8]) -> Result<(&[u8], f64), ParseError> {
    if i.len() < 8 {
        return Err(ParseError::NoData);
    }
    Ok((&i[8..], f64::from_le_bytes((&i[..8]).try_into().unwrap())))
}

pub fn le_u64(i: &[u8]) -> Result<(&[u8], u64), ParseError> {
    if i.len() < 8 {
        return Err(ParseError::NoData);
    }
    Ok((&i[8..], u64::from_le_bytes((&i[..8]).try_into().unwrap())))
}

pub fn le_u32(i: &[u8]) -> Result<(&[u8], u32), ParseError> {
    if i.len() < 4 {
        return Err(ParseError::NoData);
    }
    Ok((&i[4..], u32::from_le_bytes((&i[..4]).try_into().unwrap())))
}

pub fn le_i32(i: &[u8]) -> Result<(&[u8], i32), ParseError> {
    if i.len() < 4 {
        return Err(ParseError::NoData);
    }
    Ok((&i[4..], i32::from_le_bytes((&i[..4]).try_into().unwrap())))
}

pub fn be_u32(i: &[u8]) -> (&[u8], u32) {
    (&i[4..], u32::from_be_bytes((&i[..4]).try_into().unwrap()))
}

pub fn le_u24(i: &[u8]) -> Result<(&[u8], u32), ParseError> {
    if i.len() < 3 {
        return Err(ParseError::NoData);
    }
    let num = (i[0] as u32) | ((i[1] as u32) << 8) | ((i[2] as u32) << 16);
    Ok((&i[3..], num))
}

pub fn be_u24(i: &[u8]) -> Result<(&[u8], u32), ParseError> {
    if i.len() < 3 {
        return Err(ParseError::NoData);
    }
    let num = (i[2] as u32) | ((i[1] as u32) << 8) | ((i[0] as u32) << 16);
    Ok((&i[3..], num))
}

pub fn le_u16(i: &[u8]) -> Result<(&[u8], u16), ParseError> {
    if i.len() < 2 {
        return Err(ParseError::NoData);
    }
    Ok((&i[2..], u16::from_le_bytes((&i[..2]).try_into().unwrap())))
}

pub fn be_u16(i: &[u8]) -> (&[u8], u16) {
    (&i[2..], u16::from_be_bytes((&i[..2]).try_into().unwrap()))
}

pub fn ne_u8(i: &[u8]) -> Result<(&[u8], u8), ParseError> {
    if i.is_empty() {
        return Err(ParseError::NoData);
    }

    Ok((&i[1..], i[0]))
}

pub fn take(i: &[u8], count: usize) -> Result<(&[u8], &[u8]), ParseError> {
    if i.len() < count {
        return Err(ParseError::NoData);
    }
    Ok((&i[count..], &i[..count]))
}

#[cfg(feature = "alloc")]
extern crate alloc;

use crate::ParseError;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

#[cfg(feature = "alloc")]
pub fn take_vec<T, E, F: Fn(&[u8]) -> Result<(&[u8], T), E>>(
    i: &[u8],
    count: usize,
    func: F,
) -> Result<(&[u8], Vec<T>), E> {
    let mut vec = Vec::with_capacity(count);

    let mut i = i;
    for _ in 0..count {
        let (j, v) = func(i)?;
        vec.push(v);
        i = j;
    }

    Ok((i, vec))
}
