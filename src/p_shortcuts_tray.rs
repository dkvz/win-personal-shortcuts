use std::sync::mpsc::{self, SyncSender, Receiver};
use eyre::{eyre, Result};
use std::sync::atomic::{AtomicBool, Ordering};
use nwd::NwgUi;
use nwg::NativeUi;

pub const APP_TITLE: &'static str = "Personal Shortcuts";

pub enum NotificationLevel {
  Info,
  Warning,
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

// Every single property has to implement Default as a requirement 
// for using NwgUi the declarative way.
#[derive(Default, NwgUi)]
pub struct PShortcutsTray {
  #[nwg_control]
  window: nwg::MessageWindow,

  #[nwg_resource(source_file: Some("./resources/shrimp.ico"))]
  icon: nwg::Icon,

  #[nwg_control(icon: Some(&data.icon), tip: Some(APP_TITLE))]
  #[nwg_events(
      MousePressLeftUp: [PShortcutsTray::show_menu], 
      OnContextMenu: [PShortcutsTray::show_menu]
  )]
  tray: nwg::TrayNotification,

  #[nwg_control(parent: window, popup: true)]
  tray_menu: nwg::Menu,

  #[nwg_control(parent: tray_menu, text: "Hello")]
  #[nwg_events(OnMenuItemSelected: [PShortcutsTray::hello1])]
  tray_item1: nwg::MenuItem,

  #[nwg_control(parent: tray_menu, text: "Popup")]
  #[nwg_events(OnMenuItemSelected: [PShortcutsTray::hello2])]
  tray_item2: nwg::MenuItem,

  #[nwg_control(parent: tray_menu, text: "Exit")]
  #[nwg_events(OnMenuItemSelected: [PShortcutsTray::exit])]
  tray_item3: nwg::MenuItem,

  notification_tx: Option<SyncSender<Notification>>,
  notification_rx: Option<Receiver<Notification>>,
  // AtomicBool default value is false.
  channel_open: AtomicBool
}

impl PShortcutsTray {

  pub fn new() -> Self {
    // I had to pick a queue size, I think it errors
    // when you try to send to a full queue. It also
    // means the consumer doesn't work.
    let (tx, rx) = mpsc::sync_channel::<Notification>(2);
    Self {
      notification_tx: Some(tx),
      notification_rx: Some(rx),
      ..Default::default()
    }
  }

  // Returns a clone of the notification channel, if possible.
  pub fn notification_tx(&self) -> Result<SyncSender<Notification>> {
    if self.channel_open.load(Ordering::SeqCst) == false && self.notification_tx.is_some() {
      // Open the channel...
      
    }
    match &self.notification_tx {
      Some(tx) => Ok(tx.clone()),
      None => Err(eyre!("Struct has not been initialized with ::new()"))
    }
  }

  fn show_menu(&self) {
    let (x, y) = nwg::GlobalCursor::position();
    self.tray_menu.popup(x, y);
  }

  fn hello1(&self) {
    nwg::simple_message("Hello", "Simple messagebox");
  }

  fn hello2(&self) {
    let flags = nwg::TrayNotificationFlags::USER_ICON | nwg::TrayNotificationFlags::LARGE_ICON;
    self.tray.show(
      "Hello World",
      Some("Welcome to my application"),
      Some(flags),
      Some(&self.icon),
    );
  }

  fn exit(&self) {
    nwg::stop_thread_dispatch();
  }
}