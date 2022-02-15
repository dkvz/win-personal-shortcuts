use crate::app_config::AppConfig;
use crate::notifications::Notifier;
use inputbot::{KeybdKey::*, *};
use std::process::Command;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::Arc;
use std::thread::{self, JoinHandle};

pub struct KbEvents {
  app_config: AppConfig,
  notifier: Arc<Notifier>,
  is_locked: Arc<AtomicBool>,
  obs_pid: Arc<AtomicU32>,
  thread_handle: Option<JoinHandle<()>>,
}

impl KbEvents {
  pub fn new(app_config: AppConfig, notifier: Notifier) -> Self {
    Self {
      app_config,
      notifier: Arc::new(notifier),
      is_locked: Arc::new(AtomicBool::new(false)),
      obs_pid: Arc::new(AtomicU32::new(0)),
      thread_handle: None,
    }
  }

  pub fn start(&'static mut self) {
    // Had to use static lifetime so that I don't have to also
    // clone the config. Maybe I should've cloned the config
    // one more time. I don't know to Rust.
    let is_locked = self.is_locked.clone();
    let obs_pid = self.obs_pid.clone();
    let notifier = self.notifier.clone();

    self.thread_handle = Some(thread::spawn(move || {
      if ScrollLockKey.is_toggled() && is_locked.load(Ordering::SeqCst) == false {
        is_locked.store(true, Ordering::SeqCst);
        match Command::new(format!(
          "{}\\{}",
          &self.app_config.obs_path, &self.app_config.obs_exe
        ))
        .current_dir(&self.app_config.obs_path)
        .arg("--startrecording")
        .arg(format!("--profile {}", &self.app_config.obs_profile))
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
              &self.app_config.obs_path, e
            ));
          }
        }
      } else if self.is_locked.load(Ordering::SeqCst) == true {
        // Always check that something is actually
        // running before trying disable anything.
        // Kill the task:
        match Command::new("cmd")
          .args(&[
            "/C",
            "taskkill",
            "/F",
            "/PID",
            &self.obs_pid.load(Ordering::SeqCst).to_string(),
          ])
          .output()
        {
          Ok(output) => {
            if output.status.success() {
              self.notifier.tray_notification(
                Some("Recording successful".to_string()),
                "Recording ended successfully".to_string(),
              );
              // Re-initalize everything:
              obs_pid.store(0, Ordering::SeqCst);
              is_locked.store(false, Ordering::SeqCst);
            } else {
              let err_output = String::from_utf8(output.stderr)
                .unwrap_or(String::from("Could not parse error message"));
              self
                .notifier
                .error_box(format!("Could not kill OBS: {}", err_output));
            }
          }
          Err(e) => self
            .notifier
            .error_box(format!("Could not spawn taskkill: {}", e)),
        }
      }

      handle_input_events();
    }));
  }
}
