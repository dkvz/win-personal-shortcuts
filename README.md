# Windows Personal Shortcuts
**WORK IN PROGRESS**

Meant to associate keyboard events to running or stopping things.

The main use case is to replace the Nvidia ShadowPlay start & stop recording with this and OBS Studio.

Appears in the system tray when started.

## Keyboard lib
- Looks promising: https://github.com/obv-mikhail/InputBot

But I'm afraid it'll get picked up as possible botting program, because it can be used to actually create a bot.

- https://gabdube.github.io/native-windows-gui/native-windows-docs/events.html
There may be a way to capture key presses with this.

- https://github.com/linde12/winapi-testing
Example using winapi.

## Other libs
- Easier messageboxes: https://github.com/bekker/msgbox-rs
- Actually I'm just gonna run from system tray: https://github.com/gabdube/native-windows-gui/blob/master/native-windows-gui/examples/system_tray_d.rs