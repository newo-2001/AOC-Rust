pub fn repeat_apply<T>(times: usize, mut seed: T, f: impl Fn(T) -> T) -> T {
    for _ in 0..times {
        seed = f(seed)
    }

    seed
}