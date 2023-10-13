use std::{ops::{Index, IndexMut}, fmt::Debug, collections::VecDeque};

#[derive(Default, Hash, Clone, PartialEq, Eq)]
pub struct GrowableRingBuffer<T> {
    buffer: VecDeque<T>
}

impl<T> GrowableRingBuffer<T> {
    #[must_use]
    pub fn wrap_index(&self, index: usize) -> usize {
        index % self.len()
    }

    #[must_use]
    pub fn new() -> Self {
        GrowableRingBuffer { buffer: VecDeque::new() }
    }

    #[must_use]
    pub fn get(&self, index: usize) -> Option<&T> {
        self.buffer.get(self.wrap_index(index))
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        let index = self.wrap_index(index);
        self.buffer.get_mut(index)
    }

    pub fn insert(&mut self, index: usize, element: T) {
        let index = self.wrap_index(index);
        self.buffer.insert(index, element);
    }

    pub fn remove(&mut self, index: usize) {
        self.buffer.remove(self.wrap_index(index));
    }
    
    #[must_use]
    pub fn iter(&self) -> std::collections::vec_deque::Iter<T> { self.buffer.iter() }

    #[must_use]
    pub fn len(&self) -> usize { self.buffer.len() }

    #[must_use]
    pub fn is_empty(&self) -> bool { self.buffer.is_empty() }
}

impl<T> IntoIterator for GrowableRingBuffer<T> {
    type Item = T;
    type IntoIter = std::collections::vec_deque::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.buffer.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a GrowableRingBuffer<T> {
    type Item = &'a T;
    type IntoIter = std::collections::vec_deque::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.buffer.iter()
    }
}

impl<T> FromIterator<T> for GrowableRingBuffer<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self {
            buffer: iter.into_iter().collect()
        }
    }
}

impl<T> Index<usize> for GrowableRingBuffer<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.buffer[self.wrap_index(index)]
    }
}

impl<T> IndexMut<usize> for GrowableRingBuffer<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let index = self.wrap_index(index);
        &mut self.buffer[index]
    }
}

impl<T: Debug> Debug for GrowableRingBuffer<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.buffer)
    }
}