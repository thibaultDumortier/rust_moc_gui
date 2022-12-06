#![warn(clippy::all, rust_2018_idioms)]

pub mod app;
pub mod op;
pub mod loaders;
pub mod uis;
pub(crate) mod commons;
pub(crate) mod infoui;
pub use app::FileApp;