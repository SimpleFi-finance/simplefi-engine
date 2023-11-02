//! Collection of metrics utilities.
//!
//! ## Feature Flags
//!
//! - `common`: Common metrics utilities, such as wrappers around tokio senders and receivers. Pulls
//!   in `tokio`.

/// Metrics derive macro.
pub use simp_metrics_derive::Metrics;

/// Implementation of common metric utilities.
#[cfg(feature = "common")]
pub mod common;

/// Re-export core metrics crate.
pub use metrics;
