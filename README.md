# bevy_link_window_to_monitor  
 `bevy_link_window_to_monitor` is a bevy micro-crate designed to provide a small API around linking windows with monitors,
since that API is currently missing within Bevy itself.

To use this crate you just need to insert the `LinkWindowsToMonitorPlugin`, after which the plugin will attempt to determine
and add a relationship onto the Window specifying which Monitor the Window is on.

You may also use `determine_monitor()` which will return the Monitor we determine the window to be on, if we can determine it,
if you wish to handle the linking yourself.

## Known Edge Cases
Please note there are a handful of situations where this Plugin can not determine the Monitor the Window is on:
- If you set `window.position` to `WindowPosition::Automatic` then detecting the Monitor may take up to one frame.
- If you set `window.position` to `WindowPosition::Centered` with `MonitorSelection::Curent` or `MonitorSelection::Index`
  then monitor detection may take up to one frame.

Detection can completely fail if you continuously set `window.position` or do it immediately before the linking system
runs or immediately after the `changed_windows` system provided by Bevy.