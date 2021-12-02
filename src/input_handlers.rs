use inputbot::{KeybdKey::*, MouseButton::*, *};
use std::sync::atomic::{AtomicBool, Ordering};
//use std::{thread::sleep, time::Duration};

pub fn bind_kb_events() {
  // The closures from inputbot are not
  // FnMut so the only way to have some
  // kind of state is to use something
  // that implements "sync". I think.
  let is_locked = AtomicBool::new(false);

  ScrollLockKey.bind(move || {
    if ScrollLockKey.is_toggled() {
      println!("Scrolllock enabled");
      is_locked.store(true, Ordering::SeqCst);
    } else {
      // Always check that something is actually
      // running before trying disable anything.
      if is_locked.load(Ordering::SeqCst) == true {
        println!("We're still locked.");
      } else {
        println!("Scrollock disabled");
      }
    }
  });

  handle_input_events();
}
