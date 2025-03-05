//! All the parser combinators you can use.
//!
//! NOTE: When using parsers, if you have yet to use/encounter a custom error, the parser (tries
//! to) default to `DummyError` as the `CustomError`. However, this may need to be specified when
//! using parser combinators. So, specify the error type when using combinators if you don't have
//! a custom error type. If you do, specify it in the `State` and `Return` types, which should then
//! handle everything (see `examples/hex.rs`).

mod any;
mod between;
mod compare;
mod consumed;
mod eoi;
mod id;
mod many;
mod maybe;
mod numeric;
mod recovery;
mod separated;
mod take;
mod whitespace;

pub use any::*;
pub use between::*;
pub use compare::*;
pub use consumed::*;
pub use eoi::*;
pub use id::*;
pub use many::*;
pub use maybe::*;
pub use numeric::*;
pub use recovery::*;
pub use separated::*;
pub use take::*;
pub use whitespace::*;
