pub fn repeat_apply<T>(times: usize, mut seed: T, f: impl Fn(T) -> T) -> T {
    for _ in 0..times {
        seed = f(seed)
    }

    seed
}

pub fn repeat_apply_while<T>(mut seed: T, f: impl Fn(T) -> T, predicate: impl Fn(&T) -> bool) -> T {
    while predicate(&seed) {
        seed = f(seed)
    }

    seed
}

pub fn swap<T, U>((a, b): (T, U)) -> (U, T) { (b, a) }