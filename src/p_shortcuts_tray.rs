use std::sync::mpsc::{self, SyncSender, Receiver};
use eyre::{eyre, Result};
use std::thread;
use crate::app_config::AppConfig;
use crate::input_handlers::bind_kb_events;
use std::rc::Rc;
use nwd::NwgUi;
use nwg::NativeUi;

pub const APP_TITLE: &'static str = "Personal Shortcuts";

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

// Every single property has to implement Default as a requirement 
// for using NwgUi the declarative way.
#[derive(Default, NwgUi)]
pub struct PShortcutsTray {
  #[nwg_control]
  #[nwg_events(
    OnInit: [PShortcutsTray::init(SELF)]
  )]
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

  #[nwg_control(parent: tray_menu, text: "Popup")]
  #[nwg_events(OnMenuItemSelected: [PShortcutsTray::hello2])]
  tray_item2: nwg::MenuItem,

  #[nwg_control(parent: tray_menu, text: "Exit")]
  #[nwg_events(OnMenuItemSelected: [PShortcutsTray::exit])]
  tray_item3: nwg::MenuItem,
}

impl PShortcutsTray {

  /*pub fn new() -> Self {
    let (tx, rx) = mpsc::sync_channel::<Notification>(2);
    Self {
      notification_tx: Some(tx),
      notification_rx: Some(rx),
      ..Default::default()
    }
  }*/

  fn init(app: &'static PShortcutsTray) {
    let app_config = AppConfig::from_env()
      .expect("Config error - Should not happen");

    // Open the notification channel.
    // I had to pick a queue size, I think it errors
    // when you try to send to a full queue. It also
    // means the consumer doesn't work.
    let (tx, rx) = mpsc::sync_channel::<Notification>(2);

    // I don't think we can avoid having another thread for the
    // keyboard events.
    // I could clone tx but I only have one.
    //thread::spawn(move || bind_kb_events(app_config, tx));

    nwg::dispatch_thread_events_with_callback(move || loop {
      match rx.recv() {
        Ok(notif) => {
          let _ = app.notify(notif);
        },
        Err(_) => panic!("Notification channel failed")
      }
    });
  }

  fn show_menu(&self) {
    let (x, y) = nwg::GlobalCursor::position();
    self.tray_menu.popup(x, y);
  }

  // The "MessageChoice" should always be OK.
  fn notify(&self, msg: Notification) -> nwg::MessageChoice {
    let title = msg.title.unwrap_or(String::from(APP_TITLE));
    match msg.notification_level {
      NotificationLevel::Info => nwg::simple_message(
        &title,
        &msg.text
      ),
      NotificationLevel::Error => nwg::error_message(
        &title, 
        &msg.text
      )
    }
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