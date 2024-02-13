#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

fn main() {
    use tokio::runtime::Runtime;
    use tokio::time;

    let native_options = eframe::NativeOptions {
        follow_system_theme: true,
        persist_window: true,
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

    eframe::run_native(
        "CaveStory Save Editor",
        native_options,
        Box::new(|cc| Box::new(doukutsu_save_editor::MainApp::new(cc))),
    )
    .unwrap();
}
