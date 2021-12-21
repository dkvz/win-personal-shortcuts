//#![windows_subsystem = "windows"]

use nwg::NativeUi;
mod p_shortcuts_tray;
use p_shortcuts_tray::PShortcutsTray;
mod input_handlers;
mod app_config;
use dotenv::dotenv;
use eyre::Result;

fn main() -> Result<()> {
    dotenv().ok();

    nwg::init().expect("Failed to init Native Windows GUI");
    //let ui_base = PShortcutsTray::new();
    let _ui = PShortcutsTray::build_ui(Default::default())
        .expect("Failed to build UI");
    
    // This will block the main thread:
    nwg::dispatch_thread_events();

    Ok(())
}
