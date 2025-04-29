pub trait ParseBytes<'a> {
    #[allow(clippy::result_unit_err)]
    fn parse(i: &'a [u8]) -> Result<(&'a [u8], Self), ()>
    where
        Self: Sized;
}
