use inputbot::{KeybdKey::*, *};
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
//use std::{thread::sleep, time::Duration};
use crate::app_config::AppConfig;
use crate::notifications::Notifier;
use std::process::Command;

pub fn bind_kb_events(app_config: AppConfig, notifier: Notifier) {
  // The closures from inputbot are not
  // FnMut so the only way to have some
  // kind of state is to use something
  // that implements "sync". I think.
  let is_locked = AtomicBool::new(false);
  let obs_pid = AtomicU32::new(0);

  ScrollLockKey.bind(move || {
    if ScrollLockKey.is_toggled() && is_locked.load(Ordering::SeqCst) == false {
      is_locked.store(true, Ordering::SeqCst);
      match Command::new(format!("{}\\{}", &app_config.obs_path, &app_config.obs_exe))
        .current_dir(&app_config.obs_path)
        .arg("--startrecording")
        .arg(format!("--profile {}", app_config.obs_profile))
        .arg("--minimize-to-tray")
        .arg("--disable-updater")
        .spawn()
      {
        Ok(child) => {
          obs_pid.store(child.id(), Ordering::SeqCst);
        }
        Err(e) => {
          is_locked.store(false, Ordering::SeqCst);
          notifier.error_box(format!("Could not start OBS: {}", e));
        }
      }
    } else if is_locked.load(Ordering::SeqCst) == true {
      // Always check that something is actually
      // running before trying disable anything.
      // Kill the task:
      match Command::new("cmd")
        .args(&[
          "/C",
          "taskkill",
          "/F",
          "/PID",
          &obs_pid.load(Ordering::SeqCst).to_string(),
        ])
        .output()
      {
        Ok(output) => {
          if output.status.success() {
            println!("Recording ended.");
            // Re-initalize everything:
            obs_pid.store(0, Ordering::SeqCst);
            is_locked.store(false, Ordering::SeqCst);
          } else {
            let err_output = String::from_utf8(output.stderr)
              .unwrap_or(String::from("Could not parse error message"));
            notifier.error_box(format!("Could not kill OBS: {}", err_output));
          }
        }
        Err(e) => notifier.error_box(format!("Could not spawn taskkill: {}", e))
      }
    }
  });

  handle_input_events();
}
