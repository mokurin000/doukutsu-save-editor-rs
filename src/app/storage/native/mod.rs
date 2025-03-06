use std::{fs, path::PathBuf, sync::mpsc};

use rfd::{AsyncFileDialog, AsyncMessageDialog, MessageLevel};
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

        let Some(Ok(data)) = self.path.as_ref().map(|p| fs::read(p)) else {
            return None;
        };

        Some(data)
    }

    fn try_write_data(&self, data: &[u8]) {
        let Some(path) = &self.path else {
            return;
        };
        if let Err(e) = fs::write(path, data) {
            tokio::task::spawn(async move {
                AsyncMessageDialog::new()
                    .set_level(MessageLevel::Error)
                    .set_description(&e.to_string())
                    .set_title("Error occured on saving!")
                    .show()
                    .await;
            });
        }
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
            .pipe(tokio::task::spawn);
    }
}
