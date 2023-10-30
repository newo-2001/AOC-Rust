#![feature(
    step_trait,
    impl_trait_in_assoc_type,
    never_type
)]

#![allow(
    clippy::module_name_repetitions,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc
)]

pub mod parsing;
pub mod math;
pub mod geometry;
pub mod iteration;
pub mod functional;
pub mod datastructures;

mod errors;
pub mod cpu;
pub use errors::*;