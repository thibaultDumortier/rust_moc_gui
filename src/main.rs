#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

// When compiling natively.
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    tracing_subscriber::fmt::init();

    let options = eframe::NativeOptions {
        default_theme: eframe::Theme::Light,
        ..Default::default()
    };
    let _ = eframe::run_native(
        "fileapp",
        options,
        Box::new(|_| Box::new(rust_moc_gui::FileApp::default())),
    ); // This is supposed to work, if it does not, then there is a greater error, please open an issue.
}

// when compiling to web using trunk.
#[cfg(target_arch = "wasm32")]
fn main() {
    // Make sure panics are logged using `console.error`.
    console_error_panic_hook::set_once();

    // Redirect tracing to console.log and friends:
    tracing_wasm::set_as_global_default();

    let web_options = eframe::WebOptions {
        follow_system_theme: false,
        default_theme: eframe::Theme::Light,
        ..Default::default()
    };

    wasm_bindgen_futures::spawn_local(async {
        eframe::start_web(
            "filecanvas", // hardcode it
            web_options,
            Box::new(|_| Box::new(rust_moc_gui::FileApp::default())),
        )
        .await
        .expect("failed to start eframe");
    });
}
