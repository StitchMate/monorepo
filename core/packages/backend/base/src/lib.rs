#![feature(associated_type_bounds)]

pub mod application;
pub mod config;
pub mod domain;
pub mod infrastructure;

// Export derive macros
#[cfg(feature = "derive")]
pub use base_derive::{Aggregate, Command, Event, EventMetadata, Query};
