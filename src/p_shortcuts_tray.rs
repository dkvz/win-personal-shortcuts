use std::sync::mpsc::{self, Receiver};
use eyre::{eyre, Result};
use std::thread;
use std::cell::RefCell;
use crate::app_config::AppConfig;
use crate::input_handlers::bind_kb_events;
use crate::notifications::*;
use nwd::NwgUi;
use nwg::NativeUi;

pub const APP_TITLE: &'static str = "Personal Shortcuts";

// Loading the icon as binary data to make sure it gets included
// in the executable:
// I switched to using it from the embedded resources since it's
// being loaded there anyway.
//const MAIN_ICON: &[u8] = include_bytes!("../resources/shrimp.ico");

// Every single property has to implement Default as a requirement 
// for using NwgUi the declarative way.
#[derive(Default, NwgUi)]
pub struct PShortcutsTray {
  #[nwg_control]
  #[nwg_events(OnInit: [PShortcutsTray::init])]
  window: nwg::MessageWindow,

  //#[nwg_resource(source_file: Some("./resources/shrimp.ico"))]
  //#[nwg_resource(source_bin: Some(MAIN_ICON))]
  // In the end, loading from resources:
  #[nwg_resource]
  embed: nwg::EmbedResource,
  
  #[nwg_resource(source_embed: Some(&data.embed), source_embed_str: Some("MAINICON"))]
  icon: nwg::Icon,

  #[nwg_control(icon: Some(&data.icon), tip: Some(APP_TITLE))]
  #[nwg_events(
      MousePressLeftUp: [PShortcutsTray::show_menu], 
      OnContextMenu: [PShortcutsTray::show_menu]
  )]
  tray: nwg::TrayNotification,

  #[nwg_control(parent: window, popup: true)]
  tray_menu: nwg::Menu,

  /*#[nwg_control(parent: tray_menu, text: "Popup")]
  #[nwg_events(OnMenuItemSelected: [PShortcutsTray::hello2])]
  tray_item2: nwg::MenuItem,*/

  #[nwg_control(parent: tray_menu, text: "Exit")]
  #[nwg_events(OnMenuItemSelected: [PShortcutsTray::exit])]
  tray_item3: nwg::MenuItem,

  #[nwg_control]
  #[nwg_events(OnNotice: [PShortcutsTray::notify_event])]
  notify_event: nwg::Notice,

  notification_rx: RefCell<Option<Receiver<Notification>>>
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

  fn init(&self) {
    let app_config = AppConfig::from_env()
      .expect("Config error - Should not happen");

    // Open the notification channel. We need it to implement Sync 
    // because of how the input handler works.
    // Had to pick an arbitrary queue size. I assume even 1 should
    // work with no error.
    let (tx, rx) = mpsc::sync_channel::<Notification>(2);
    // I could clone tx but I only have one.
    let notifier = Notifier::new(tx, self.notify_event.sender());
    
    // I don't think we can avoid having another thread for the
    // keyboard events.
    thread::spawn(move || bind_kb_events(app_config, notifier));

    *self.notification_rx.borrow_mut() = Some(rx);
  }

  fn show_menu(&self) {
    let (x, y) = nwg::GlobalCursor::position();
    self.tray_menu.popup(x, y);
  }

  fn notify_event(&self) {
    let mut rx_ref = self.notification_rx.borrow_mut();
    let rx = rx_ref.as_mut().expect("Notification channel is down");
    while let Ok(msg) = rx.try_recv() {
      self.notify(msg);
    }
  }

  // The "MessageChoice" should always be OK.
  fn notify(&self, msg: Notification) {
    match msg.notification_type {
      NotificationType::MessageBox => {
        let title = msg.title.unwrap_or(String::from(APP_TITLE));
        // We don't care about the response to the dialog:
        let _ = match msg.notification_level {
          NotificationLevel::Info => nwg::simple_message(
            &title,
            &msg.text
          ),
          NotificationLevel::Error => nwg::error_message(
            &title, 
            &msg.text
          )
        };
      },
      NotificationType::TrayNotification => {
        let title = match &msg.title {
          Some(t) => Some(t.as_ref()),
          _ => None
        };
        let flags = nwg::TrayNotificationFlags::USER_ICON | nwg::TrayNotificationFlags::LARGE_ICON;
        self.tray.show(
          &msg.text,
          title,
          Some(flags),
          Some(&self.icon),
        );
      }
    }
    
  }

  fn exit(&self) {
    nwg::stop_thread_dispatch();
  }
}