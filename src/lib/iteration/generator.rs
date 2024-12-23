/// Produces an iterator from a generator function.
///
/// The generator function will be called with the previously produced value
/// to produce the next value in the sequence.
/// It turns a recursive function into an iterator.
pub const fn generate<T, F>(seed: T, generator: F) -> Generator<T, F>
    where F: Fn(&T) -> Option<T>
{
    Generator { state: Some(seed), generator }
}

pub struct Generator<T, F> {
    state: Option<T>,
    generator: F
}

impl<T, F> Iterator for Generator<T, F>
    where F: Fn(&T) -> Option<T>
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.state.take();

        if let Some(current) = &current {
            self.state = (self.generator)(current);
        }

        current
    }
}