//! All the parser combinators you can use.
//!
//! NOTE: When using parsers, if you have yet to use/encounter a custom error, the parser (tries
//! to) default to `()` as the `CustomError`. However, this may need to be specified when using
//! parser combinators. So, specify the error type when using combinators if you don't have a
//! custom error type. If you do, specify it in the `State` and `Return` types, which should then
//! handle everything (see `examples/hex.rs`).

mod any;
mod between;
mod eoi;
mod id;
mod is;
mod many;
mod maybe;
mod numeric;

pub use any::*;
pub use between::*;
pub use eoi::*;
pub use id::*;
pub use is::*;
pub use many::*;
pub use maybe::*;
pub use numeric::*;
