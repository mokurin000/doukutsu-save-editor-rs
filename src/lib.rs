#![warn(clippy::all, rust_2018_idioms)]

mod app;

#[cfg(not(target_arch = "wasm32"))]
use std::{future::Future, pin::Pin};

pub use app::MainApp;

#[cfg(not(target_arch = "wasm32"))]
pub static TASK_SENDER: std::sync::OnceLock<kanal::Sender<BoxedFuture<()>>> =
    std::sync::OnceLock::new();

#[cfg(not(target_arch = "wasm32"))]
type BoxedFuture<T> = Pin<Box<dyn Send + Future<Output = T>>>;
