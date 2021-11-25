use inputbot::{KeybdKey::*, MouseButton::*, *};
//use std::{thread::sleep, time::Duration};

pub fn bind_kb_events() {
  ScrollLockKey.bind(|| {
    if ScrollLockKey.is_toggled() {
      println!("Scrolllock enabled");
    } else {
      println!("Scrollock disabled");
    }
  });

  handle_input_events();
}
