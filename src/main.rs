//#![windows_subsystem = "windows"]

use nwg::NativeUi;
use std::thread;
mod p_shortcuts_tray;
use p_shortcuts_tray::PShortcutsTray;
mod input_handlers;
use input_handlers::bind_kb_events;
mod app_config;
use dotenv::dotenv;

fn main() {
    dotenv().ok();
    nwg::init().expect("Failed to init Native Windows GUI");
    let _ui = PShortcutsTray::build_ui(Default::default()).expect("Failed to build UI");
    // I don't think we can avoid having another thread for the
    // keyboard events:
    thread::spawn(bind_kb_events);
    // This will block the main thread:
    nwg::dispatch_thread_events();
}
