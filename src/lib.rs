#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub(crate) mod op2;
pub(crate) mod op1;
pub(crate) mod commons;
pub(crate) mod load;
pub use app::FileApp;
