//#![windows_subsystem = "windows"]

use nwg::NativeUi;
use std::thread;
mod p_shortcuts_tray;
use p_shortcuts_tray::PShortcutsTray;
mod input_handlers;
use input_handlers::bind_kb_events;

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    let _ui = PShortcutsTray::build_ui(Default::default()).expect("Failed to build UI");
    thread::spawn(bind_kb_events);
    nwg::dispatch_thread_events();
}
