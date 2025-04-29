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
