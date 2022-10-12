#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    let options = eframe::NativeOptions {
        default_theme: eframe::Theme::Light,
        ..Default::default()
    };
    eframe::run_native(
        "fileapp",
        options,
        Box::new(|_| Box::new(rust_moc_gui::FileApp::new())),
    );
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
    eframe::start_web(
        "filecanvas", // hardcode it
        web_options,
        Box::new(|_| Box::new(rust_moc_gui::FileApp::new())),
    )
    .expect("failed to start eframe");
}
