//#![windows_subsystem = "windows"]

use nwg::NativeUi;
use std::thread;
mod p_shortcuts_tray;
use p_shortcuts_tray::PShortcutsTray;
mod input_handlers;
use input_handlers::bind_kb_events;
mod app_config;
use app_config::AppConfig;
use dotenv::dotenv;
use eyre::Result;

fn main() -> Result<()> {
    dotenv().ok();

    nwg::init().expect("Failed to init Native Windows GUI");
    let app_config = AppConfig::from_env().expect("Config error - Should not happen");
    let ui_base = PShortcutsTray::new("Is this working?".to_string());
    let _ui = PShortcutsTray::build_ui(ui_base).expect("Failed to build UI");
    // I don't think we can avoid having another thread for the
    // keyboard events:
    thread::spawn(move || bind_kb_events(app_config));
    // This will block the main thread:
    nwg::dispatch_thread_events();

    Ok(())
}
