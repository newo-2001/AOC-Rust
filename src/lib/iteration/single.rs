use thiserror::Error;

#[derive(Debug, Error)]
pub enum SingleError {
    #[error("Iterator yielded no elements")] None,
    #[error("Iterator yielded more than one element")] More
}

pub fn single<T>(mut it: impl Iterator<Item=T>) -> Result<T, SingleError> {
    it
        .next()
        .map_or_else(|| Err(SingleError::None), |v| match it.next() {
            None => Ok(v),
            Some(_) => Err(SingleError::More)
        })
}