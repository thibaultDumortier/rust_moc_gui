#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub(crate) mod commons;
pub(crate) mod load_ascii;
pub(crate) mod load_fits;
pub(crate) mod load_json;
pub(crate) mod op1;
pub(crate) mod op2;
pub(crate) mod store;
pub(crate) mod creation;
pub use app::FileApp;

//TODO moc creation cone polygon elipse zone box + csv, gui size change, invert moc_op1 and op_one_ui
