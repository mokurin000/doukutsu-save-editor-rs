#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use egui::Vec2;

fn main() {
    use tokio::runtime::Runtime;
    use tokio::time;

    let native_options = eframe::NativeOptions {
        persist_window: true,
        viewport: egui::ViewportBuilder::default()
            .with_drag_and_drop(true)
            .with_min_inner_size(Vec2::new(840., 840.)),
        hardware_acceleration: eframe::HardwareAcceleration::Preferred,
        ..Default::default()
    };

    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    let async_rt = Runtime::new().unwrap();
    let _guard = async_rt.enter();

    std::thread::spawn(move || {
        async_rt.block_on(async move {
            loop {
                time::sleep(time::Duration::from_secs(114514)).await;
            }
        });
    });

    let app_name = "CaveStory Save Editor";
    eframe::run_native(
        app_name,
        native_options,
        Box::new(|cc| Ok(Box::new(doukutsu_save_editor::MainApp::new(cc)))),
    )
    .unwrap();
}
