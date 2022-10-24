#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub(crate) mod op2;
pub(crate) mod op1;
pub(crate) mod commons;
pub(crate) mod load_fits;
pub(crate) mod load_json;
pub(crate) mod load_ascii;
pub(crate) mod store;
pub use app::FileApp;
