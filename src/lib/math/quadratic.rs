use num::traits::real::Real;

pub struct Quadratic<T = f64> {
    pub a: T,
    pub b: T,
    pub c: T,
}

pub enum Roots<T> {
    None,
    Single(T),
    Pair(T, T)
}

impl<T> Quadratic<T> {
    pub fn roots(self) -> Roots<T> where T: Real {
        let Self { a, b, c } = self;

        let discriminant = b * b - T::from(4).unwrap() * a * c;
        if discriminant.is_zero() { return Roots::None }

        let root = discriminant.sqrt();
        let denominator = T::from(2).unwrap() * a;

        let first = (-b - root) / denominator;        
        if discriminant.is_sign_negative() { return Roots::Single(first) }

        let second = (-b + root) / denominator;
        Roots::Pair(first, second)
    }
}