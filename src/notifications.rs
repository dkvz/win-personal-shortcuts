use crate::p_shortcuts_tray::APP_TITLE;
use std::sync::mpsc::{self, Receiver, SyncSender};

pub enum NotificationLevel {
  Info,
  Error,
}

pub enum NotificationType {
  MessageBox,
  TrayNotification,
}

pub struct Notification {
  pub text: String,
  pub title: Option<String>,
  pub notification_type: NotificationType,
  pub notification_level: NotificationLevel,
}

pub struct Notifier {
  tx: SyncSender<Notification>,
  notice_sender: nwg::NoticeSender,
}

impl Notifier {
  pub fn new(tx: SyncSender<Notification>, notice_sender: nwg::NoticeSender) -> Self {
    Self { tx, notice_sender }
  }

  pub fn info_box(&self, msg: String) {
    let notif = Notification::info_box(msg);
    self.send_notification(notif)
  }

  pub fn error_box(&self, msg: String) {
    let notif = Notification::error_box(msg);
    self.send_notification(notif)
  }

  pub fn tray_notification(&self, title: Option<String>, msg: String) {
    let notif = Notification::tray_notification(title, msg);
    self.send_notification(notif)
  }

  fn send_notification(&self, notif: Notification) {
    match self.tx.try_send(notif) {
      Err(_) => eprintln!("Could not send notification"),
      _ => self.notice_sender.notice(),
    }
  }
}

impl Notification {
  pub fn info_box(msg: String) -> Self {
    Self {
      text: msg,
      title: None,
      notification_level: NotificationLevel::Info,
      notification_type: NotificationType::MessageBox,
    }
  }

  pub fn error_box(msg: String) -> Self {
    Self {
      text: msg,
      title: None,
      notification_level: NotificationLevel::Error,
      notification_type: NotificationType::MessageBox,
    }
  }

  pub fn tray_notification(title: Option<String>, msg: String) -> Self {
    Self {
      text: msg,
      title,
      notification_level: NotificationLevel::Info,
      notification_type: NotificationType::TrayNotification,
    }
  }
}

impl Default for Notification {
  fn default() -> Self {
    Self {
      text: String::default(),
      title: Some(String::from(APP_TITLE)),
      notification_type: NotificationType::MessageBox,
      notification_level: NotificationLevel::Info,
    }
  }
}
