use nwd::NwgUi;
use nwg::NativeUi;

pub const APP_TITLE: &'static str = "Personal Shortcuts";

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
}

impl PShortcutsTray {
  fn show_menu(&self) {
      let (x, y) = nwg::GlobalCursor::position();
      self.tray_menu.popup(x, y);
  }

  fn hello1(&self) {
      nwg::simple_message("Hello", "Hello World!");
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