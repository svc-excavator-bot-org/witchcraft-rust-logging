//! A structured logging facade for Witchcraft servers.
//!
//! While modeled after the standard Rust `log` library, `witchcraft-log` differs in some key ways.
//!
//! # Structured Logging
//!
//! Witchcraft logs are *structured*. Rather than including runtime information by interpolating them into the log
//! message, information is included via a separate set of parameters. Parameters are partitioned into "safe" parameters
//! and "unsafe" parameters. Safety in this context is *not* safety in the traditional Rust sense of memory safety, but
//! instead safety against information leakage. Safe parameters do not contain any sensitive information about use of a
//! service, and can be exfiltrated from a specific environment, while unsafe parameters contain sensitive information
//! that should not leave the environment at all. For example, the amount of memory used to process a request could be
//! a safe parameter, while information about the user executing the request could be an unsafe parameter.
//!
//! Parameters can be arbitrary `serde`-serializable values. Note, however, that loggers may commonly serialize
//! parameters to JSON, so values that cannot be serialized into JSON are not recommended.
//!
//! All dynamic information in the log record should be represented via parameters. In fact, Witchcraft-log requires the
//! log message to be a static string - no interpolation of any kind can be performed. This means that the message
//! itself can always be considered safe.
//!
//! ## Examples
//!
//! ```
//! # let (user_id, memory_overhead) = ("", "");
//! // with the standard log crate
//! log::info!("ran a request for {} using {} bytes of memory", user_id, memory_overhead);
//!
//! // with the witchcraft-log crate
//! witchcraft_log::info!("ran a request", safe: { memory: memory_overhead }, unsafe: { user: user_id });
//! ```
//!
//! # Errors
//!
//! Additionally, a `conjure_error::Error` can be associated with a log message. Since many logs occur due to an error,
//! this allows more information about the error (e.g. its stacktrace) to be automatically included in the record.
//!
//! ## Examples
//!
//! ```
//! # fn shave_a_yak(_: ()) -> Result<(), conjure_error::Error> { Ok(()) }
//! # let my_yak = ();
//! if let Err(e) = shave_a_yak(my_yak) {
//!     witchcraft_log::warn!("error shaving a yak", safe: { yak: my_yak }, error: e);
//! }
//! ```
//!
//! # Bridging
//!
//! Even when an application is using `witchcraft-log`, many of its dependencies may still use the `log` crate. The
//! `bridge` module provides functionality to forward records from the `log` crate to `witchcraft-log`.
#![warn(missing_docs)]

pub use crate::level::*;
pub use crate::logger::*;
pub use crate::record::*;

pub mod bridge;
mod level;
mod logger;
#[macro_use]
mod macros;
#[doc(hidden)]
pub mod private;
mod record;