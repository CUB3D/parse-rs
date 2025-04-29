pub struct SliceWriter<'a>(&'a mut [u8], usize);

impl<'a> SliceWriter<'a> {
    pub fn new_from(data: &'a mut [u8]) -> SliceWriter<'a> {
        Self(data, 0)
    }

    pub fn len_written(&self) -> usize {
        self.1
    }

    pub fn as_slice_mut(&mut self) -> &mut [u8] {
        self.0
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.0[..self.1]
    }

    pub fn put(&mut self, data: &[u8]) {
        self.0[self.1..self.1 + data.len()].copy_from_slice(data);
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
