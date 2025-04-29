use crate::SliceWriter;

pub trait GenerateBytes {
    fn generate<'b>(&'b self, i: &'b mut SliceWriter<'_>);

    /// How much data will be produced?
    fn generated_size(&self) -> usize;
}
