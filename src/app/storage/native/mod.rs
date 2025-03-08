use std::{fs, path::PathBuf, sync::mpsc};

use rfd::AsyncFileDialog;
use tap::pipe::Pipe;

pub struct Storage {
    path: Option<PathBuf>,
    path_sender: mpsc::Sender<PathBuf>,
    path_receiver: mpsc::Receiver<PathBuf>,
}

impl Default for Storage {
    fn default() -> Self {
        let (path_sender, path_receiver) = mpsc::channel();

        Self {
            path: None,
            path_sender,
            path_receiver,
        }
    }
}

impl super::StorageIO for Storage {
    fn drag_handle(&mut self, ctx: &egui::Context) {
        let dragged_path = ctx.input(|i| {
            let dropped_files = &i.raw.hovered_files;

            let file = dropped_files.get(0).map(|df| &df.path).cloned().flatten();
            file
        });

        if let Some(path) = dragged_path {
            let _ = self.path_sender.send(path);
            ctx.input_mut(|i| i.raw.hovered_files.clear());
        }
    }

    fn try_read_data(&mut self) -> Option<Vec<u8>> {
        while let Ok(path) = self.path_receiver.try_recv() {
            self.path = Some(path);
        }

        let path = self.path.take()?;
        let Ok(data) = fs::read(path) else {
            return None;
        };

        Some(data)
    }

    fn try_write_data(&self, data: &[u8]) {
        let data = data.to_vec();
        let future = async move {
            if let Some(handle) = rfd::AsyncFileDialog::default()
                .add_filter("Profile", &["dat"])
                .set_title("Save your game profile")
                .set_file_name("profile.dat")
                .save_file()
                .await
            {
                let _ = handle.write(&data).await;
            }
        };

        crate::TOKIO_HANDLE.get().unwrap().spawn(future);
    }

    fn open_dialog(&self, ctx: &egui::Context) {
        (self.path_sender.clone(), ctx.clone())
            .pipe(|(tx, ctx)| async move {
                let path = AsyncFileDialog::default()
                    .add_filter("Profile", &["dat"])
                    .set_title("Pick your game profile")
                    .pick_file()
                    .await;
                if let Some(path) = path {
                    let path = path.into();
                    let _ = tx.send(path);
                    ctx.request_repaint();
                }
            })
            .pipe(|future| crate::TOKIO_HANDLE.get().unwrap().spawn(future));
    }
}
