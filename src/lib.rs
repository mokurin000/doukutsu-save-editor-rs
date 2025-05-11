#![warn(clippy::all, rust_2018_idioms)]

mod app;
use std::{future::Future, pin::Pin};

pub use app::MainApp;

#[cfg(not(target_arch = "wasm32"))]
pub static TASK_SENDER: std::sync::OnceLock<kanal::Sender<BoxedFuture<()>>> =
    std::sync::OnceLock::new();

type BoxedFuture<T> = Pin<Box<dyn Send + Future<Output = T>>>;
