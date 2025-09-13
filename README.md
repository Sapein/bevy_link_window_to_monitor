# bevy_link_window_to_monitor  
 `bevy_link_window_to_monitor` is a bevy micro-crate designed to provide a small API around linking windows with monitors,
since that API is currently missing within Bevy itself.

To use this crate you just need to insert the `LinkWindowToMonitorPlugin`, after which the plugin will attempt to determine
and add a relationship onto the Window specifying which Monitor the Window is on. After which you can use the `OnMonitor`
and `HasWindows` Relationship Components to determine which window it's on.

You may also use `determine_monitor()` which will return the Monitor we determine the window to be on, if we can determine it,
if you wish to handle the linking yourself.

## Bevy Version Compatibility
| Link Window to Monitor | Bevy Version |
|:----------------------:|:------------:|
|          0.1           |     0.16     |
|          0.2           |     0.16     |
|        0.3-rc.1        |  0.17-rc.1   |

## Example
Here's a brief example of a very basic Bevy App that, alongside this crate, will create a window and print out the monitor
it is on.
```rust
use bevy::prelude::*;
use bevy::window::{Monitor, PrimaryWindow};
use bevy_link_window_to_monitor::LinkWindowToMonitorPlugin;

fn main() {
  App::new()
          .add_plugins((DefaultPlugins, LinkWindowToMonitorPlugin))
          .add_systems(Update, print_monitor)
          .run();
}

fn print_monitor(primary_window: Single<&OnMontior, With<PrimaryWindow>>, monitors: Query<(Entity, &Monitor)>) {
  println!("{:?}", monitors.iter().find(|(e, ..)| *e == primary_window.0).unwrap().1.name)
}
```

## Known Edge Cases
If you want to use `determine_monitor()`, you may encounter some edge-cases where the function is unable to determine
the Monitor the Window is on, or otherwise experience a delay, such as:
- If you set `window.position` to `WindowPosition::Automatic` then detecting the Monitor may take up to one frame.
- If you set `window.position` to `WindowPosition::Centered` with `MonitorSelection::Curent` or `MonitorSelection::Index`
  then monitor detection may take up to one frame.
- Detection can completely fail if you continuously set `window.position` or do it immediately before the linking system
  runs or immediately after the `changed_windows` system provided by Bevy.

The plugin manages to get access to the underlying Winit Windows and is able to get the monitor the window is on that way.