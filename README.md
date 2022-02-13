# Windows Personal Shortcuts
**WORK IN PROGRESS**

Meant to associate keyboard events to running or stopping things.

The main use case is to replace the Nvidia ShadowPlay start & stop recording with this and OBS Studio and I should probably rename the project to reflect that.

Appears in the system tray when started.

## Releases
I should be providing compiled executables in the [Releases section](https://github.com/dkvz/win-personal-shortcuts/releases), alongside an example .env file, if I don't forget about that.

## Configure
- Copy `.env.example` as `.env` and edit the variables. Backslashes have to be escaped in Windows paths.

## Building
Embedding the resources (well, just the one icon) requires having "windres.exe" which I think is part of the MSVC toolchain.

When using the GNU compiler like I do, you have to download MinGW64 and add the bin folder of it in your PATH.

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

## New plan to make notifications work
Everything has to be inside of the struct that implements NwgUI or it's just too hard.

Fortunately it's possible to call an event on init that could start a thread, which is what I need.

There's an example here which also includes a "self reference" (that I might not need): https://github.com/gabdube/native-windows-gui/blob/master/native-windows-gui/examples/self_referencing.rs

Actually this is much closer to what I want to do: 
https://github.com/gabdube/native-windows-gui/issues/232

## Adding icon to the exe
This only seems to work with the Visual Studio compiler. However I had the manifest set to require admin privileges so that may have been the issue, it's now commented out so we get a regular user level executable.

# TODO
- [ ] Rebrand this project to that it's linked more closely to its purpose of replacing ShadowPlay.
- [ ] Add a config option to disable notifications completely.
- [ ] Add a checkbox in the tray menu to be able to disable or enable notifications.
- [ ] Allow configuring an alternate key to ScrLk, which would probably be a weird key like "-" and check for Ctrl.
- [ ] We could have some option to run a post-treatment (using ffmpeg) on the captured video.
- [ ] Try using ffmpeg instead of OBS Studio, should be smoother.
- [x] Uncomment the windows subsystem thingy in main.rs.
- [x] Load the notification icon from resources inside p_shortcuts_tray since I'm now using resources anyway.