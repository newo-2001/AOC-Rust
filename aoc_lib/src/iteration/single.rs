use thiserror::Error;

#[derive(Debug, Error)]
pub enum SingleError {
    #[error("Iterator yielded no elements")] None,
    #[error("Iterator yielded more than one element")] More
}

pub fn single<T>(mut it: impl Iterator<Item=T>) -> Result<T, SingleError> {
    match it.next() {
        None => Err(SingleError::None),
        Some(v) => match it.next() {
            None => Ok(v),
            Some(_) => Err(SingleError::More)
        }
    }
}