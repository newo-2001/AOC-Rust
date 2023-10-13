#![feature(
    step_trait,
    return_position_impl_trait_in_trait,
    impl_trait_in_assoc_type
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
pub use errors::*;