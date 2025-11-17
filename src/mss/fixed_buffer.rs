pub struct FixedBuffer<T, const N: usize> {
    pub data: [T; N],
    pub len: usize,
}

impl<T: Copy + Default, const N: usize> FixedBuffer<T, N> {
    pub fn new() -> Self {
        Self {
            data: [T::default(); N],
            len: 0,
        }
    }

    pub fn capacity(&self) -> usize {
        self.data.len()
    }

    pub fn set_len(&mut self, size: usize) {
        self.len = core::cmp::min(self.data.len(), size);
    }

    pub fn as_slice(&self) -> &[T] {
        &self.data[..self.len]
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.data[..self.len]
    }

    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.data.as_mut_ptr()
    }
}
