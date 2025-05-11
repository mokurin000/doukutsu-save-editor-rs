#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

#[cfg(target_arch = "wasm32")]
fn main() {
    use eframe::wasm_bindgen::JsCast as _;

    // Redirect `log` message to `console.log` and friends:
    eframe::WebLogger::init(log::LevelFilter::Debug).unwrap();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        let document = web_sys::window()
            .expect("No window")
            .document()
            .expect("No document");

        let canvas = document
            .get_element_by_id("the_canvas_id")
            .expect("Failed to find the_canvas_id")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("the_canvas_id was not a HtmlCanvasElement");

        let start_result = eframe::WebRunner::new()
            .start(
                canvas,
                web_options,
                Box::new(|cc| Ok(Box::new(doukutsu_save_editor::MainApp::new(cc)))),
            )
            .await;

        // Remove the loading text and spinner:
        if let Some(loading_text) = document.get_element_by_id("loading_text") {
            match start_result {
                Ok(_) => {
                    loading_text.remove();
                }
                Err(e) => {
                    loading_text.set_inner_html(
                        "<p> The app has crashed. See the developer console for details. </p>",
                    );
                    panic!("Failed to start eframe: {e:?}");
                }
            }
        }
    });
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    use doukutsu_save_editor::TASK_SENDER;
    use egui::Vec2;
    use spdlog::Level;

    // Log to stdout (if you run with `RUST_LOG=debug`).
    spdlog::default_logger().set_level_filter(spdlog::LevelFilter::MoreSevereEqual(Level::Debug));

    let native_options = eframe::NativeOptions {
        persist_window: true,
        viewport: egui::ViewportBuilder::default()
            .with_drag_and_drop(true)
            .with_min_inner_size(Vec2::new(840., 840.)),
        hardware_acceleration: eframe::HardwareAcceleration::Preferred,
        ..Default::default()
    };

    std::thread::spawn(move || {
        let (tx, rx) = kanal::unbounded();
        let rx = rx.to_async();

        _ = TASK_SENDER.set(tx);

        let async_rt = compio::runtime::Runtime::new().unwrap();
        async_rt.block_on(async move {
            while let Ok(fut) = rx.recv().await {
                fut.await;
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
