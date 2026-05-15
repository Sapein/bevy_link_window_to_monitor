#![deprecated = "This functionality is now native in Bevy"]
#![doc = include_str!("../README.md")]
use bevy_app::{App, Plugin};

#[deprecated = "Please import `OnMonitor` from Bevy/Bevy Window directly!"]
pub use bevy_window::OnMonitor;

#[deprecated = "Please import `HasWindows` from Bevy/Bevy Window directly!`"]
pub use bevy_window::HasWindows;

#[deprecated = "This functionality is now native in Bevy"]
/// The main plugin that, when added, will automagically update and manage Window positions.
pub struct LinkWindowToMonitorPlugin;

impl Plugin for LinkWindowToMonitorPlugin {
    fn build(&self, app: &mut App) {
        () 
    }
}
