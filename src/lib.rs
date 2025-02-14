#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub use app::TextureViewer;

pub mod codecs;
pub mod graphics;
pub mod io;
