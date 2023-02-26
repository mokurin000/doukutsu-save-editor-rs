#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    use tokio::runtime::Runtime;
    use tokio::time;

    use eframe::NativeOptions;

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
        NativeOptions::default(),
        Box::new(|cc| Box::new(doukutsu_save_editor::MainApp::new(cc))),
    )
    .unwrap();
}

// when compiling to web using trunk.
#[cfg(target_arch = "wasm32")]
// #[tokio::main]
fn main() {
    // Make sure panics are logged using `console.error`.
    console_error_panic_hook::set_once();

    // Redirect tracing to console.log and friends:
    tracing_wasm::set_as_global_default();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::start_web(
            "the_canvas_id", // hardcode it
            web_options,
            Box::new(|cc| Box::new(doukutsu_save_editor::MainApp::new(cc))),
        )
        .await
        .expect("failed to start eframe");
    });
}
