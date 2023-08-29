use std::iter::repeat;

pub fn repeat_apply<T>(times: usize, seed: T, f: impl Fn(T) -> T) -> T {
    repeat(())
        .take(times)
        .fold(seed, |value, _| f(value))
}