// Copyright (c) 2025 mokurin000
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

#[cfg(not(target_arch = "wasm32"))]
mod native;
#[cfg(target_arch = "wasm32")]
mod web;

pub use native::Storage;

pub trait StorageIO {
    fn drag_handle(&mut self, ctx: &egui::Context);
    fn try_read_data(&mut self) -> Option<Vec<u8>>;
    fn try_write_data(&self, data: &[u8]);
    fn open_dialog(&self, ctx: &egui::Context);
}
