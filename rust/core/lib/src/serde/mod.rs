//! Deserialization deps on serde crate.
//!
//!
//! For convenience, variants re-exports `serde`'s `Deserialize`
//! traits and derive macros from this module. However, due to Rust's limited
//! support for derive macro re-exports, using the re-exported derive macros
//! requires annotating structures with `#[serde(crate = "variants::serde")]`:
//!
//! ```rust
//! use variants_de::serde::Deserialize;
//!
//! #[derive(Deserialize)]
//! #[serde(crate = "variants_de::serde")]
//! struct Config {
//!     key: String,
//! }
//! ```
//!
//! If you'd like to avoid this extra annotation, you must depend on `serde`
//! directly via your crate's `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! serde = { version = "1.0", features = ["derive"] }
//! ```

#[doc(inline)]
pub use serde::de::{Deserialize, DeserializeOwned, Deserializer};

#[doc(hidden)]
pub use serde::*;
