use vec1::Vec1;

pub struct VecWithCursor<T> {
    vec:    Vec1<T>,
    cursor: usize,
}

impl<T> From<Vec1<T>> for VecWithCursor<T> {
    fn from(vec: Vec1<T>) -> Self { Self { vec, cursor: 0 } }
}

impl<T> VecWithCursor<T> {
    pub fn inner(&self) -> &Vec1<T> { &self.vec }

    #[allow(unused)]
    pub fn get_mut(&mut self) -> &mut T {
        // We guarantee cursor is in vec bounds, let's take advantage of that
        unsafe { self.vec.get_unchecked_mut(self.cursor) }
    }

    pub fn get(&self) -> &T {
        // We guarantee cursor is in vec bounds, let's take advantage of that
        unsafe { self.vec.get_unchecked(self.cursor) }
    }

    pub fn cursor(&self) -> usize { self.cursor }

    /// If on the last element, go to the first.
    pub fn cursor_increment(&mut self) { self.cursor = (self.cursor + 1) % self.vec.len(); }

    /// If on the first element, go to the last.
    pub fn cursor_decrement(&mut self) {
        self.cursor = (self.cursor + self.vec.len() - 1) % self.vec.len();
    }
}
