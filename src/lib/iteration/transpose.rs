use std::iter::Iterator;

pub struct Transpose<Row> where
    Row: Iterator
{
    pub(super) rows: Vec<Row>
}

impl<Row> Iterator for Transpose<Row> where
    Row: Iterator
{
    type Item = Vec<Row::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        self.rows.iter_mut()
            .map(Iterator::next)
            .collect()
    }
}