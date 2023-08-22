#![feature(generators)]
#[macro_use] extern crate impl_ops;

#[deprecated(note="Use the more abstract `geometry` module instead")]
pub mod spatial;
pub mod parsing;
pub mod math;
pub mod geometry;