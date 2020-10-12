//! # XVII
//!
//! ...Pronounced any way you like--including "seventeen."
//!
//! This library provides parsing and formatting for Roman numerals. According to my
//! (probably extremely suspect) benchmarks, this is the best-performing library of
//! its kind available on crates.io (you know, as of St. Patrick's Day, 2017 when I
//! did the tests), so you should definitely employ it in your high-availability NAAS
//! (numerals-as-a-service) project.
//!
//! (Seriously, though, read the license--no warranties implied!)
//!
//! ```rust
//! # use xvii::Roman;
//! let seventeen: Roman = "XVII".parse().unwrap();
//! assert_eq!(17, seventeen.value());
//! assert_eq!("XVII", seventeen.to_string());
//!
//! let seventeen = Roman::new(17).unwrap();
//! assert_eq!(17, seventeen.value());
//! assert_eq!("XVII", seventeen.to_string());
//! ```
//!
//! # `no_std` support
//!
//! No-std mode is supported, **unless** `std` crate feature is enabled
//!
//! ```toml
//! xvii = { version = "...", features = ["std"] }
//! ```
#![cfg_attr(not(any(feature = "std", test)), no_std)]
// To build docs properly, run
// `RUSTFLAGS="--cfg docsrs" cargo +nightly doc --all-features`
#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(
    unsafe_code,
    missing_docs,
    missing_debug_implementations,
    broken_intra_doc_links
)]

mod error;
mod roman;
mod unit;

pub use error::Error;
pub use roman::{Roman, RomanFormatter, Style};

/// [`Result`](std::result::Result) with error defaulted to [`xvii::Error`](Error)
pub type Result<T, E = Error> = core::result::Result<T, E>;
