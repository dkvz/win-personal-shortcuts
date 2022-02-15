use crate::app_config::AppConfig;
use crate::notifications::Notifier;
use inputbot::{KeybdKey::*, *};
use std::process::Command;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::Arc;
use std::thread::{self, JoinHandle};

pub struct KbEvents {
  app_config: Arc<AppConfig>,
  notifier: Arc<Notifier>,
  is_locked: Arc<AtomicBool>,
  obs_pid: Arc<AtomicU32>,
  // I can't do anything useful with this I don't know
  // why I'm saving it.
  thread_handle: Option<JoinHandle<()>>,
}

// TODO: Should be moved elsewhere, I need a module to handle all the
// process spawning and killing at some point.
fn kill_pid(pid: u32) -> Result<(), String> {
  match Command::new("taskkill")
    .args(&["/F", "/PID", &pid.to_string()])
    .output()
  {
    Ok(output) => {
      if output.status.success() {
        Ok(())
      } else {
        let err_output =
          String::from_utf8(output.stderr).unwrap_or(String::from("Could not parse error message"));
        Err(format!("Could not kill OBS: {}", err_output))
      }
    }
    Err(e) => Err(format!("Could not spawn taskkill: {}", e)),
  }
}

impl KbEvents {
  pub fn new(app_config: AppConfig, notifier: Notifier) -> Self {
    Self {
      app_config: Arc::new(app_config),
      notifier: Arc::new(notifier),
      is_locked: Arc::new(AtomicBool::new(false)),
      obs_pid: Arc::new(AtomicU32::new(0)),
      thread_handle: None,
    }
  }

  // Trying to make it consume self but I don't know if that'll work.
  pub fn stop(&mut self) {
    self.is_locked.store(true, Ordering::SeqCst);
    let pid = self.obs_pid.load(Ordering::SeqCst);
    if pid > 0 {
      if let Err(msg) = kill_pid(pid) {
        self.notifier.error_box(format!(
          "Possibly failed to kill OBS, check task manager: {}",
          msg
        ));
      }
    }
  }

  pub fn start(mut self) -> Self {
    // I have to clone or Rc EVERYTHING or the move closure will try
    // to borrow "self" and that's another level of crazy.
    let is_locked = self.is_locked.clone();
    let obs_pid = self.obs_pid.clone();
    let notifier = self.notifier.clone();
    let app_config = self.app_config.clone();

    self.thread_handle = Some(thread::spawn(move || {
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
              notifier.error_box(format!(
                "Could not start OBS at path {} - {}",
                &app_config.obs_path, e
              ));
            }
          }
        } else if is_locked.load(Ordering::SeqCst) == true {
          // Always check that something is actually
          // running before trying disable anything.
          // Kill the task:
          let pid = obs_pid.load(Ordering::SeqCst);
          if pid == 0 {
            // Shouldn't happen.
            notifier.error_box(
              "Missing OBS process ID, you may have to kill the process manually.".to_string(),
            );
            return;
          }
          match kill_pid(pid) {
            Ok(_) => {
              notifier.tray_notification(
                Some("Recording successful".to_string()),
                "Recording ended successfully".to_string(),
              );
              // Re-initalize everything:
              obs_pid.store(0, Ordering::SeqCst);
              is_locked.store(false, Ordering::SeqCst);
            }
            Err(msg) => {
              notifier.error_box(msg);
            }
          }
        }
      });

      handle_input_events();
    }));

    return self;
  }
}
