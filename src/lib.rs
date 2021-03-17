// #[cfg(not(feature = "sync"))]
pub mod collection;
pub mod doc;
mod engine;
pub mod errors;
pub mod memdb;
mod utils;

pub use engine::{DocumentCollection, Documents};
