pub enum TryFoldWhile<T, E> {
    Done(T),
    Continue(T),
    Err(E)
}

pub fn try_fold_while<I, T, E, F>(iter: I, mut state: T, folder: F) -> Result<T, E>
    where F: Fn(T, I::Item) -> TryFoldWhile<T, E>,
          I: Iterator
{
    for item in iter {
        state = match (folder)(state, item) {
            TryFoldWhile::Done(result) => return Ok(result),
            TryFoldWhile::Err(err) => return Err(err),
            TryFoldWhile::Continue(state) => state
        };
    }

    Ok(state)
}