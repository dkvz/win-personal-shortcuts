#![windows_subsystem = "windows"]

use nwg::NativeUi;
mod p_shortcuts_tray;
use p_shortcuts_tray::PShortcutsTray;
mod app_config;
use app_config::AppConfig;
mod input_handlers;
mod kb_events;
mod notifications;
use dotenv::dotenv;
use eyre::Result;

fn main() -> Result<()> {
    dotenv().ok();

    nwg::init().expect("Failed to init Native Windows GUI");

    let app_config = AppConfig::from_env().expect("Config error - Should not happen");

    let ui_base = PShortcutsTray::new(app_config);
    let _ui = PShortcutsTray::build_ui(ui_base).expect("Failed to build UI");
    // This will block the main thread:
    nwg::dispatch_thread_events();

    Ok(())
}
