pub struct LinearSequence<I>
    where I: Iterator
{
    pub it: I,
    pub step_size: usize,
    pub offset: usize
}

impl<I: Iterator> Iterator for LinearSequence<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.it.nth(match self.offset {
            0 => self.step_size - 1,
            _ => {
                let offset = self.offset;
                self.offset = 0;
                offset
            }
        })
    }
}