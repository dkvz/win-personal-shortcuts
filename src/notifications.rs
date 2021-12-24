use std::sync::mpsc::{self, SyncSender, Receiver};
use crate::p_shortcuts_tray::APP_TITLE;

pub enum NotificationLevel {
  Info,
  Error
}

pub enum NotificationType {
  MessageBox,
  TrayNotification
}

pub struct Notification {
  pub text: String,
  pub title: Option<String>,
  pub notification_type: NotificationType,
  pub notification_level: NotificationLevel
}

pub struct Notifier {
  tx: SyncSender<Notification>,
  notice_sender: nwg::NoticeSender
}

impl Notifier {
  pub fn new(
    tx: SyncSender<Notification>, 
    notice_sender: nwg::NoticeSender
  ) -> Self {
    Self {
      tx,
      notice_sender
    }
  }

  pub fn info_box(&self, msg: String) {
    let notif = Notification::info_box(msg);
    self.send_notification(notif)
  }

  fn send_notification(&self, notif: Notification) {
    match self.tx.try_send(notif) {
      Err(_) => eprintln!("Could not send notification"),
      _ => self.notice_sender.notice()
    }
  }
}

impl Notification {
  pub fn info_box(msg: String) -> Self {
    Self {
      text: msg,
      title: None,
      notification_level: NotificationLevel::Info,
      notification_type: NotificationType::MessageBox
    }
  }
}

impl Default for Notification {
  fn default() -> Self {
    Self {
      text: String::default(),
      title: Some(String::from(APP_TITLE)),
      notification_type: NotificationType::MessageBox,
      notification_level: NotificationLevel::Info
    }
    
  }
}