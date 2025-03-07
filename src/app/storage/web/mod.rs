// Copyright (c) 2025 mokurin000
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::sync::mpsc::{self, Receiver, Sender};

pub struct Storage {
    data: Option<Vec<u8>>,
    data_sender: Sender<Vec<u8>>,
    data_recv: Receiver<Vec<u8>>,
}
impl Default for Storage {
    fn default() -> Self {
        let (sender, recv) = mpsc::channel();
        Self {
            data: Default::default(),
            data_sender: sender,
            data_recv: recv,
        }
    }
}

impl super::StorageIO for Storage {
    fn drag_handle(&mut self, ctx: &egui::Context) {
        let dragged_path: Option<_> = ctx.input(|i| {
            let dropped_files = &i.raw.dropped_files;
            let file = dropped_files.get(0)?;
            let bytes = file.bytes.clone()?;
            Some(bytes.to_vec())
        });

        if let Some(data) = dragged_path {
            let _ = self.data_sender.send(data);
            ctx.input_mut(|i| i.raw.dropped_files.clear());
        }
    }

    fn try_read_data(&mut self) -> Option<Vec<u8>> {
        while let Ok(data) = self.data_recv.try_recv() {
            self.data = Some(data);
        }

        self.data.take()
    }

    fn try_write_data(&self, data: &[u8]) {
        let data: Vec<u8> = data.into();
        let future = async move {
            if let Some(handle) = rfd::AsyncFileDialog::default()
                .add_filter("Profile", &["dat"])
                .set_title("Pick your game profile")
                .set_file_name("profile.dat")
                .save_file()
                .await
            {
                let _ = handle.write(&data).await;
            }
        };

        let _ = poll_promise::Promise::spawn_local(future);
    }

    fn open_dialog(&self, _ctx: &egui::Context) {
        let sender = self.data_sender.clone();
        let future = async move {
            if let Some(handle) = rfd::AsyncFileDialog::default()
                .add_filter("Profile", &["dat"])
                .set_title("Pick your game profile")
                .pick_file()
                .await
            {
                let data = handle.read().await;
                let _ = sender.send(data);
            }
        };

        let _ = poll_promise::Promise::spawn_local(future);
    }
}
