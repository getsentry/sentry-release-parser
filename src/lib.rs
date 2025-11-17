//! This package implements a release name parser that is used by Sentry.
//!
//! ```
//! use sentry_release_parser::Release;
//!
//! let release = Release::parse("org.example.FooApp@1.0rc1+20200101100").unwrap();
//! assert_eq!(release.package(), Some("org.example.FooApp"));
//! assert_eq!(release.version_raw(), "1.0rc1+20200101100");
//!
//! let version = release.version().unwrap();
//! assert_eq!(version.major(), 1);
//! assert_eq!(version.minor(), 0);
//! assert_eq!(version.patch(), 0);
//! assert_eq!(version.triple(), (1, 0, 0));
//! assert_eq!(version.pre(), Some("rc1"));
//! assert_eq!(version.build_code(), Some("20200101100"));
//! ```
//!
//! # Features
//!
//! - `semver`: if enabled the version object provides a method to convert it
//!   into a semver (0.9 API) if it's compatible.
//! - `semver-1`: if enabled the version object provide a method to convert it
//!   into a semver (1.0+ API) if it's compatible.
//! - `serde`: turns on serde serialization.
mod parser;

pub use self::parser::*;
