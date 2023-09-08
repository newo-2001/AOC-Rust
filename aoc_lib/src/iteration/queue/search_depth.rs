use std::hash::Hash;

#[derive(Clone)]
pub struct SearchDepth<T> {
    pub depth: usize,
    pub state: T
}

impl<T: Copy> Copy for SearchDepth<T> {}

impl<T: Hash> Hash for SearchDepth<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.state.hash(state);
    }
}

impl<T: Eq> Eq for SearchDepth<T> {}
impl<T: PartialEq> PartialEq for SearchDepth<T> {
    fn eq(&self, other: &Self) -> bool {
        self.state == other.state
    }
}

impl<T: Default> Default for SearchDepth<T> {
    fn default() -> Self {
        Self { depth: 0, state: T::default() }
    }
}

impl<T> SearchDepth<T> {
    pub fn new(state: T) -> Self {
        Self { depth: 0, state }
    }

    #[must_use]
    pub fn with(&self, state: T) -> Self {
        Self { depth: self.depth + 1, state }
    }
}