#![warn(clippy::all, rust_2018_idioms)]

pub mod app;
pub(crate) mod commons;
pub mod controllers;
pub mod views;
pub use app::FileApp;
pub mod namestore;