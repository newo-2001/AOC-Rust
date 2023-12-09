pub fn generate<T, F>(seed: T, generator: F) -> Generator<T, F>
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