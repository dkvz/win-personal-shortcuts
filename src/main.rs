#![windows_subsystem = "windows"]

use nwg::NativeUi;
mod p_shortcuts_tray;
use p_shortcuts_tray::PShortcutsTray;

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    let _ui = PShortcutsTray::build_ui(Default::default()).expect("Failed to build UI");
    nwg::dispatch_thread_events();
}
