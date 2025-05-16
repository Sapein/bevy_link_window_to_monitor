#![doc = include_str!("../README.md")]
use bevy_app::{App, Plugin, PreUpdate};
use bevy_ecs::component::Component;
use bevy_ecs::entity::Entity;
use bevy_ecs::query::{Changed, Or, With};
use bevy_ecs::system::{Commands, NonSend, Query};
use bevy_window::{Monitor, MonitorSelection, PrimaryMonitor, Window, WindowPosition};
use bevy_winit::WinitWindows;

/// The main plugin that, when added, will automagically update and manage Window positions.
pub struct LinkWindowToMonitorPlugin;

impl Plugin for LinkWindowToMonitorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, link_monitor_to_window);
    }
}

#[derive(Component, Debug)]
#[relationship(relationship_target = HasWindows)]
pub struct OnMonitor(pub Entity);

#[derive(Component, Debug)]
#[relationship_target(relationship = OnMonitor, linked_spawn)]
pub struct HasWindows(Vec<Entity>);

/// Attempt to use heuristics to determine the which monitor the window is likely on. This may,
/// however, fail under certain circumstances.
///
/// # Arguments
///
/// * `window`: The Window that you're trying to find which monitor it's on.
/// * `monitors`: A vector of all known monitors, with some additional data to make it easier to find.
///
/// returns: Option<&Monitor>
///
/// # Examples
///
/// ```
/// use bevy_ecs::entity::Entity;
/// use bevy_ecs::query::With;
/// use bevy_ecs::system::{Query, Single};
/// use bevy_window::{Monitor, PrimaryMonitor, PrimaryWindow, Window};
///
/// use bevy_link_window_to_monitor::determine_monitor;
///
/// fn which_monitor(window: Single<&Window, With<PrimaryWindow>>, monitors: Query<(&Monitor, Option<&PrimaryMonitor>, Entity)>) {
///     if let Some(monitor) = determine_monitor(*window, monitors.iter().collect()) {
///         println!("On Monitor: {}", monitor.name);
///     } else {
///         println!("No monitor found!");
///     }
/// }
/// ```
#[deprecated(
    since = "0.2.0",
    note = "Use the System Parameter NonSend<WinitWindows> to use the winit window to get the current monitor instead."
)]
pub fn determine_monitor<'a>(
    window: &Window,
    monitors: Vec<(&'a Monitor, Option<&PrimaryMonitor>, Entity)>,
) -> Option<&'a Monitor> {
    match window.position {
        WindowPosition::Automatic => None,
        WindowPosition::At(window_pos) => {
            let mut zero_monitor = None;
            for (monitor, ..) in monitors {
                let extents = [
                    monitor.physical_position,
                    monitor.physical_position + monitor.physical_size().as_ivec2(),
                ];

                if monitor.physical_position.x == 0 && monitor.physical_position.y == 0 {
                    zero_monitor = Some(monitor);
                }

                if window_pos.x >= extents[0].x
                    && window_pos.x <= extents[1].x
                    && window_pos.y >= extents[0].y
                    && window_pos.y <= extents[1].y
                {
                    return Some(monitor);
                }
            }
            if window_pos.x < 0 || window_pos.y < 0 {
                zero_monitor
            } else {
                None
            }
        }
        WindowPosition::Centered(selection) => match selection {
            MonitorSelection::Current => None, // We can not actually determine position.
            MonitorSelection::Index(_) => None,
            MonitorSelection::Primary => monitors
                .into_iter()
                .find(|(_, p, ..)| p.is_some())
                .map(|(m, ..)| m),
            MonitorSelection::Entity(entity) => monitors
                .into_iter()
                .find(|(.., e)| *e == entity)
                .map(|(m, ..)| m),
        },
    }
}

fn link_monitor_to_window(
    mut commands: Commands,
    windows: Query<
        (Entity, Option<&OnMonitor>),
        (With<Window>, Or<(Changed<Monitor>, Changed<Window>)>),
    >,
    monitors: Query<(&Monitor, Entity)>,
    underlying: NonSend<WinitWindows>,
) {
    'window: for (entity, on_monitor) in windows {
        let winit_monitor = match underlying
            .entity_to_winit
            .get(&entity)
            .and_then(|i| underlying.windows.get(i))
            .and_then(|w| w.current_monitor())
        {
            None => {
                commands.entity(entity).remove::<OnMonitor>();
                continue;
            }
            Some(monitor) => monitor,
        };

        for (monitor, monitor_entity) in monitors {
            if monitor.name != winit_monitor.name()
                || monitor.physical_position.x != winit_monitor.position().x
                || monitor.physical_position.y != winit_monitor.position().y
            {
                continue;
            }

            if on_monitor.is_none_or(|m| m.0 != monitor_entity) {
                commands.entity(entity).insert(OnMonitor(monitor_entity));
            }
            continue 'window;
        }
        commands.entity(entity).remove::<OnMonitor>();
    }
}
