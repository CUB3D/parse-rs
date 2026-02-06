#[derive(Debug)]
pub enum ParseError {
    /// Tried to read from an empty slice
    NoData,
    
    /// Stuck in a loop, without progressing
    NoProgress,
    
    /// Failed to read a utf8 string
    InvalidString,
}

impl core::fmt::Display for ParseError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl core::error::Error for ParseError {}
