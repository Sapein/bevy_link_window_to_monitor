use bevy_app::{App, Plugin, PreUpdate};
use bevy_ecs::component::Component;
use bevy_ecs::entity::Entity;
use bevy_ecs::query::{Changed, Or};
use bevy_ecs::system::{Commands, Query};
use bevy_window::{Monitor, MonitorSelection, PrimaryMonitor, Window, WindowPosition};

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

/// determine_monitor
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
pub fn determine_monitor<'a>(
    window: &Window,
    monitors: Vec<(&'a Monitor, Option<&PrimaryMonitor>, Entity)>,
) -> Option<&'a Monitor> {
    match window.position {
        WindowPosition::Automatic => None, // we can not determine the monitor it's on at all.
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
    windows: Query<(&Window, Option<&OnMonitor>, Entity), Or<(Changed<Monitor>, Changed<Window>)>>,
    monitors: Query<(&Monitor, Option<&PrimaryMonitor>, Entity)>,
) {
    for (window, on_monitor, window_entity) in windows {
        let identified_monitor = match determine_monitor(window, monitors.iter().collect()) {
            Some(monitor) => monitor,
            None => {
                if on_monitor.is_some() {
                    commands.entity(window_entity).remove::<OnMonitor>();
                }
                continue;
            }
        };
        let identified_monitor_entity = match monitors
            .iter()
            .find(|(m, ..)| m.name == identified_monitor.name)
            .map(|(.., e)| e)
        {
            Some(e) => e,
            None => continue, // Maybe error here?
        };

        if let Some(attached_monitor) = on_monitor {
            if identified_monitor_entity == attached_monitor.0 {
                continue;
            }
        }

        commands
            .entity(window_entity)
            .insert(OnMonitor(identified_monitor_entity));
    }
}
