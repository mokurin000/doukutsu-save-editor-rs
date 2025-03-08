#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub use app::MainApp;

#[cfg(not(target_arch = "wasm32"))]
pub static TOKIO_HANDLE: std::sync::OnceLock<tokio::runtime::Handle> = std::sync::OnceLock::new();
