use core::convert::TryInto;

#[derive(Debug)]
pub enum ParseError {
    NoData,
}

impl core::fmt::Display for ParseError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl core::error::Error for ParseError {}

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

pub fn be_u32(i: &[u8]) -> (&[u8], u32) {
    (&i[4..], u32::from_be_bytes((&i[..4]).try_into().unwrap()))
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
    if i.len() < 1 {
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

#[cfg(feature = "alloc")]
use alloc::vec::Vec;

#[cfg(feature = "alloc")]
pub fn take_vec<T, E, F: Fn(&[u8])->Result<(&[u8], T), E>>(i: &[u8], count: usize, func: F) -> Result<(&[u8], Vec<T>), E> {
    let mut vec = Vec::with_capacity(count);

    let mut i = i;
    for _ in 0..count {
        let (j, v) = func(i)?;
        vec.push(v);
        i = j;
    }

    Ok((i, vec))
}

pub trait ParseBytes<'a> {
    fn parse(i: &'a [u8]) -> Result<(&'a [u8], Self), ()>   where Self:  Sized;
}

pub struct SliceWriter<'a>(&'a mut [u8], usize);

impl<'a> SliceWriter<'a> {
    pub fn new_from(data: &'a mut [u8]) -> SliceWriter<'a> {
        Self(data, 0)
    }

    pub fn len_written(&self) -> usize { self.1}

    pub fn as_slice_mut(&mut self) -> &mut[u8] {
        self.0
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.0[..self.1]
    }

    pub fn put(&mut self, data: &[u8]) {
        (&mut self.0[self.1..self.1+data.len()]).copy_from_slice(data);
        self.1 += data.len();
    }

    pub fn le_u16(&mut self, v: u16) {
        self.put(&v.to_le_bytes());
    }

    pub fn be_u16(&mut self, v: u16) {
        self.put(&v.to_be_bytes());
    }

    pub fn be_u32(&mut self, v: u32) {
        self.put(&v.to_be_bytes());
    }

    #[deprecated]
    pub fn le_u8(&mut self, v: u8) {
        self.put(&v.to_le_bytes());
    }
    
    pub fn ne_u8(&mut self, v: u8) {
        self.put(&v.to_le_bytes());
    }
}

pub trait GenerateBytes {
    fn generate<'a, 'b>(&'b self, i: &'b mut SliceWriter<'a>);

    /// How much data will be produced?
    fn generated_size(&self) -> usize;
}
