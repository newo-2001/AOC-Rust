pub fn repeat_apply<T>(times: u32, mut seed: T, f: impl Fn(T) -> T) -> T {
    for _ in 0..times {
        seed = f(seed);
    }

    seed
}

pub fn repeat_apply_while<T>(mut seed: T, f: impl Fn(T) -> T, predicate: impl Fn(&T) -> bool) -> T {
    while predicate(&seed) {
        seed = f(seed);
    }

    seed
}

pub fn consume<T, U>(f: impl Fn(&T) -> U) -> impl Fn(T) -> U { move |value| f(&value) }

pub fn consume_mut<T>(f: impl Fn(&mut T)) -> impl Fn(T) -> T {
    move |mut value| {
        f(&mut value);
        value
    }
}

pub fn swap<T, U>((a, b): (T, U)) -> (U, T) { (b, a) }