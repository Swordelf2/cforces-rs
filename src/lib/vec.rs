#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Vec2<T> {
    dims: (usize, usize),
    vec: Vec<T>,
}

impl<T: Default> Vec2<T> {
    #[must_use]
    pub fn new(dims: (usize, usize)) -> Self {
        let mut vec = Vec::new();
        vec.resize_with(dims.0 * dims.1, || T::default());
        Self { dims, vec }
    }
}

impl<T> Vec2<T> {
    #[must_use]
    pub fn get(&self, index: (usize, usize)) -> Option<&T> {
        if !(index.0 < self.dims.0 && index.1 < self.dims.1) {
            return None;
        }
        Some(unsafe { self.vec.get_unchecked(index.0 * self.dims.1 + index.1) })
    }

    #[must_use]
    pub fn get_mut(&mut self, index: (usize, usize)) -> Option<&mut T> {
        if !(index.0 < self.dims.0 && index.1 < self.dims.1) {
            return None;
        }
        Some(unsafe { self.vec.get_unchecked_mut(index.0 * self.dims.1 + index.1) })
    }
}

impl<T> std::ops::Index<(usize, usize)> for Vec2<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        assert!(index.0 < self.dims.0, "index 0 out of bounds");
        assert!(index.1 < self.dims.1, "index 1 out of bounds");
        unsafe { self.vec.get_unchecked(index.0 * self.dims.1 + index.1) }
    }
}

impl<T> std::ops::IndexMut<(usize, usize)> for Vec2<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        assert!(index.0 < self.dims.0, "index 0 out of bounds");
        assert!(index.1 < self.dims.1, "index 1 out of bounds");
        unsafe { self.vec.get_unchecked_mut(index.0 * self.dims.1 + index.1) }
    }
}
