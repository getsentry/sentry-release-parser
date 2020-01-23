//! This package implements a release name parser that is used by Sentry.
//!
//! # Features
//!
//! - `semver`: if enabled the version object provides a method to convert it
//!   into a semver if it's compatible.
mod parser;

pub use self::parser::*;
