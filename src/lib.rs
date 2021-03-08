#[cfg(not(feature = "sync"))]
pub mod collection;
pub mod doc;
mod engine;
pub mod errors;
#[cfg(not(feature = "sync"))]
pub mod memdb;
#[cfg(feature = "sync")]
pub mod sync_collection;
#[cfg(feature = "sync")]
pub mod sync_memdb;
mod utils;
